use crate::commands::{
    Command, clear_display_zone_image_command_factory, initiate_set_background_command_factory,
    initiate_set_display_zone_image_command_factory, output_buffer, refresh_command_factory,
    send_image_data_packet_command_factory, wake_screen_command_factory,
};
use crate::commands::{clear_all_images_command_factory, set_brightness_command_factory};
use crate::common::{ByteArray, IMAGE_DATA_PACKET_LENGTH};
use crate::display_zones::DisplayZones;
use crate::inputs::InputActions;
use crate::inputs::input_buffer::BUFFER_SIZE_13;
use hidapi::HidResult;
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub trait HidDeviceOperations {
    fn read(&self, buffer: &mut [u8]) -> HidResult<usize>;
    fn write(&self, data: &[u8]) -> HidResult<usize>;
}

pub struct HidDeviceWrapper {
    device: hidapi::HidDevice,
}

/// Warning: this device will read in non-blocking mode
impl HidDeviceWrapper {
    pub fn new(device: hidapi::HidDevice, blocking_read: bool) -> Self {
        device.set_blocking_mode(blocking_read).ok();
        Self { device }
    }
}

impl HidDeviceOperations for HidDeviceWrapper {
    fn read(&self, buffer: &mut [u8]) -> HidResult<usize> {
        self.device.read(buffer)
    }

    fn write(&self, data: &[u8]) -> HidResult<usize> {
        self.device.write(data)
    }
}

pub trait InputHandler {
    fn handle(&self, action: InputActions);
}

pub struct FunctionHandler {
    handler: fn(InputActions),
}

impl FunctionHandler {
    pub fn new(handler: fn(InputActions)) -> Self {
        Self { handler }
    }
}

impl InputHandler for FunctionHandler {
    fn handle(&self, action: InputActions) {
        (self.handler)(action);
    }
}

pub struct Device<H: HidDeviceOperations, I: InputHandler> {
    hid_device: H,
    handler: I,
}

impl<H: HidDeviceOperations, I: InputHandler> Device<H, I> {
    /// * `hid_device`: an opened HID device that is the launchpad and contains the required operations (read/write)
    /// * `handler`: callback which accepts an `InputAction` (will be called everytime there is a hardware action)
    pub fn new(hid_device: H, handler: I) -> Self {
        Self {
            hid_device,
            handler,
        }
    }

    pub fn handler(&self) -> &I {
        &self.handler
    }

    pub fn update_handler(&mut self, handler: I) {
        self.handler = handler;
    }

    /// Processes a hardware action performed by the launchpad.
    /// This will return an `InputAction` that is to be handled by a callback
    pub fn read_input(&self) -> Result<(), Box<dyn Error>> {
        let mut buffer: ByteArray<BUFFER_SIZE_13> = [0; BUFFER_SIZE_13];
        self.hid_device.read(&mut buffer)?;

        // If we have empty buffer means that no available message was there
        if !buffer.iter().all(|&bit| bit == 0) {
            let action = InputActions::from(buffer);
            self.handler.handle(action);
        }

        Ok(())
    }

    /// Sets the brightness of the launchpad displays to a percentage (0 - 100)
    /// * `brightness` - any value between 0 and 100
    pub fn set_brightness(&self, brightness: u8) -> HidResult<usize> {
        let set_brightness_command = set_brightness_command_factory(match brightness {
            0..=100 => brightness,
            _ => 0,
        });
        set_brightness_command.execute(|buf| self.hid_device.write(buf))
    }

    /// Will wake up the device from its sleep state, otherwise does nothing
    pub fn wake_screen(&self) -> HidResult<usize> {
        let wake_screen_command = wake_screen_command_factory();
        wake_screen_command.execute(|buf| self.hid_device.write(buf))
    }

    pub fn refresh(&self) -> HidResult<usize> {
        let refresh_command = refresh_command_factory();
        refresh_command.execute(|buf| self.hid_device.write(buf))
    }

    pub fn clear_all_images(&self) -> HidResult<usize> {
        let clear_all_images_command = clear_all_images_command_factory();
        clear_all_images_command.execute(|buf| self.hid_device.write(buf))
    }

    pub fn clear_display_zone_image(&self, display_zone: DisplayZones) -> HidResult<usize> {
        let clear_display_zone_image_command =
            clear_display_zone_image_command_factory(display_zone);
        clear_display_zone_image_command.execute(|buf| self.hid_device.write(buf))
    }

    pub fn set_background_image(&self, file: File) -> HidResult<usize> {
        // Let the device know to prepare
        let init_command = initiate_set_background_command_factory(file.metadata()?.len() as u32);
        self.write_image_to_device_command(init_command, file)
    }

    pub fn set_display_zone_image(
        &self,
        display_zone: DisplayZones,
        file: File,
    ) -> HidResult<usize> {
        let init_command = initiate_set_display_zone_image_command_factory(
            file.metadata()?.len() as u32,
            display_zone,
        );
        self.write_image_to_device_command(init_command, file)
    }

    /// Generic factory to deal with image writing operations
    fn write_image_to_device_command(
        &self,
        init_command: impl Command<{ output_buffer::BUFFER_SIZE_1025 }, HidResult<usize>>,
        mut file: File,
    ) -> HidResult<usize> {
        // Let the device know to prepare
        init_command.execute(|buf| self.hid_device.write(buf))?;

        let mut buffer: ByteArray<IMAGE_DATA_PACKET_LENGTH> = [0; IMAGE_DATA_PACKET_LENGTH];
        let mut last_result = Ok(0);

        // Send image data until EOF
        while file.read(&mut buffer)? > 0 {
            let command = send_image_data_packet_command_factory(buffer);
            last_result = command.execute(|buf| self.hid_device.write(buf));
        }

        last_result
    }
}

impl Device<HidDeviceWrapper, FunctionHandler> {
    pub fn from_hid_device(
        hid_device: hidapi::HidDevice,
        handler: FunctionHandler,
        blocking_read: bool,
    ) -> Self {
        Self::new(HidDeviceWrapper::new(hid_device, blocking_read), handler)
    }
}
