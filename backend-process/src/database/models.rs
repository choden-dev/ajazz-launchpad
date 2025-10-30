use enigo::Key;
use firmware_api::inputs::InputActions;

/// The format we want to use inside the backend to handle actions
#[derive(Debug, PartialEq)]
pub struct InputMapping {
    input: InputActions,
    actions: Vec<Key>,
}

impl InputMapping {
    pub fn new(input: InputActions, actions: Vec<Key>) -> Self {
        Self { input, actions }
    }

    pub fn input(&self) -> InputActions {
        self.input.clone()
    }
    pub fn actions(&self) -> Vec<Key> {
        self.actions.clone()
    }
}
