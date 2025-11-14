use hidapi::{HidApi, HidDevice};
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

pub struct DeviceManagement {
    hid_api: HidApi,
}

impl DeviceManagement {
    pub fn new() -> Self {
        Self {
            hid_api: HidApi::new_without_enumerate().unwrap(),
        }
    }

    fn refresh_launchpad_filter(&mut self) {
        self.hid_api.reset_devices().unwrap();
        self.hid_api
            .add_devices(AJAZZ_LAUNCHPAD.vid, AJAZZ_LAUNCHPAD.pid)
            .unwrap();
    }
    pub fn scan_for_launchpad(&mut self) -> HidDevice {
        loop {
            self.refresh_launchpad_filter();

            // Bit of a hack, there are 3 identified devices with the given vid/pid, so need to find the one that works
            for device in self.hid_api.device_list() {
                // Refer to https://learn.microsoft.com/en-us/windows-hardware/drivers/hid/hid-usages
                let launchpad = self.hid_api.open_path(device.path());

                if let Ok(device) = launchpad {
                    return device;
                }
            }

            sleep(Duration::from_millis(500));
        }
    }
}
