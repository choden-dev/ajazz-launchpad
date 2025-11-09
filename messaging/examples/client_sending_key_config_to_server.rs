//! This example focuses on what a client would send while there is a running server.
//! It requires there being an actively running `backend-process`

use messaging::socket;
use std::thread::sleep;
use std::time::Duration;

use messaging::client_wrapper::{ClientCommands, ClientWrapper};

use messaging::proto_builders::KeyConfigActionBuilder;
use messaging::protos::inputs::InputId;
use messaging::protos::keys::Key;

fn main() {
    let client: socket::Client;

    loop {
        match socket::Client::new() {
            Ok(init_client) => {
                client = init_client;
                break;
            }
            Err(err) => {
                println!("Failed to connect to server: {}", err);
                sleep(Duration::from_secs(1));
            }
        }
    }

    let mut handler = ClientWrapper::new(client);

    handler
        .send_key_config(
            InputId::KNOB_1_CLOCKWISE,
            KeyConfigActionBuilder::new()
                .add_key_action(Key::KEY_VOLUME_UP)
                .actions()
                .clone(),
        )
        .unwrap();

    handler
        .send_key_config(
            InputId::KNOB_1_COUNTER_CLOCKWISE,
            KeyConfigActionBuilder::new()
                .add_key_action(Key::KEY_VOLUME_DOWN)
                .actions()
                .clone(),
        )
        .unwrap();
}
