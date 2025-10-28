use firmware_api::device::InputHandler;
use firmware_api::inputs::InputActions;
use firmware_api::inputs::buttons::ButtonActions;
use firmware_api::inputs::knobs::KnobActions;
use firmware_api::inputs::touchscreen::TouchscreenAction;

pub struct LaunchpadInputHandler;

impl LaunchpadInputHandler {
    fn handle_touchscreen(&self, touchscreen_action: TouchscreenAction) {
        todo!("{:?}", touchscreen_action)
    }
    fn handle_button(&self, button_action: ButtonActions) {
        todo!("{:?}", button_action)
    }
    fn handle_knob(&self, knob_action: KnobActions) {
        todo!("{:?}", knob_action)
    }
}
impl InputHandler for LaunchpadInputHandler {
    fn handle(&self, action: InputActions) {
        match action {
            InputActions::Touchscreen(touchscreen_action) => {
                self.handle_touchscreen(touchscreen_action);
            }
            InputActions::Knob(knob_action) => self.handle_knob(knob_action),
            InputActions::Button(button_action) => self.handle_button(button_action),
            InputActions::Unknown => (),
        }
    }
}
