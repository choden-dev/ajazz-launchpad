mod device_management;
mod input_handler;
mod database;

use crate::input_handler::LaunchpadInputHandler;
use firmware_api::device;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let hid_device = device_management::scan_for_launchpad();

    let device = device::Device::new(hid_device, LaunchpadInputHandler);

    loop {
        let _ = device.read_input();
        sleep(Duration::from_millis(100));
    }
}
