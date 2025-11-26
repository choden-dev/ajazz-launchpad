use enigo::Key;
use firmware_api::display_zones::DisplayZones;
use firmware_api::inputs::InputActions;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Action {
    Key(Key),
    /// (command, args)
    Command(String, Vec<String>),
    Noop
}

/// The format we want to use inside the backend to handle actions
#[derive(Debug, PartialEq, Clone)]
pub struct InputMapping {
    input: InputActions,
    actions: Vec<Action>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImageMapping {
    pub display_zone: DisplayZones,
    pub image_path: String,
}

impl InputMapping {
    pub fn new(input: InputActions, actions: Vec<Action>) -> Self {
        Self { input, actions }
    }

    pub fn input(&self) -> InputActions {
        self.input.clone()
    }
    pub fn actions(&self) -> Vec<Action> {
        self.actions.clone()
    }
}
