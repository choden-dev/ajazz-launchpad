use crate::common::ByteArray;

pub const WAKE_SCREEN: ByteArray<9> = *b"\0CRT\0\0DIS";

/// We need an extra parameter for the brightness (0-100)
pub const SET_BRIGHTNESS: ByteArray<11> = *b"\0CRT\0\0LIG\0\0";

pub const REFRESH: ByteArray<9> = *b"\0CRT\0\0STP";

/// Note that this is just a prefix for the other clear commands
/// You will need to provide a byte referring to the key to clear
pub const CLEAR_KEY: ByteArray<12> = *b"\0CRT\0\0CLE\0\0\0";

pub const CLEAR_ALL_IMAGES: ByteArray<13> = *b"\0CRT\0\0CLE\0\0\0\xFF";

pub const INITIATE_SET_BACKGROUND_IMAGE: ByteArray<9> = *b"\0CRT\0\0LOG";
