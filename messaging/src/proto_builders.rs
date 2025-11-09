use crate::protos;

#[derive(Default)]
pub struct KeyConfigActionBuilder {
    actions: Vec<protos::key_config::Action>,
}
/// Used to handle creating the vector of Actions to turn into a protobuf
impl KeyConfigActionBuilder {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }
    /// Appends the given `key` to the current protobuf
    pub fn add_key_action(mut self, key: protos::keys::Key) -> Self {
        let action = protos::key_config::Action {
            action_data: Some(protos::key_config::action::Action_data::KeyAction(
                protos::key_config::KeyAction {
                    key: protobuf::EnumOrUnknown::from(key),
                    ..protos::key_config::KeyAction::default()
                },
            )),
            ..protos::key_config::Action::default()
        };
        self.actions.push(action);
        self
    }

    /// Vector of built protobuf actions
    pub fn actions(&self) -> &Vec<protos::key_config::Action> {
        &self.actions
    }
}
