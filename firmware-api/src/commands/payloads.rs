use crate::commands::messages;
use crate::commands::output_buffer::{BUFFER_SIZE_513, BUFFER_SIZE_1025, create_output_buffer};
use crate::common::{ByteArray, IMAGE_DATA_PACKET_LENGTH, IMAGE_SIZE_LENGTH_IN_BYTES};
use crate::display_zones::DisplayZones;

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

pub struct ClearAllImages;

impl Payload<BUFFER_SIZE_513> for ClearAllImages {
    fn generate(&self) -> ByteArray<BUFFER_SIZE_513> {
        create_output_buffer(&messages::CLEAR_ALL_IMAGES)
    }
}

pub struct InitiateSetBackgroundImage {
    image_size_bytes: u32,
}

impl InitiateSetBackgroundImage {
    pub fn new(image_size_bytes: u32) -> Self {
        Self { image_size_bytes }
    }
}
impl Payload<BUFFER_SIZE_1025> for InitiateSetBackgroundImage {
    fn generate(&self) -> ByteArray<BUFFER_SIZE_1025> {
        let mut default_buffer = create_output_buffer(&messages::INITIATE_SET_BACKGROUND_IMAGE);
        let last_index = messages::INITIATE_SET_BACKGROUND_IMAGE.len();

        default_buffer[last_index..last_index + IMAGE_SIZE_LENGTH_IN_BYTES]
            .clone_from_slice(&self.image_size_bytes.to_be_bytes());

        default_buffer[last_index + IMAGE_SIZE_LENGTH_IN_BYTES] = 0x01;

        default_buffer
    }
}

pub struct InitiateDisplayZoneImage {
    image_size_bytes: u32,
    display_zone: DisplayZones,
}

impl InitiateDisplayZoneImage {
    pub fn new(image_size_bytes: u32, display_zone: DisplayZones) -> Self {
        Self {
            image_size_bytes,
            display_zone,
        }
    }
}

impl Payload<BUFFER_SIZE_1025> for InitiateDisplayZoneImage {
    fn generate(&self) -> ByteArray<BUFFER_SIZE_1025> {
        let mut default_buffer = create_output_buffer(&messages::INITIATE_SET_DISPLAY_ZONE_IMAGE);
        let last_index = messages::INITIATE_SET_DISPLAY_ZONE_IMAGE.len();

        default_buffer[last_index..last_index + IMAGE_SIZE_LENGTH_IN_BYTES]
            .clone_from_slice(&self.image_size_bytes.to_be_bytes());

        default_buffer[last_index + IMAGE_SIZE_LENGTH_IN_BYTES] =
            DisplayZones::into(self.display_zone);

        default_buffer
    }
}

pub struct SendImageDataPacket {
    packet: ByteArray<IMAGE_DATA_PACKET_LENGTH>,
}
impl SendImageDataPacket {
    pub fn new(packet: ByteArray<IMAGE_DATA_PACKET_LENGTH>) -> Self {
        Self { packet }
    }
}
impl Payload<BUFFER_SIZE_1025> for SendImageDataPacket {
    fn generate(&self) -> ByteArray<BUFFER_SIZE_1025> {
        let mut default_buffer: ByteArray<BUFFER_SIZE_1025> = [0; 1025];
        default_buffer[1..BUFFER_SIZE_1025].clone_from_slice(&self.packet);
        default_buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::messages::{
        INITIATE_SET_BACKGROUND_IMAGE, INITIATE_SET_DISPLAY_ZONE_IMAGE, REFRESH, SET_BRIGHTNESS,
        WAKE_SCREEN,
    };

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
        let payload = SetBrightness::generate(&SetBrightness::new(30));

        let mut message_buffer = [0; 12];
        message_buffer[..11].copy_from_slice(&SET_BRIGHTNESS);

        message_buffer[11] = 30;

        assert_eq!(payload, create_output_buffer(&message_buffer))
    }

    #[test]
    fn correct_initiate_set_background_image_payload() {
        let payload =
            InitiateSetBackgroundImage::generate(&InitiateSetBackgroundImage::new(0x20u32));

        let mut message_buffer = [0; 14];
        message_buffer[..9].copy_from_slice(&INITIATE_SET_BACKGROUND_IMAGE);
        message_buffer[9] = 0x00;
        message_buffer[10] = 0x00;
        message_buffer[11] = 0x00;
        message_buffer[12] = 0x20;
        message_buffer[13] = 0x01;

        assert_eq!(payload, create_output_buffer(&message_buffer))
    }

    #[test]
    fn correct_initiate_set_display_zone_image_payload() {
        let payload = InitiateDisplayZoneImage::generate(&InitiateDisplayZoneImage::new(
            0x20u32,
            DisplayZones::Button7,
        ));

        let mut message_buffer = [0; 14];
        message_buffer[..9].copy_from_slice(&INITIATE_SET_DISPLAY_ZONE_IMAGE);
        message_buffer[9] = 0x00;
        message_buffer[10] = 0x00;
        message_buffer[11] = 0x00;
        message_buffer[12] = 0x20;
        message_buffer[13] = 0x07;

        assert_eq!(payload, create_output_buffer(&message_buffer))
    }
}
