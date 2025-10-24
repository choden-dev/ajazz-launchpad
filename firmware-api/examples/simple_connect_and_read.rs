use firmware_api::device::{Device, FunctionHandler, HidDeviceWrapper};
use hidapi::HidApi;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let hid_api = HidApi::new().unwrap_or_else(|e| panic!("Failed to initialize HID API: {}", e));

    let hid_device = hid_api
        .open(0x0300, 0x3004)
        .unwrap_or_else(|e| panic!("Failed to open device: {}", e));

    let device = Device::new(
        HidDeviceWrapper::new(hid_device),
        FunctionHandler::new(|action| println!("{:?}", action)),
    );

    device
        .refresh()
        .unwrap_or_else(|e| panic!("Failed to refresh device: {}", e));

    loop {
        device
            .read_input()
            .unwrap_or_else(|e| println!("Failed to read input: {}", e));
        sleep(Duration::from_millis(500));
    }
}
