mod database;
mod device_management;
mod input_handler;
mod protobuf_conversion;
mod socket;

use crate::database::operations::Operations;
use crate::input_handler::{EnigoKeyActionHandler, InputMapping, LaunchpadInputHandler};
use firmware_api::device;
use log::info;

fn main() {
    env_logger::init();

    let db = Operations::new(database::sqlite::SqLite::new(true).unwrap());
    let mut default_mappings = InputMapping::default();
    let stored_mappings = db.get_all_input_mappings().unwrap();
    default_mappings.override_config(stored_mappings.into());

    let mut server = socket::connection::ServerHandler::new(&db).expect("Failed to create server");

    let hid_device = device_management::scan_for_launchpad();
    let device = device::Device::new(
        hid_device,
        LaunchpadInputHandler::new(default_mappings, Box::new(EnigoKeyActionHandler::default())),
    );
    device.refresh().unwrap();

    loop {
        match server.server().connected_clients() {
            connections if connections == 0 => match server.add_new_connection_if_exists() {
                Ok(_) => {
                    info!("New connection added");
                }
                Err(e) => {
                    info!("New connection could not be added: {}", e);
                }
            },
            _ => {
                match server.handle_next_message() {
                    Ok(messageType) => {}
                    Err(e) => {}
                };
            }
        }
        device.read_input().ok();
    }
}
