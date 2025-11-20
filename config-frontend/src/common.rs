#[derive(Debug, Clone, Default, PartialEq)]
pub enum ConfigurableZones {
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
    Touchscreen,
    Knob1,
    Knob2,
    Knob3,
    Knob4,
    #[default]
    None,
}
