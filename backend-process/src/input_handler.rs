use enigo::{Enigo, Key, Keyboard};
use firmware_api::device::InputHandler;
use firmware_api::inputs::InputActions;
use firmware_api::inputs::buttons::ButtonActions;
use firmware_api::inputs::buttons::ButtonActions::Button1Pressed;
use firmware_api::inputs::knobs::KnobActions;
use firmware_api::inputs::touchscreen::TouchscreenAction;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::database::models;

pub trait KeyActionExecutor {
    fn execute(&self, actions: &[Key]) -> Result<(), String>;
}

pub struct EnigoKeyActionHandler {
    enigo: Mutex<Enigo>,
}

impl Default for EnigoKeyActionHandler {
    fn default() -> Self {
        Self {
            enigo: Mutex::new(Enigo::new(&enigo::Settings::default()).unwrap()),
        }
    }
}

/// Used by the application to access the current set of input mappings in-memory,
/// This should be the object queried when handling input to avoid database queries.
#[derive(Clone)]
pub struct InputMapping(HashMap<InputActions, Vec<Key>>);

impl InputMapping {
    /// Used to set new configs for the keys
    /// * `new_actions`: The new set of `InputMapping`s which will overwrite any existing mappings
    pub fn override_config(&mut self, new_actions: InputMapping) {
        self.0.extend(new_actions.0);
    }
}

impl Default for InputMapping {
    fn default() -> Self {
        Self(HashMap::from([(
            InputActions::Button(Button1Pressed),
            vec![Key::VolumeDown],
        )]))
    }
}
impl From<Vec<models::InputMapping>> for InputMapping {
    fn from(value: Vec<models::InputMapping>) -> Self {
        Self(
            value
                .into_iter()
                .map(|mapping| (mapping.input(), mapping.actions().to_vec()))
                .collect(),
        )
    }
}
impl From<models::InputMapping> for InputMapping {
    fn from(value: models::InputMapping) -> Self {
        Self(
            vec![value]
                .into_iter()
                .map(|mapping| (mapping.input(), mapping.actions().to_vec()))
                .collect(),
        )
    }
}
impl KeyActionExecutor for EnigoKeyActionHandler {
    fn execute(&self, actions: &[Key]) -> Result<(), String> {
        let mut lock = self.enigo.lock().map_err(|e| e.to_string())?;
        let _: () = actions.iter().for_each(|action| {
            lock.key(*action, enigo::Direction::Click).ok();
        });
        Ok(())
    }
}

pub struct LaunchpadInputHandler<'a> {
    input_mapping: InputMapping,
    key_action_executor: &'a Box<dyn KeyActionExecutor>,
}

impl<'a> LaunchpadInputHandler<'a> {
    pub fn new(mapping: InputMapping, key_action_executor: &'a Box<dyn KeyActionExecutor>) -> Self {
        Self {
            input_mapping: mapping,
            key_action_executor,
        }
    }

    pub fn new_updated_mappings(&self, new_mapping: InputMapping) -> InputMapping {
        let mut new_created_mappings = self.input_mapping.clone();
        new_created_mappings.override_config(new_mapping);

        new_created_mappings
    }

    fn execute_keys(&self, input_action: InputActions) {
        if let Some(actions) = self.input_mapping.0.get(&input_action) {
            self.key_action_executor.execute(actions).ok();
        }
    }

    fn handle_touchscreen(&self, touchscreen_action: TouchscreenAction) {
        self.execute_keys(InputActions::Touchscreen(touchscreen_action));
    }

    fn handle_button(&self, button_action: ButtonActions) {
        self.execute_keys(InputActions::Button(button_action));
    }

    fn handle_knob(&self, knob_action: KnobActions) {
        self.execute_keys(InputActions::Knob(knob_action));
    }
}
impl InputHandler for LaunchpadInputHandler<'_> {
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
