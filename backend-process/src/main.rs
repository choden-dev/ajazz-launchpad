mod database;
mod device_management;
mod input_handler;
mod protobuf_conversion;

use crate::database::operations::Operations;
use crate::input_handler::{EnigoKeyActionHandler, InputMapping, LaunchpadInputHandler};
use firmware_api::device;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let db = Operations::new(database::sqlite::SqLite::new(true).unwrap());
    let mut default_mappings = InputMapping::default();
    let stored_mappings = db.get_all_input_mappings().unwrap();
    default_mappings.override_config(stored_mappings.into());

    let hid_device = device_management::scan_for_launchpad();
    let device = device::Device::new(
        hid_device,
        LaunchpadInputHandler::new(default_mappings, Box::new(EnigoKeyActionHandler::default())),
    );
    device.refresh().unwrap();

    loop {
        let _ = device.read_input();
        sleep(Duration::from_millis(100));
    }
}
