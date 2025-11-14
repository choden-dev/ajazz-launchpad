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
        HidDeviceWrapper::new(hid_device, false),
        FunctionHandler::new(|_| {}),
    );
    device
        .refresh()
        .unwrap_or_else(|e| panic!("Failed to refresh device: {}", e));

    let _ = device.set_brightness(0);
    sleep(Duration::from_secs(2));

    let _ = device.set_brightness(50);
    sleep(Duration::from_secs(2));

    let _ = device.set_brightness(100);
    sleep(Duration::from_secs(2));

    let _ = device.set_brightness(20);
    sleep(Duration::from_secs(2));
}
