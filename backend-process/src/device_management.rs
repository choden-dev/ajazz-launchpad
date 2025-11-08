use hidapi::HidDevice;
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

pub fn scan_for_launchpad() -> HidDevice {
    let mut hid_api = hidapi::HidApi::new_without_enumerate().unwrap();

    loop {
        hid_api.reset_devices().unwrap();
        hid_api
            .add_devices(AJAZZ_LAUNCHPAD.vid, AJAZZ_LAUNCHPAD.pid)
            .unwrap();

        // Bit of a hack, there are 3 identified devices with the given vid/pid, so need to find the one that works
        for device in hid_api.device_list() {
            // Refer to https://learn.microsoft.com/en-us/windows-hardware/drivers/hid/hid-usages
            let launchpad = hid_api.open_path(device.path());

            if let Ok(device) = launchpad {
                return device;
            }
        }

        sleep(Duration::from_millis(500));
    }
}
