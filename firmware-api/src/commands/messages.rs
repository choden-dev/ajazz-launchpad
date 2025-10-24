use crate::common::ByteArray;

pub const WAKE_SCREEN: ByteArray<9> = *b"\0CRT\0\0DIS";

pub const SET_BRIGHTNESS: ByteArray<11> = *b"\0CRT\0\0LIG\0\0";
// We need an extra parameter for the brightness (0-100)

pub const REFRESH: ByteArray<9> = *b"\0CRT\0\0STP";
