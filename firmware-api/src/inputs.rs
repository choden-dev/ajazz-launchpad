use crate::common::ByteArray;
use crate::inputs::InputActions::Knob;
use crate::inputs::InputActions::{Button, Touchscreen};
use crate::inputs::buttons::ButtonActions;
use crate::inputs::input_buffer::BUFFER_SIZE_13;
use crate::inputs::knobs::KnobActions;
use crate::inputs::touchscreen::TouchscreenAction;

pub mod buttons;
pub mod input_buffer;
pub mod knobs;
pub mod touchscreen;

#[derive(Debug, Clone)]
pub enum InputActions {
    Button(ButtonActions),
    Knob(KnobActions),
    Touchscreen(TouchscreenAction),
    Unknown,
}
impl From<ByteArray<BUFFER_SIZE_13>> for InputActions {
    fn from(value: ByteArray<BUFFER_SIZE_13>) -> Self {
        match value {
            // Buttons Pressed
            buttons::BUTTON_1_PRESSED => Button(ButtonActions::Button1Pressed),
            buttons::BUTTON_2_PRESSED => Button(ButtonActions::Button2Pressed),
            buttons::BUTTON_3_PRESSED => Button(ButtonActions::Button3Pressed),
            buttons::BUTTON_4_PRESSED => Button(ButtonActions::Button4Pressed),
            buttons::BUTTON_5_PRESSED => Button(ButtonActions::Button5Pressed),
            buttons::BUTTON_6_PRESSED => Button(ButtonActions::Button6Pressed),
            buttons::BUTTON_7_PRESSED => Button(ButtonActions::Button7Pressed),
            buttons::BUTTON_8_PRESSED => Button(ButtonActions::Button8Pressed),
            buttons::BUTTON_9_PRESSED => Button(ButtonActions::Button9Pressed),
            buttons::BUTTON_10_PRESSED => Button(ButtonActions::Button10Pressed),

            // Buttons Pressed
            buttons::BUTTON_1_RELEASED => Button(ButtonActions::Button1Released),
            buttons::BUTTON_2_RELEASED => Button(ButtonActions::Button2Released),
            buttons::BUTTON_3_RELEASED => Button(ButtonActions::Button3Released),
            buttons::BUTTON_4_RELEASED => Button(ButtonActions::Button4Released),
            buttons::BUTTON_5_RELEASED => Button(ButtonActions::Button5Released),
            buttons::BUTTON_6_RELEASED => Button(ButtonActions::Button6Released),
            buttons::BUTTON_7_RELEASED => Button(ButtonActions::Button7Released),
            buttons::BUTTON_8_RELEASED => Button(ButtonActions::Button8Released),
            buttons::BUTTON_9_RELEASED => Button(ButtonActions::Button9Released),
            buttons::BUTTON_10_RELEASED => Button(ButtonActions::Button10Released),

            // Touchscreen pressed
            touchscreen::ZONE_1_PRESSED => Touchscreen(TouchscreenAction::Zone1Pressed),
            touchscreen::ZONE_2_PRESSED => Touchscreen(TouchscreenAction::Zone2Pressed),
            touchscreen::ZONE_3_PRESSED => Touchscreen(TouchscreenAction::Zone3Pressed),
            touchscreen::ZONE_4_PRESSED => Touchscreen(TouchscreenAction::Zone4Pressed),

            // Touchscreen swiped
            touchscreen::SWIPED_LEFT => Touchscreen(TouchscreenAction::SwipedLeft),
            touchscreen::SWIPED_RIGHT => Touchscreen(TouchscreenAction::SwipedRight),

            // Knob clockwise
            knobs::KNOB_1_CLOCKWISE => Knob(KnobActions::Knob1Clockwise),
            knobs::KNOB_2_CLOCKWISE => Knob(KnobActions::Knob2Clockwise),
            knobs::KNOB_3_CLOCKWISE => Knob(KnobActions::Knob3Clockwise),
            knobs::KNOB_4_CLOCKWISE => Knob(KnobActions::Knob4Clockwise),

            // Knob counter-clockwise
            knobs::KNOB_1_COUNTER_CLOCKWISE => Knob(KnobActions::Knob1CounterClockwise),
            knobs::KNOB_2_COUNTER_CLOCKWISE => Knob(KnobActions::Knob2CounterClockwise),
            knobs::KNOB_3_COUNTER_CLOCKWISE => Knob(KnobActions::Knob3CounterClockwise),
            knobs::KNOB_4_COUNTER_CLOCKWISE => Knob(KnobActions::Knob4CounterClockwise),

            // Knob pressed
            knobs::KNOB_1_PRESSED => Knob(KnobActions::Knob1Pressed),
            knobs::KNOB_2_PRESSED => Knob(KnobActions::Knob2Pressed),
            knobs::KNOB_3_PRESSED => Knob(KnobActions::Knob3Pressed),
            knobs::KNOB_4_PRESSED => Knob(KnobActions::Knob4Pressed),

            _ => InputActions::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_pressed_actions() {
        // Test all button pressed actions
        assert!(matches!(
            InputActions::from(buttons::BUTTON_1_PRESSED),
            Button(ButtonActions::Button1Pressed)
        ));
        assert!(matches!(
            InputActions::from(buttons::BUTTON_2_PRESSED),
            Button(ButtonActions::Button2Pressed)
        ));
        assert!(matches!(
            InputActions::from(buttons::BUTTON_3_PRESSED),
            Button(ButtonActions::Button3Pressed)
        ));
        assert!(matches!(
            InputActions::from(buttons::BUTTON_4_PRESSED),
            Button(ButtonActions::Button4Pressed)
        ));
        assert!(matches!(
            InputActions::from(buttons::BUTTON_5_PRESSED),
            Button(ButtonActions::Button5Pressed)
        ));
        assert!(matches!(
            InputActions::from(buttons::BUTTON_6_PRESSED),
            Button(ButtonActions::Button6Pressed)
        ));
        assert!(matches!(
            InputActions::from(buttons::BUTTON_7_PRESSED),
            Button(ButtonActions::Button7Pressed)
        ));
        assert!(matches!(
            InputActions::from(buttons::BUTTON_8_PRESSED),
            Button(ButtonActions::Button8Pressed)
        ));
        assert!(matches!(
            InputActions::from(buttons::BUTTON_9_PRESSED),
            Button(ButtonActions::Button9Pressed)
        ));
        assert!(matches!(
            InputActions::from(buttons::BUTTON_10_PRESSED),
            Button(ButtonActions::Button10Pressed)
        ));
    }

    #[test]
    fn test_button_released_actions() {
        // Test all button released actions
        assert!(matches!(
            InputActions::from(buttons::BUTTON_1_RELEASED),
            Button(ButtonActions::Button1Released)
        ));
        assert!(matches!(
            InputActions::from(buttons::BUTTON_2_RELEASED),
            Button(ButtonActions::Button2Released)
        ));
        assert!(matches!(
            InputActions::from(buttons::BUTTON_3_RELEASED),
            Button(ButtonActions::Button3Released)
        ));
        assert!(matches!(
            InputActions::from(buttons::BUTTON_4_RELEASED),
            Button(ButtonActions::Button4Released)
        ));
        assert!(matches!(
            InputActions::from(buttons::BUTTON_5_RELEASED),
            Button(ButtonActions::Button5Released)
        ));
        assert!(matches!(
            InputActions::from(buttons::BUTTON_6_RELEASED),
            Button(ButtonActions::Button6Released)
        ));
        assert!(matches!(
            InputActions::from(buttons::BUTTON_7_RELEASED),
            Button(ButtonActions::Button7Released)
        ));
        assert!(matches!(
            InputActions::from(buttons::BUTTON_8_RELEASED),
            Button(ButtonActions::Button8Released)
        ));
        assert!(matches!(
            InputActions::from(buttons::BUTTON_9_RELEASED),
            Button(ButtonActions::Button9Released)
        ));
        assert!(matches!(
            InputActions::from(buttons::BUTTON_10_RELEASED),
            Button(ButtonActions::Button10Released)
        ));
    }

    #[test]
    fn test_touchscreen_pressed_actions() {
        // Test all touchscreen zone pressed actions
        assert!(matches!(
            InputActions::from(touchscreen::ZONE_1_PRESSED),
            Touchscreen(TouchscreenAction::Zone1Pressed)
        ));
        assert!(matches!(
            InputActions::from(touchscreen::ZONE_2_PRESSED),
            Touchscreen(TouchscreenAction::Zone2Pressed)
        ));
        assert!(matches!(
            InputActions::from(touchscreen::ZONE_3_PRESSED),
            Touchscreen(TouchscreenAction::Zone3Pressed)
        ));
        assert!(matches!(
            InputActions::from(touchscreen::ZONE_4_PRESSED),
            Touchscreen(TouchscreenAction::Zone4Pressed)
        ));
    }

    #[test]
    fn test_touchscreen_swipe_actions() {
        // Test touchscreen swipe actions
        assert!(matches!(
            InputActions::from(touchscreen::SWIPED_LEFT),
            Touchscreen(TouchscreenAction::SwipedLeft)
        ));
        assert!(matches!(
            InputActions::from(touchscreen::SWIPED_RIGHT),
            Touchscreen(TouchscreenAction::SwipedRight)
        ));
    }

    #[test]
    fn test_knob_clockwise_actions() {
        // Test all knob clockwise actions
        assert!(matches!(
            InputActions::from(knobs::KNOB_1_CLOCKWISE),
            Knob(KnobActions::Knob1Clockwise)
        ));
        assert!(matches!(
            InputActions::from(knobs::KNOB_2_CLOCKWISE),
            Knob(KnobActions::Knob2Clockwise)
        ));
        assert!(matches!(
            InputActions::from(knobs::KNOB_3_CLOCKWISE),
            Knob(KnobActions::Knob3Clockwise)
        ));
        assert!(matches!(
            InputActions::from(knobs::KNOB_4_CLOCKWISE),
            Knob(KnobActions::Knob4Clockwise)
        ));
    }

    #[test]
    fn test_knob_counter_clockwise_actions() {
        // Test all knob counter-clockwise actions
        assert!(matches!(
            InputActions::from(knobs::KNOB_1_COUNTER_CLOCKWISE),
            Knob(KnobActions::Knob1CounterClockwise)
        ));
        assert!(matches!(
            InputActions::from(knobs::KNOB_2_COUNTER_CLOCKWISE),
            Knob(KnobActions::Knob2CounterClockwise)
        ));
        assert!(matches!(
            InputActions::from(knobs::KNOB_3_COUNTER_CLOCKWISE),
            Knob(KnobActions::Knob3CounterClockwise)
        ));
        assert!(matches!(
            InputActions::from(knobs::KNOB_4_COUNTER_CLOCKWISE),
            Knob(KnobActions::Knob4CounterClockwise)
        ));
    }

    #[test]
    fn test_knob_pressed_actions() {
        assert!(matches!(
            InputActions::from(knobs::KNOB_1_PRESSED),
            Knob(KnobActions::Knob1Pressed)
        ));
        assert!(matches!(
            InputActions::from(knobs::KNOB_2_PRESSED),
            Knob(KnobActions::Knob2Pressed)
        ));
        assert!(matches!(
            InputActions::from(knobs::KNOB_3_PRESSED),
            Knob(KnobActions::Knob3Pressed)
        ));
        assert!(matches!(
            InputActions::from(knobs::KNOB_4_PRESSED),
            Knob(KnobActions::Knob4Pressed)
        ));
    }

    #[test]
    fn test_unknown_action() {
        // Test that unknown byte arrays return Unknown
        let unknown_bytes = [0xFF; BUFFER_SIZE_13];
        assert!(matches!(
            InputActions::from(unknown_bytes),
            InputActions::Unknown
        ));

        // Test with all zeros
        let zero_bytes = [0x00; BUFFER_SIZE_13];
        assert!(matches!(
            InputActions::from(zero_bytes),
            InputActions::Unknown
        ));
    }
}
