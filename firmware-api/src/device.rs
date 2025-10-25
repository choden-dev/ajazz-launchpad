use crate::commands::{
    Command, initiate_set_background_command_factory, refresh_command_factory,
    send_image_data_packet_command_factory, wake_screen_command_factory,
};
use crate::commands::{clear_all_images_command_factory, set_brightness_command_factory};
use crate::common::{ByteArray, IMAGE_DATA_PACKET_LENGTH, IMAGE_SIZE_LENGTH_IN_BYTES};
use crate::inputs::InputActions;
use crate::inputs::input_buffer::BUFFER_SIZE_13;
use hidapi::{HidError, HidResult};
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

impl HidDeviceWrapper {
    pub fn new(device: hidapi::HidDevice) -> Self {
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

    /// Processes a hardware action performed by the launchpad.
    /// This will return an `InputAction` that is to be handled by a callback
    pub fn read_input(&self) -> Result<(), Box<dyn Error>> {
        let mut buffer: ByteArray<BUFFER_SIZE_13> = [0; BUFFER_SIZE_13];
        self.hid_device.read(&mut buffer)?;

        let action = InputActions::from(buffer);
        self.handler.handle(action);
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

    pub fn set_background_image(&self, image_size: u32, mut file: File) -> HidResult<usize> {
        // Let the device know to prepare
        let init_command = initiate_set_background_command_factory(image_size);
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
    pub fn from_hid_device(hid_device: hidapi::HidDevice, handler: fn(InputActions)) -> Self {
        Self::new(
            HidDeviceWrapper::new(hid_device),
            FunctionHandler::new(handler),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hidapi::HidError;
    use std::cell::RefCell;
    use std::rc::Rc;

    struct MockHidDevice {
        read_data: Vec<u8>,
        read_calls: Rc<RefCell<usize>>,
        write_calls: Rc<RefCell<Vec<Vec<u8>>>>,
        read_error: Option<hidapi::HidError>,
        write_error: Option<hidapi::HidError>,
    }
    impl MockHidDevice {
        fn new() -> Self {
            Self {
                read_data: vec![0; BUFFER_SIZE_13],
                read_calls: Rc::new(RefCell::new(0)),
                write_calls: Rc::new(RefCell::new(Vec::new())),
                read_error: None,
                write_error: None,
            }
        }
        fn with_read_data(mut self, data: Vec<u8>) -> Self {
            self.read_data = data;
            self
        }
        fn with_read_error(mut self, error: HidError) -> Self {
            self.read_error = Some(error);
            self
        }
        fn with_write_error(mut self, error: HidError) -> Self {
            self.write_error = Some(error);
            self
        }
        fn get_read_calls(&self) -> usize {
            *self.read_calls.borrow()
        }
        fn get_write_calls(&self) -> Vec<Vec<u8>> {
            self.write_calls.borrow().clone()
        }
    }
    impl HidDeviceOperations for MockHidDevice {
        fn read(&self, buffer: &mut [u8]) -> HidResult<usize> {
            *self.read_calls.borrow_mut() += 1;

            if let Some(ref error) = self.read_error {
                return Err(HidError::HidApiError {
                    message: error.to_string(),
                });
            }

            let len = std::cmp::min(buffer.len(), self.read_data.len());
            buffer[..len].copy_from_slice(&self.read_data[..len]);
            Ok(len)
        }
        fn write(&self, data: &[u8]) -> HidResult<usize> {
            self.write_calls.borrow_mut().push(data.to_vec());

            if let Some(ref error) = self.write_error {
                return Err(HidError::HidApiError {
                    message: error.to_string(),
                });
            }

            Ok(data.len())
        }
    }
    struct MockInputHandler {
        calls: Rc<RefCell<Vec<InputActions>>>,
    }
    impl MockInputHandler {
        fn new() -> Self {
            Self {
                calls: Rc::new(RefCell::new(Vec::new())),
            }
        }
        fn get_calls(&self) -> Vec<InputActions> {
            self.calls.borrow().clone()
        }
    }
    impl InputHandler for MockInputHandler {
        fn handle(&self, action: InputActions) {
            self.calls.borrow_mut().push(action);
        }
    }
    #[test]
    fn test_read_input_success() {
        let mock_device =
            MockHidDevice::new().with_read_data(vec![1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0]);
        let mock_handler = MockInputHandler::new();
        let device = Device::new(mock_device, mock_handler);
        let result = device.read_input();
        assert!(result.is_ok());
        assert_eq!(device.hid_device.get_read_calls(), 1);
        assert_eq!(device.handler.get_calls().len(), 1);
    }
    #[test]
    fn test_read_input_hid_error() {
        let mock_device = MockHidDevice::new().with_read_error(HidError::HidApiError {
            message: "Test error".to_string(),
        });
        let mock_handler = MockInputHandler::new();
        let device = Device::new(mock_device, mock_handler);
        let result = device.read_input();
        assert!(result.is_err());
        assert_eq!(device.hid_device.get_read_calls(), 1);
        assert_eq!(device.handler.get_calls().len(), 0); // Handler shouldn't be called on error
    }
    #[test]
    fn test_set_brightness_valid_range() {
        let mock_device = MockHidDevice::new();
        let mock_handler = MockInputHandler::new();
        let device = Device::new(mock_device, mock_handler);
        let result = device.set_brightness(50);
        assert!(result.is_ok());
        let write_calls = device.hid_device.get_write_calls();
        assert_eq!(write_calls.len(), 1);
        // You could verify the actual command bytes here
    }
    #[test]
    fn test_set_brightness_invalid_range() {
        let mock_device = MockHidDevice::new();
        let mock_handler = MockInputHandler::new();
        let device = Device::new(mock_device, mock_handler);
        let result = device.set_brightness(150); // Invalid brightness
        assert!(result.is_ok());
        let write_calls = device.hid_device.get_write_calls();
        assert_eq!(write_calls.len(), 1);
        // Verify that brightness was clamped to 0
    }
    #[test]
    fn test_set_brightness_write_error() {
        let mock_device = MockHidDevice::new().with_write_error(HidError::HidApiError {
            message: "Write failed".to_string(),
        });
        let mock_handler = MockInputHandler::new();
        let device = Device::new(mock_device, mock_handler);
        let result = device.set_brightness(50);
        assert!(result.is_err());
    }
    #[test]
    fn test_wake_screen() {
        let mock_device = MockHidDevice::new();
        let mock_handler = MockInputHandler::new();
        let device = Device::new(mock_device, mock_handler);
        let result = device.wake_screen();
        assert!(result.is_ok());
        let write_calls = device.hid_device.get_write_calls();
        assert_eq!(write_calls.len(), 1);
    }
    #[test]
    fn test_refresh() {
        let mock_device = MockHidDevice::new();
        let mock_handler = MockInputHandler::new();
        let device = Device::new(mock_device, mock_handler);
        let result = device.refresh();
        assert!(result.is_ok());
        let write_calls = device.hid_device.get_write_calls();
        assert_eq!(write_calls.len(), 1);
    }
}
