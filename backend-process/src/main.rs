mod database;
mod device_management;
mod input_handler;
mod protobuf_conversion;
mod socket;

use crate::database::operations::Operations;
use crate::input_handler::{
    EnigoKeyActionHandler, InputMapping, KeyActionExecutor, LaunchpadInputHandler,
};
use crate::socket::commands::IncomingCommands;
use firmware_api::device;
use firmware_api::device::HidDeviceWrapper;
use log::info;
use std::fs::File;

#[derive(Clone)]
enum States {
    EstablishConnection,
    ReadClientMessages,
    HandleDeviceInput,
    PruneConnections,
}

struct StateMachine {
    current_state: States,
}

impl StateMachine {
    fn new() -> Self {
        Self {
            current_state: States::EstablishConnection,
        }
    }
    fn next_state(&mut self, current_connections: u8) {
        match self.current_state {
            States::EstablishConnection => match current_connections {
                connections if connections > 0 => self.current_state = States::ReadClientMessages,
                _ => self.current_state = States::HandleDeviceInput,
            },
            States::HandleDeviceInput => match current_connections {
                connections if connections > 0 => self.current_state = States::ReadClientMessages,
                _ => self.current_state = States::EstablishConnection,
            },
            States::ReadClientMessages => {
                self.current_state = States::PruneConnections;
            }
            States::PruneConnections => {
                self.current_state = States::HandleDeviceInput;
            }
        }
    }

    fn current_state(&self) -> States {
        self.current_state.clone()
    }
}

fn main() {
    env_logger::init();
    let mut state_machine = StateMachine::new();

    let db = Operations::new(database::sqlite::SqLite::new(true).unwrap());
    let mut default_mappings = InputMapping::default();
    default_mappings.override_config(db.get_all_input_mappings().unwrap().into());

    let mut server = socket::connection::ServerHandler::new(&db).expect("Failed to create server");

    let key_action_handler: Box<dyn KeyActionExecutor> = Box::new(EnigoKeyActionHandler::default());
    let input_handler = LaunchpadInputHandler::new(default_mappings, key_action_handler.as_ref());
    let hid_device = device_management::scan_for_launchpad();
    let mut device = device::Device::new(HidDeviceWrapper::new(&hid_device, false), input_handler);
    device.refresh().unwrap();

    loop {
        let current_state = state_machine.current_state();

        match current_state {
            States::EstablishConnection => match server.add_new_connection_if_exists() {
                Ok(_) => {
                    info!("New connection added");
                }
                Err(e) => {
                    info!("New connection could not be added: {}", e);
                }
            },
            States::ReadClientMessages => match server.handle_command_and_persist_config() {
                Ok(message_type) => match message_type {
                    IncomingCommands::SetKeyConfig(mapping) => {
                        let input_handler = LaunchpadInputHandler::new(
                            device.handler().new_updated_mappings(mapping),
                            key_action_handler.as_ref(),
                        );
                        device.update_handler(input_handler);
                    }
                    IncomingCommands::SetDisplayZoneImage(mapping) => {
                        if let Ok(image) = File::open(mapping.image_path) {
                            device
                                .set_display_zone_image(mapping.display_zone, image)
                                .ok();
                        }
                    }
                    IncomingCommands::SetBrightness(brightness) => {
                        device.set_brightness(brightness).ok();
                    }
                    IncomingCommands::ClearDisplayZoneImage(display_zone) => {
                        device.clear_display_zone_image(display_zone).ok();
                    }
                    IncomingCommands::SetBootLogo(file_path) => {
                        if let Ok(image) = File::open(file_path) {
                            device.set_background_image(image).ok();
                        }
                    }
                    IncomingCommands::ClearAllDisplayZoneImages => {
                        device.clear_all_images().ok();
                    }
                },
                Err(e) => {
                    info!("No known command was handled: {}", e);
                }
            },
            States::HandleDeviceInput => {
                device.read_input().ok();
            }

            States::PruneConnections => {
                server.prune_connections().ok();
            }
        }

        state_machine.next_state(server.server().connected_clients() as u8);
    }
}
