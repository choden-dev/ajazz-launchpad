use crate::common::ByteArray;
use crate::inputs::input_buffer::BUFFER_SIZE_13;

#[derive(Debug, Clone)]
pub enum KnobActions {
    Knob1Clockwise,
    Knob2Clockwise,
    Knob3Clockwise,
    Knob4Clockwise,
    Knob1CounterClockwise,
    Knob2CounterClockwise,
    Knob3CounterClockwise,
    Knob4CounterClockwise,
}

pub const KNOB_1_CLOCKWISE: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\xA1\0\0\0";
pub const KNOB_2_CLOCKWISE: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x51\0\0\0";
pub const KNOB_3_CLOCKWISE: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x91\0\0\0";
pub const KNOB_4_CLOCKWISE: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x71\0\0\0";

pub const KNOB_1_COUNTER_CLOCKWISE: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\xA0\0\0\0";
pub const KNOB_2_COUNTER_CLOCKWISE: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x50\0\0\0";
pub const KNOB_3_COUNTER_CLOCKWISE: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x90\0\0\0";
pub const KNOB_4_COUNTER_CLOCKWISE: ByteArray<BUFFER_SIZE_13> = *b"ACK\0\0OK\0\0\x70\0\0\0";
