use crate::database::operations::Operations;
use crate::socket::commands::IncomingCommands;
use messaging::protos;
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
        // Will propagate error if there is no message.
        let message = self.server.read_message()?;

        if let Ok(key_config) = protos::key_config::KeyConfig::parse_from_bytes(message.as_slice())
            && let Ok(mapping) = key_config.try_into()
        {
            self.operations.set_mapping_for_input(mapping).ok();
            return Ok(IncomingCommands::SetKeyConfig);
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
