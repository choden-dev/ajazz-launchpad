mod database;
mod device_management;
mod input_handler;
mod protobuf_conversion;
mod socket;

use crate::database::operations::Operations;
use crate::device_management::DeviceManagement;
use crate::input_handler::{
    EnigoKeyActionHandler, InputMapping, KeyActionExecutor, LaunchpadInputHandler,
};
use crate::socket::commands::IncomingCommands;
use firmware_api::device;
use log::{debug, info};
use std::fs::File;

#[derive(Clone)]
enum States {
    EstablishConnection,
    ReadClientMessages,
    HandleDeviceInput,
    PruneConnections,
    InitialiseDevice,
}

struct StateMachine {
    current_state: States,
}

impl StateMachine {
    fn new() -> Self {
        Self {
            current_state: States::InitialiseDevice,
        }
    }
    fn next_state(&mut self, current_connections: u8, device_is_connected: bool) {
        match self.current_state {
            States::InitialiseDevice => {
                self.current_state = States::PruneConnections;
            }
            States::EstablishConnection => match current_connections {
                connections if connections > 0 => self.current_state = States::ReadClientMessages,
                _ => self.current_state = States::HandleDeviceInput,
            },
            States::HandleDeviceInput => match device_is_connected {
                true => match current_connections {
                    connections if connections > 0 => {
                        self.current_state = States::ReadClientMessages
                    }
                    _ => self.current_state = States::EstablishConnection,
                },
                false => self.current_state = States::InitialiseDevice,
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
    let mut server = socket::connection::ServerHandler::new(&db).expect("Failed to create server");
    let mut device: Option<device::Device<device::HidDeviceWrapper, LaunchpadInputHandler>> = None;
    let key_action_handler: Box<dyn KeyActionExecutor> = Box::new(EnigoKeyActionHandler::default());
    loop {
        let current_state = state_machine.current_state();
        let mut device_disconnected_during_read = false;
        match current_state {
            States::InitialiseDevice => {
                let mut device_management = DeviceManagement::new();
                let hid_device = device_management.scan_for_launchpad();
                // Image config fetching
                let default_images = db.get_all_image_mappings().unwrap();

                // Input config fetching
                let mut default_mappings = InputMapping::default();
                default_mappings.override_config(db.get_all_input_mappings().unwrap().into());
                let input_handler =
                    LaunchpadInputHandler::new(default_mappings, key_action_handler.as_ref());

                // Brightness config fetching
                let stored_brightness = db.get_stored_brightness().unwrap();

                let new_device = device::Device::new(
                    device::HidDeviceWrapper::new(hid_device, false), // No borrowing here
                    input_handler,
                );
                new_device.refresh().unwrap();
                for default_mapping in default_images {
                    if let Ok(image) = File::open(default_mapping.image_path) {
                        new_device
                            .set_display_zone_image(default_mapping.display_zone, image)
                            .ok();
                    }
                }
                if let Some(brightness) = stored_brightness {
                    new_device.set_brightness(brightness).ok();
                }
                device = Some(new_device);
            }
            States::EstablishConnection => match server.add_new_connection_if_exists() {
                Ok(_) => {
                    debug!("New connection added");
                }
                Err(e) => {
                    debug!("New connection could not be added: {}", e);
                }
            },
            States::ReadClientMessages => {
                if let Some(ref mut dev) = device {
                    match server.handle_command_and_persist_config() {
                        Ok(message_type) => match message_type {
                            IncomingCommands::SetKeyConfig(mapping) => {
                                let input_handler = LaunchpadInputHandler::new(
                                    dev.handler().new_updated_mappings(mapping),
                                    key_action_handler.as_ref(),
                                );
                                dev.update_handler(input_handler);
                            }
                            IncomingCommands::SetDisplayZoneImage(mapping) => {
                                if let Ok(image) = File::open(mapping.image_path) {
                                    dev.set_display_zone_image(mapping.display_zone, image).ok();
                                }
                            }
                            IncomingCommands::SetBrightness(brightness) => {
                                dev.set_brightness(brightness).ok();
                            }
                            IncomingCommands::ClearDisplayZoneImage(display_zone) => {
                                dev.clear_display_zone_image(display_zone).ok();
                            }
                            IncomingCommands::SetBootLogo(file_path) => {
                                if let Ok(image) = File::open(file_path) {
                                    dev.set_background_image(image).ok();
                                }
                            }
                            IncomingCommands::ClearAllDisplayZoneImages => {
                                dev.clear_all_images().ok();
                            }
                        },
                        Err(e) => {
                            info!("No known command was handled: {}", e);
                        }
                    }
                }
            }
            States::HandleDeviceInput => {
                if let Some(ref mut dev) = device {
                    dev.read_input().unwrap_or_else(|e| {
                        debug!("Error reading input: {}", e);
                        if e.to_string() == "hidapi error: hid_read_timeout: device disconnected" {
                            info!("Disconnected from device");
                            device_disconnected_during_read = true;
                        }
                    });
                }
            }
            States::PruneConnections => {
                server.prune_connections().ok();
            }
        }

        let device_is_connected = device.is_some() && !device_disconnected_during_read;
        if device_disconnected_during_read {
            device = None;
        }
        state_machine.next_state(
            server.server().connected_clients() as u8,
            device_is_connected,
        );
    }
}
