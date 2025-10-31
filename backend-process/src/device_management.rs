use firmware_api::device::HidDeviceWrapper;
use hidapi;
use std::thread::sleep;
use std::time::Duration;

struct DeviceIdentifier {
    pid: u16,
    vid: u16,
}

const AJAZZ_LAUNCHPAD: DeviceIdentifier = DeviceIdentifier {
    vid: 0x0300,
    pid: 0x3004,
};

pub fn scan_for_launchpad() -> HidDeviceWrapper {
    let hid_api = hidapi::HidApi::new().unwrap();

    loop {
        let launchpad = hid_api.open(AJAZZ_LAUNCHPAD.vid, AJAZZ_LAUNCHPAD.pid);

        if launchpad.is_ok() {
            return HidDeviceWrapper::new(launchpad.unwrap());
        }

        sleep(Duration::from_millis(500));
    }
}
