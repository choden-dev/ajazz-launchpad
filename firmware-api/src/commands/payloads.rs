use crate::commands::messages;
use crate::commands::output_buffer::{BUFFER_SIZE_513, create_output_buffer};
use crate::common::ByteArray;

pub trait Payload<const N: usize> {
    fn generate(&self) -> ByteArray<N>;
}

pub struct WakeScreen;
impl Payload<BUFFER_SIZE_513> for WakeScreen {
    fn generate(&self) -> ByteArray<BUFFER_SIZE_513> {
        create_output_buffer(&messages::WAKE_SCREEN)
    }
}

pub struct Refresh;
impl Payload<BUFFER_SIZE_513> for Refresh {
    fn generate(&self) -> ByteArray<BUFFER_SIZE_513> {
        create_output_buffer(&messages::REFRESH)
    }
}

pub struct SetBrightness {
    brightness: u8,
}

impl SetBrightness {
    pub fn new(brightness: u8) -> Self {
        Self { brightness }
    }
}

impl Payload<BUFFER_SIZE_513> for SetBrightness {
    fn generate(&self) -> ByteArray<BUFFER_SIZE_513> {
        let mut default_buffer = create_output_buffer(&messages::SET_BRIGHTNESS);
        default_buffer[messages::SET_BRIGHTNESS.len()] = self.brightness;
        default_buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::messages::{REFRESH, SET_BRIGHTNESS, WAKE_SCREEN};

    #[test]
    fn correct_refresh_payload() {
        let payload = Refresh::generate(&Refresh);
        assert_eq!(payload, create_output_buffer(&REFRESH))
    }

    #[test]
    fn correct_wake_screen_payload() {
        let payload = WakeScreen::generate(&WakeScreen);
        assert_eq!(payload, create_output_buffer(&WAKE_SCREEN))
    }

    #[test]
    fn correct_set_brightness_payload() {
        let payload = SetBrightness::generate(&SetBrightness { brightness: 30 });

        let mut message_buffer = [0; 12];
        message_buffer[..11].copy_from_slice(&SET_BRIGHTNESS);

        message_buffer[11] = 30;

        assert_eq!(payload, create_output_buffer(&message_buffer))
    }
}
