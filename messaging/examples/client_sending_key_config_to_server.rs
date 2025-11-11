//! This example focuses on what a client would send while there is a running server.
//! It requires there being an actively running `backend-process`

use messaging::socket;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, io};

use messaging::client_wrapper::{ClientCommands, ClientWrapper};

use messaging::proto_builders::KeyConfigActionBuilder;
use messaging::protos::display_zones::DisplayZone;
use messaging::protos::inputs::InputId;
use messaging::protos::keys::Key;

fn main() {
    let client: socket::Client;
    let mut buffer = String::new();

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

    println!("Choose one of the following scenarios:");
    println!("1. Set key config");
    println!("2. Set boot logo");
    println!("3. Set a key image");
    println!("4. Set brightness");
    println!("5. Clear all key images");
    println!("6. Clear single key image");
    io::stdin().read_line(&mut buffer).unwrap();

    let mut handler = ClientWrapper::new(client);
    match u8::from_str(buffer.trim()) {
        Ok(v) => match v {
            1 => {
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

            2 => handler
                .set_boot_logo(String::from(
                    fs::canonicalize(
                        "./firmware-api/examples/assets/example-touchscreen-zone-image.jpg",
                    )
                    .unwrap()
                    .to_str()
                    .unwrap(),
                ))
                .unwrap(),
            3 => handler
                .set_display_zone_image(
                    DisplayZone::BUTTON_2,
                    String::from(
                        fs::canonicalize("./firmware-api/examples/assets/example-button-image.jpg")
                            .unwrap()
                            .to_str()
                            .unwrap(),
                    ),
                )
                .unwrap(),
            4 => handler.set_brightness(2).unwrap(),
            5 => handler.clear_all_images().unwrap(),
            6 => handler
                .clear_display_zone_image(DisplayZone::BUTTON_2)
                .unwrap(),
            _ => {
                panic!("Out of range of options!")
            }
        },
        Err(_) => {
            panic!("Did not enter a valid choice")
        }
    }
}
