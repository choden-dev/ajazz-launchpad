use crate::commands::payloads::Payload;
use crate::common::ByteArray;
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
        F: Fn(&ByteArray<N>) -> HidResult<usize>
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
        payload: payloads::SetBrightness { brightness },
    }
}
