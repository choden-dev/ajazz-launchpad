mod database;
mod device_management;
mod input_handler;
mod protobuf_conversion;

use crate::input_handler::{EnigoKeyActionHandler, InputMapping, LaunchpadInputHandler};
use firmware_api::device;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let hid_device = device_management::scan_for_launchpad();

    let device = device::Device::new(
        hid_device,
        LaunchpadInputHandler::new(
            InputMapping::default(),
            Box::new(EnigoKeyActionHandler::default()),
        ),
    );

    device.refresh().unwrap();

    loop {
        let _ = device.read_input();
        sleep(Duration::from_millis(100));
    }
}
