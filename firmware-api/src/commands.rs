use crate::commands::payloads::Payload;
use crate::common::{ByteArray, IMAGE_DATA_PACKET_LENGTH};
use crate::display_zones::DisplayZones;
use hidapi::HidResult;

pub mod messages;
pub mod output_buffer;
mod payloads;

pub trait Command<const N: usize, R> {
    fn execute<F>(&self, write_callback: F) -> R
    where
        F: Fn(&ByteArray<N>) -> R;
}

// Keep the generic implementation internal
struct PayloadCommand<const N: usize, P: Payload<N>> {
    payload: P,
}

impl<const N: usize, P: Payload<N>> Command<N, HidResult<usize>> for PayloadCommand<N, P> {
    fn execute<F>(&self, write_callback: F) -> HidResult<usize>
    where
        F: Fn(&ByteArray<N>) -> HidResult<usize>,
    {
        write_callback(&self.payload.generate())
    }
}

pub fn wake_screen_command_factory()
-> impl Command<{ output_buffer::BUFFER_SIZE_513 }, HidResult<usize>> {
    PayloadCommand {
        payload: payloads::WakeScreen,
    }
}

pub fn refresh_command_factory()
-> impl Command<{ output_buffer::BUFFER_SIZE_513 }, HidResult<usize>> {
    PayloadCommand {
        payload: payloads::Refresh,
    }
}

pub fn set_brightness_command_factory(
    brightness: u8,
) -> impl Command<{ output_buffer::BUFFER_SIZE_513 }, HidResult<usize>> {
    PayloadCommand {
        payload: payloads::SetBrightness::new(brightness),
    }
}

pub fn clear_all_images_command_factory()
-> impl Command<{ output_buffer::BUFFER_SIZE_513 }, HidResult<usize>> {
    PayloadCommand {
        payload: payloads::ClearAllImages,
    }
}

pub fn initiate_set_background_command_factory(
    image_size_bytes: u32,
) -> impl Command<{ output_buffer::BUFFER_SIZE_1025 }, HidResult<usize>> {
    PayloadCommand {
        payload: payloads::InitiateSetBackgroundImage::new(image_size_bytes),
    }
}

pub fn initiate_set_display_zone_image_command_factory(
    image_size_bytes: u32,
    display_zone: DisplayZones,
) -> impl Command<{ output_buffer::BUFFER_SIZE_1025 }, HidResult<usize>> {
    PayloadCommand {
        payload: payloads::InitiateDisplayZoneImage::new(image_size_bytes, display_zone),
    }
}

pub fn send_image_data_packet_command_factory(
    packet: ByteArray<IMAGE_DATA_PACKET_LENGTH>,
) -> impl Command<{ output_buffer::BUFFER_SIZE_1025 }, HidResult<usize>> {
    PayloadCommand {
        payload: payloads::SendImageDataPacket::new(packet),
    }
}

pub fn clear_display_zone_image_command_factory(
    display_zone: DisplayZones,
) -> impl Command<{ output_buffer::BUFFER_SIZE_513 }, HidResult<usize>> {
    PayloadCommand {
        payload: payloads::ClearDisplayZoneImage::new(display_zone),
    }
}
