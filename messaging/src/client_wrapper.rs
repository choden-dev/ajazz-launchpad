use crate::protos::inputs::InputId;
use crate::protos::key_config::{Action, KeyConfig};
use crate::socket;
use crate::socket::MessageSender;
use protobuf::{EnumOrUnknown, Message};
use std::io::Error;

pub struct ClientWrapper {
    client: socket::Client,
}

/// This trait defines all known operations.
pub trait ClientCommands {
    /// Sets the corresponding action for an input (button, knob, touchscreen) from the launchpad
    ///
    /// * `input_id` - the type of action to associate the actions with
    /// * `actions` - the sequence of actions that should occur when input happens
    fn send_key_config(&mut self, input_id: InputId, actions: Vec<Action>) -> Result<(), Error>;
}

/// To be used by any client that wants to communicate with the server
impl ClientWrapper {
    pub fn new(client: socket::Client) -> Self {
        Self { client }
    }
}

impl ClientCommands for ClientWrapper {
    fn send_key_config(&mut self, input_id: InputId, actions: Vec<Action>) -> Result<(), Error> {
        let protobuf = KeyConfig {
            input_id: EnumOrUnknown::new(input_id),
            actions,
            ..KeyConfig::default()
        };
        self.client
            .send_message(protobuf.write_to_bytes()?.as_slice())
    }
}
