use crate::database::models::ImageMapping;
use crate::database::operations::Operations;
use crate::input_handler::InputMapping;
use crate::protobuf_conversion::DisplayZoneWrapper;
use crate::socket::commands::IncomingCommands;
use messaging::protos::top_level::TopLevel;
use messaging::protos::top_level::top_level::Command;
use messaging::socket;
use messaging::socket::MessageReceiver;
use protobuf::Message;
use std::io::{Error, ErrorKind};

/// Responsible for handling the database writes and reading from sockets
///
/// Should _not_ be concerned with device operations.
pub struct ServerHandler<'a> {
    server: socket::Server,
    operations: &'a Operations,
}

impl<'a> ServerHandler<'a> {
    pub fn new(operations: &'a Operations) -> Result<Self, Error> {
        Ok(Self {
            server: socket::Server::new()?,
            operations,
        })
    }

    /// Checks if there is a message from the connected clients.
    ///
    /// It will either:
    /// - Return `Ok` with the successfully parsed command type and its data (see `IncomingCommands`)
    ///   - The database is written to if the message contains data that should be persisted (i.e. key mappings)
    /// - An Error if there is no message or the received command could not be parsed
    pub fn handle_command_and_persist_config(&mut self) -> Result<IncomingCommands, Error> {
        let message = self.server.read_message()?;

        let top_level = TopLevel::parse_from_bytes(message.as_slice())?;

        match top_level.command {
            Some(command) => match command {
                Command::KeyConfigCommand(command) => {
                    let key_config_model: crate::database::models::InputMapping =
                        command.clone().try_into().map_err(|_| {
                            Error::new(
                                ErrorKind::InvalidData,
                                "Failed to convert command".to_string(),
                            )
                        })?;

                    let mappings: InputMapping = key_config_model.clone().into();

                    self.operations
                        .set_mapping_for_input(key_config_model)
                        .map_err(|e| Error::other(format!("Database operation failed: {}", e)))?;

                    return Ok(IncomingCommands::SetKeyConfig(mappings));
                }
                Command::SetBootLogoCommand(command) => {
                    return Ok(IncomingCommands::SetBootLogo(command.image_path));
                }
                Command::SetBrightnessCommand(command) => {
                    return match command.brightness_value {
                        0..=100 => {
                            self.operations
                                .set_brightness(command.brightness_value as u8)
                                .ok();
                            Ok(IncomingCommands::SetBrightness(
                                command.brightness_value as u8,
                            ))
                        }

                        _ => Err(Error::new(
                            ErrorKind::InvalidInput,
                            "Brightness value was not in the range 0 to 100!",
                        )),
                    };
                }
                Command::SetDisplayZoneImageCommand(command) => {
                    if let Ok(display_zone_image_model) = command.try_into() {
                        let database_copy: ImageMapping = display_zone_image_model;
                        self.operations
                            .set_image_for_display_zone(database_copy.clone())
                            .map_err(Error::other)?;

                        return Ok(IncomingCommands::SetDisplayZoneImage(database_copy));
                    }
                }
                Command::ClearAllDisplayZoneImagesCommand(command) => {
                    if command.unpersist_images {
                        self.operations.clear_all_display_zone_images().ok();
                    }
                    return Ok(IncomingCommands::ClearAllDisplayZoneImages);
                }
                Command::ClearDisplayZoneImageCommand(command) => {
                    if let Ok(protobuf_enum) = command.display_zone.enum_value()
                        && let Ok(display_zone_wrapper) =
                            DisplayZoneWrapper::try_from(protobuf_enum)
                    {
                        self.operations
                            .clear_image_for_display_zone(display_zone_wrapper.display_zone())
                            .ok();
                        return Ok(IncomingCommands::ClearDisplayZoneImage(
                            display_zone_wrapper.display_zone(),
                        ));
                    }
                }
                _ => {}
            },
            None => {
                return Err(Error::other("no command found"));
            }
        }

        Err(Error::new(
            ErrorKind::InvalidData,
            "Unsupported command type",
        ))
    }

    pub fn add_new_connection_if_exists(&mut self) -> Result<(), Error> {
        self.server.accept_connection_async()
    }

    /// Removes connections that are no longer valid so that the server
    /// can scan for new ones using `add_new_connection_if_exists`
    pub fn prune_connections(&mut self) -> Result<(), Error> {
        self.server.cleanup_disconnected();
        Ok(())
    }

    pub fn server(&self) -> &socket::Server {
        &self.server
    }
}
