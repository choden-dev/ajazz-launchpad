use crate::common::ByteArray;
use crate::inputs::input_buffer::BUFFER_SIZE_13;

///  ### Touchscreen Panel (4×1)
///  | touchscreen1 | touchscreen2 | touchscreen3 | touchscreen4 |
///  |--------------|--------------|--------------|--------------|
#[derive(Debug, Clone)]
pub enum TouchscreenAction {
    Zone1Pressed,
    Zone2Pressed,
    Zone3Pressed,
    Zone4Pressed,
    SwipedLeft,
    SwipedRight,
}

pub const ZONE_1_PRESSED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x40\0\0\0";
pub const ZONE_2_PRESSED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x41\0\0\0";
pub const ZONE_3_PRESSED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x42\0\0\0";
pub const ZONE_4_PRESSED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x43\0\0\0";
pub const SWIPED_LEFT: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x39\0\0\0";
pub const SWIPED_RIGHT: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x38\0\0\0";
