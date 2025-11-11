use firmware_api::device::{Device, FunctionHandler, HidDeviceWrapper};
use firmware_api::display_zones::DisplayZones;
use hidapi::HidApi;
use std::fs::File;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

const ALL_BUTTONS: [DisplayZones; 10] = [
    DisplayZones::Button1,
    DisplayZones::Button2,
    DisplayZones::Button3,
    DisplayZones::Button4,
    DisplayZones::Button5,
    DisplayZones::Button6,
    DisplayZones::Button7,
    DisplayZones::Button8,
    DisplayZones::Button9,
    DisplayZones::Button10,
];

const ALL_TOUCHSCREEN_ZONES: [DisplayZones; 4] = [
    DisplayZones::Touchscreen1,
    DisplayZones::Touchscreen2,
    DisplayZones::Touchscreen3,
    DisplayZones::Touchscreen4,
];
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

    sleep(Duration::from_secs(2));

    println!("Setting up button images");

    ALL_BUTTONS.iter().for_each(|button| {
        println!("Setting {:?}", button);
        let button_image = File::open(Path::new(
            "./firmware-api/examples/assets/example-button-image.jpg",
        ))
        .unwrap();
        device
            .set_display_zone_image(*button, button_image)
            .unwrap_or_else(|e| panic!("Failed to set button image: {}", e));
        device
            .refresh()
            .unwrap_or_else(|e| panic!("Failed to refresh device: {}", e));

        sleep(Duration::from_millis(200));
    });

    println!("Setting up touch screen images");

    ALL_TOUCHSCREEN_ZONES.iter().for_each(|touchscreen_zone| {
        println!("Setting {:?}", touchscreen_zone);
        let touchscreen_image = File::open(Path::new(
            "firmware-api/examples/assets/example-touchscreen-zone-image.jpg",
        ))
        .unwrap();
        device
            .set_display_zone_image(*touchscreen_zone, touchscreen_image)
            .unwrap_or_else(|e| panic!("Failed to set button image: {}", e));
        device
            .refresh()
            .unwrap_or_else(|e| panic!("Failed to refresh device: {}", e));

        sleep(Duration::from_millis(200));
    });

    println!("Clearing a button image...");
    let _ = device.clear_display_zone_image(DisplayZones::Button5);

    println!("Clearing a touchscreen image...");
    let _ = device.clear_display_zone_image(DisplayZones::Touchscreen2);
}
