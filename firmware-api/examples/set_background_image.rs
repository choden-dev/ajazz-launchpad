use firmware_api::device::{Device, FunctionHandler, HidDeviceWrapper};
use hidapi::HidApi;
use std::fs::File;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let hid_api = HidApi::new().unwrap_or_else(|e| panic!("Failed to initialize HID API: {}", e));

    let hid_device = hid_api
        .open(0x0300, 0x3004)
        .unwrap_or_else(|e| panic!("Failed to open device: {}", e));

    let device = Device::new(
        HidDeviceWrapper::new(&hid_device, false),
        FunctionHandler::new(|action| println!("{:?}", action)),
    );

    device
        .refresh()
        .unwrap_or_else(|e| panic!("Failed to refresh device: {}", e));

    let _ = device.clear_all_images();

    sleep(Duration::from_secs(3));

    println!("Setting up background image");

    // Set up the required image parameters (size in bytes and the file stream)
    let background_image = File::open(Path::new(
        "./firmware-api/examples/assets/example-background-image.jpg",
    ))
    .unwrap();

    device
        .set_background_image(background_image)
        .unwrap_or_else(|e| panic!("Failed to set background image: {}", e));
    device
        .refresh()
        .unwrap_or_else(|e| panic!("Failed to refresh device: {}", e));
}
