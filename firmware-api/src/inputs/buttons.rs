use crate::common::ByteArray;
use crate::inputs::input_buffer::BUFFER_SIZE_13;

///  ### Main Button Panel (5×2)
///  | button1 | button2 | button3 | button4 | button5 |
///  |---------|---------|---------|---------|---------|
///  | button6 | button7 | button8 | button9 | button10|
#[derive(Debug, Clone)]
pub enum ButtonActions {
    Button1Pressed,
    Button2Pressed,
    Button3Pressed,
    Button4Pressed,
    Button5Pressed,
    Button6Pressed,
    Button7Pressed,
    Button8Pressed,
    Button9Pressed,
    Button10Pressed,

    Button1Released,
    Button2Released,
    Button3Released,
    Button4Released,
    Button5Released,
    Button6Released,
    Button7Released,
    Button8Released,
    Button9Released,
    Button10Released,
}

pub const BUTTON_1_PRESSED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x01\x01\0\0";
pub const BUTTON_2_PRESSED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x02\x01\0\0";
pub const BUTTON_3_PRESSED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x03\x01\0\0";
pub const BUTTON_4_PRESSED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x04\x01\0\0";
pub const BUTTON_5_PRESSED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x05\x01\0\0";
pub const BUTTON_6_PRESSED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x06\x01\0\0";
pub const BUTTON_7_PRESSED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x07\x01\0\0";
pub const BUTTON_8_PRESSED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x08\x01\0\0";

pub const BUTTON_9_PRESSED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x09\x01\0\0";
pub const BUTTON_10_PRESSED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x0A\x01\0\0";

pub const BUTTON_1_RELEASED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x01\x00\0\0";
pub const BUTTON_2_RELEASED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x02\x00\0\0";
pub const BUTTON_3_RELEASED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x03\x00\0\0";
pub const BUTTON_4_RELEASED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x04\x00\0\0";
pub const BUTTON_5_RELEASED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x05\x00\0\0";
pub const BUTTON_6_RELEASED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x06\x00\0\0";
pub const BUTTON_7_RELEASED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x07\x00\0\0";
pub const BUTTON_8_RELEASED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x08\x00\0\0";
pub const BUTTON_9_RELEASED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x09\x00\0\0";
pub const BUTTON_10_RELEASED: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x0A\x00\0\0";
