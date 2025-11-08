use crate::protos::inputs::InputId;
use crate::protos::key_config::{Action, KeyConfig};
use crate::socket;
use crate::socket::MessageSender;
use protobuf::{EnumOrUnknown, Message};
use std::io::Error;

pub struct ClientWrapper {
    client: socket::Client,
}

impl ClientWrapper {
    pub fn new(client: socket::Client) -> Self {
        Self { client }
    }

    pub fn send_key_config(
        &mut self,
        input_id: InputId,
        actions: Vec<Action>,
    ) -> Result<(), Error> {
        let protobuf = KeyConfig {
            input_id: EnumOrUnknown::new(input_id),
            actions,
            ..KeyConfig::default()
        };
        self.client
            .send_message(protobuf.write_to_bytes()?.as_slice())
    }
}
