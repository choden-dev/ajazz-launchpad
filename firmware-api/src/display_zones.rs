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

impl DisplayZones {
    const MAPPINGS: &'static [(DisplayZones, u8)] = &[
        (DisplayZones::Button1, 11),
        (DisplayZones::Button2, 12),
        (DisplayZones::Button3, 13),
        (DisplayZones::Button4, 14),
        (DisplayZones::Button5, 15),
        (DisplayZones::Button6, 6),
        (DisplayZones::Button7, 7),
        (DisplayZones::Button8, 8),
        (DisplayZones::Button9, 9),
        (DisplayZones::Button10, 10),
        (DisplayZones::Touchscreen1, 1),
        (DisplayZones::Touchscreen2, 2),
        (DisplayZones::Touchscreen3, 3),
        (DisplayZones::Touchscreen4, 4),
    ];
}

impl From<DisplayZones> for u8 {
    fn from(display: DisplayZones) -> Self {
        DisplayZones::MAPPINGS
            .iter()
            .find(|(zone, _)| *zone == display)
            .map(|(_, value)| *value)
            .unwrap() // Safe because all enum variants are in MAPPINGS
    }
}

impl TryFrom<u8> for DisplayZones {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        DisplayZones::MAPPINGS
            .iter()
            .find(|(_, v)| *v == value)
            .map(|(zone, _)| *zone)
            .ok_or("Unknown ZONES ID")
    }
}
