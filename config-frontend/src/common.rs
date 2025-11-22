#[derive(Debug, Clone, Default, PartialEq)]
pub enum ConfigurableZones {
    Button1(ButtonInput),
    Button2(ButtonInput),
    Button3(ButtonInput),
    Button4(ButtonInput),
    Button5(ButtonInput),
    Button6(ButtonInput),
    Button7(ButtonInput),
    Button8(ButtonInput),
    Button9(ButtonInput),
    Button10(ButtonInput),
    Touchscreen1(TouchscreenZoneInput),
    Touchscreen2(TouchscreenZoneInput),
    Touchscreen3(TouchscreenZoneInput),
    Touchscreen4(TouchscreenZoneInput),
    TouchscreenExtra(TouchscreenInput),
    Knob1(KnobInput),
    Knob2(KnobInput),
    Knob3(KnobInput),
    Knob4(KnobInput),
    #[default]
    None,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum ButtonInput {
    Pressed,
    Released,
    #[default]
    None,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum KnobInput {
    Pressed,
    Clockwise,
    CounterClockwise,
    #[default]
    None,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum TouchscreenZoneInput {
    Pressed,
    #[default]
    None,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum TouchscreenInput {
    SwipeLeft,
    SwipeRight,
    #[default]
    None,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum ExtraConfigMode {
    #[default]
    Default,
    KeyRecording,
}
