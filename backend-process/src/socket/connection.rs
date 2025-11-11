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
    /// - Return `Ok` with the successfully parsed command
    /// - An Error if there is no message or the received command could not be parsed
    pub fn handle_next_message(&mut self) -> Result<IncomingCommands, Error> {
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
                        .map_err(|e| {
                            Error::new(
                                ErrorKind::Other,
                                format!("Database operation failed: {}", e),
                            )
                        })?;

                    return Ok(IncomingCommands::SetKeyConfig(mappings));
                }
                Command::SetBootLogoCommand(command) => {
                    return Ok(IncomingCommands::SetBootLogo(command.image_path));
                }
                Command::SetBrightnessCommand(command) => {
                    return Ok(IncomingCommands::SetBrightness(
                        command.brightness_value as u8,
                    ));
                }
                Command::SetDisplayZoneImageCommand(command) => {
                    if let Ok(display_zone_image_model) = command.try_into() {
                        let database_copy: ImageMapping = display_zone_image_model;
                        self.operations
                            .set_image_for_display_zone(database_copy.clone())
                            .map_err(|e| Error::new(ErrorKind::Other, e))?;

                        return Ok(IncomingCommands::SetDisplayZoneImage(database_copy));
                    }
                }
                Command::ClearAllDisplayZoneImagesCommand(_) => {
                    return Ok(IncomingCommands::ClearAllDisplayZoneImages);
                }
                Command::ClearDisplayZoneImageCommand(command) => {
                    if let Ok(protobuf_enum) = command.display_zone.enum_value()
                        && let Ok(display_zone_wrapper) =
                            DisplayZoneWrapper::try_from(protobuf_enum)
                    {
                        return Ok(IncomingCommands::ClearDisplayZoneImage(
                            display_zone_wrapper.display_zone(),
                        ));
                    }
                }
                _ => {}
            },
            None => {
                return Err(Error::new(ErrorKind::Other, "no command found"));
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

    pub fn prune_connections(&mut self) -> Result<(), Error> {
        Ok(self.server.cleanup_disconnected())
    }

    pub fn server(&self) -> &socket::Server {
        &self.server
    }
}
