use crate::database::operations::Operations;
use crate::input_handler::{InputMapping};
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
                Command::ClearDisplayZoneImageCommand(command) => {}
                Command::KeyConfigCommand(command) => {
                    let fuck: crate::database::models::InputMapping =
                        command.clone().try_into().unwrap();
                    let mappings: InputMapping = fuck.into();

                    if let Ok(key_config_model) = command.try_into() {
                        self.operations
                            .set_mapping_for_input(key_config_model)
                            .map_err(|e| Error::new(ErrorKind::Other, e))?;
                    }
                    return Ok(IncomingCommands::SetKeyConfig(mappings));
                }
                Command::SetBootLogoCommand(command) => {}
                Command::SetBrightnessCommand(command) => {}
                Command::SetDisplayZoneImageCommand(command) => {
                    if let Ok(display_zone_image_model) = command.try_into() {
                        self.operations
                            .set_image_for_display_zone(display_zone_image_model)
                            .map_err(|e| Error::new(ErrorKind::Other, e))?;
                    }
                }
                Command::ClearAllDisplayZoneImagesCommand(command) => {}
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

    pub fn server(&self) -> &socket::Server {
        &self.server
    }
}
