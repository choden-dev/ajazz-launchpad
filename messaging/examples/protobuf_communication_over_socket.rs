use messaging::protos;
use messaging::socket;
use messaging::socket::{MessageReceiver, MessageSender};
use protobuf::Message;
use std::thread;
use std::time::Duration;

fn main() {
    let mut server = socket::Server::new().unwrap();

    let server_handle = thread::spawn(move || {
        server
            .accept_connection()
            .expect("Failed to accept connections");

        loop {
            println!("Current connections {}", server.connected_clients());
            match server.read_message() {
                Ok(msg) => {
                    println!("Received message (byte form): {:?}", msg);
                    if let Ok(parsed) =
                        protos::key_config::KeyConfig::parse_from_bytes(msg.as_slice())
                    {
                        println!("{}", parsed);
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }

            thread::sleep(Duration::from_millis(400));
        }
    });

    let mut client: socket::Client;

    loop {
        if let Ok(loop_client) = socket::Client::new() {
            client = loop_client;
            println!("Client connected");
            break;
        }
    }

    match client.send_message(
        protos::key_config::KeyConfig {
            input_id: protobuf::EnumOrUnknown::new(
                protos::inputs::InputId::KNOB_1_COUNTER_CLOCKWISE,
            ),
            actions: vec![protos::key_config::Action {
                action_data: Some(protos::key_config::action::Action_data::KeyAction(
                    protos::key_config::KeyAction {
                        key: protobuf::EnumOrUnknown::from(protos::keys::Key::KEY_ADD),
                        ..protos::key_config::KeyAction::default()
                    },
                )),
                ..protos::key_config::Action::default()
            }],
            ..protos::key_config::KeyConfig::default()
        }
        .write_to_bytes()
        .expect("Failed to serialize protobuf")
        .as_slice(),
    ) {
        Ok(_) => {
            println!("Message sent");
        }
        Err(e) => {
            println!("Error sending message: {}", e);
        }
    }

    server_handle.join().unwrap();
}
