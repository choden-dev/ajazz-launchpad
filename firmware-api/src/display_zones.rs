use std::io::{Error, ErrorKind};

/// Used when trying to clear or set an image that was set on one of the
/// smaller display zones, this does **not** include the background image.
///
///  ### Main Button Panel (5×2)
///  | button1 | button2 | button3 | button4 | button5 |
///  |---------|---------|---------|---------|---------|
///  | button6 | button7 | button8 | button9 | button10|
///
///  ### Touchscreen Panel (4×1)
///  | touchscreen1 | touchscreen2 | touchscreen3 | touchscreen4 |
///  |--------------|--------------|--------------|--------------|
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayZones {
    Button1,
    Button2,
    Button3,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
    Button9,
    Button10,
    Touchscreen1,
    Touchscreen2,
    Touchscreen3,
    Touchscreen4,
}

impl From<DisplayZones> for u8 {
    fn from(display: DisplayZones) -> Self {
        match display {
            DisplayZones::Button1 => 11,
            DisplayZones::Button2 => 12,
            DisplayZones::Button3 => 13,
            DisplayZones::Button4 => 14,
            DisplayZones::Button5 => 15,
            DisplayZones::Button6 => 6,
            DisplayZones::Button7 => 7,
            DisplayZones::Button8 => 8,
            DisplayZones::Button9 => 9,
            DisplayZones::Button10 => 10,
            DisplayZones::Touchscreen1 => 1,
            DisplayZones::Touchscreen2 => 2,
            DisplayZones::Touchscreen3 => 3,
            DisplayZones::Touchscreen4 => 4,
        }
    }
}

impl TryFrom<u8> for DisplayZones {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let display_zone = match value {
            11 => DisplayZones::Button1,
            12 => DisplayZones::Button2,
            13 => DisplayZones::Button3,
            14 => DisplayZones::Button4,
            15 => DisplayZones::Button5,
            16 => DisplayZones::Button6,
            17 => DisplayZones::Button7,
            18 => DisplayZones::Button8,
            19 => DisplayZones::Button9,
            20 => DisplayZones::Button10,

            1 => DisplayZones::Touchscreen1,
            2 => DisplayZones::Touchscreen2,
            3 => DisplayZones::Touchscreen3,
            4 => DisplayZones::Touchscreen4,

            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                "not a recognised display zone",
            ))?,
        };

        Ok(display_zone)
    }
}
