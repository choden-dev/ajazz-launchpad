use crate::database::models::{Action, ImageMapping, InputMapping};
use enigo::Key;
use firmware_api::display_zones::DisplayZones;
use firmware_api::inputs::InputActions;
use firmware_api::inputs::buttons::ButtonActions;
use firmware_api::inputs::knobs::KnobActions;
use firmware_api::inputs::touchscreen::TouchscreenAction;
use messaging::protos;
use protobuf::EnumOrUnknown;
use std::char;
use std::io::{Error, ErrorKind};

/// Util struct for mapping the protobuf key into an `Enigo` key
#[derive(Debug, PartialEq)]
pub struct KeyWrapper(Key);
impl KeyWrapper {
    // Single source of truth for all mappings
    fn get_mappings() -> &'static [(protos::keys::Key, Key)] {
        &[
            (protos::keys::Key::KEY_ADD, Key::Add),
            (protos::keys::Key::KEY_ALT, Key::Alt),
            (protos::keys::Key::KEY_BACKSPACE, Key::Backspace),
            (protos::keys::Key::KEY_CAPS_LOCK, Key::CapsLock),
            (protos::keys::Key::KEY_CONTROL, Key::Control),
            (protos::keys::Key::KEY_DECIMAL, Key::Decimal),
            (protos::keys::Key::KEY_DELETE, Key::Delete),
            (protos::keys::Key::KEY_DIVIDE, Key::Divide),
            (protos::keys::Key::KEY_DOWN_ARROW, Key::DownArrow),
            (protos::keys::Key::KEY_END, Key::End),
            (protos::keys::Key::KEY_ESCAPE, Key::Escape),
            (protos::keys::Key::KEY_F1, Key::F1),
            (protos::keys::Key::KEY_F2, Key::F2),
            (protos::keys::Key::KEY_F3, Key::F3),
            (protos::keys::Key::KEY_F4, Key::F4),
            (protos::keys::Key::KEY_F5, Key::F5),
            (protos::keys::Key::KEY_F6, Key::F6),
            (protos::keys::Key::KEY_F7, Key::F7),
            (protos::keys::Key::KEY_F8, Key::F8),
            (protos::keys::Key::KEY_F9, Key::F9),
            (protos::keys::Key::KEY_F10, Key::F10),
            (protos::keys::Key::KEY_F11, Key::F11),
            (protos::keys::Key::KEY_F12, Key::F12),
            (protos::keys::Key::KEY_F13, Key::F13),
            (protos::keys::Key::KEY_F14, Key::F14),
            (protos::keys::Key::KEY_F15, Key::F15),
            (protos::keys::Key::KEY_F16, Key::F16),
            (protos::keys::Key::KEY_F17, Key::F17),
            (protos::keys::Key::KEY_F18, Key::F18),
            (protos::keys::Key::KEY_F19, Key::F19),
            (protos::keys::Key::KEY_F20, Key::F20),
            (protos::keys::Key::KEY_HELP, Key::Help),
            (protos::keys::Key::KEY_HOME, Key::Home),
            (protos::keys::Key::KEY_L_CONTROL, Key::LControl),
            (protos::keys::Key::KEY_LEFT_ARROW, Key::LeftArrow),
            (protos::keys::Key::KEY_L_SHIFT, Key::LShift),
            (protos::keys::Key::KEY_MEDIA_NEXT_TRACK, Key::MediaNextTrack),
            (protos::keys::Key::KEY_MEDIA_PLAY_PAUSE, Key::MediaPlayPause),
            (protos::keys::Key::KEY_MEDIA_PREV_TRACK, Key::MediaPrevTrack),
            (protos::keys::Key::KEY_META, Key::Meta),
            (protos::keys::Key::KEY_MULTIPLY, Key::Multiply),
            (protos::keys::Key::KEY_NUMPAD0, Key::Numpad0),
            (protos::keys::Key::KEY_NUMPAD1, Key::Numpad1),
            (protos::keys::Key::KEY_NUMPAD2, Key::Numpad2),
            (protos::keys::Key::KEY_NUMPAD3, Key::Numpad3),
            (protos::keys::Key::KEY_NUMPAD4, Key::Numpad4),
            (protos::keys::Key::KEY_NUMPAD5, Key::Numpad5),
            (protos::keys::Key::KEY_NUMPAD6, Key::Numpad6),
            (protos::keys::Key::KEY_NUMPAD7, Key::Numpad7),
            (protos::keys::Key::KEY_NUMPAD8, Key::Numpad8),
            (protos::keys::Key::KEY_NUMPAD9, Key::Numpad9),
            (protos::keys::Key::KEY_OPTION, Key::Option),
            (protos::keys::Key::KEY_PAGE_DOWN, Key::PageDown),
            (protos::keys::Key::KEY_PAGE_UP, Key::PageUp),
            (protos::keys::Key::KEY_R_CONTROL, Key::RControl),
            (protos::keys::Key::KEY_RETURN, Key::Return),
            (protos::keys::Key::KEY_RIGHT_ARROW, Key::RightArrow),
            (protos::keys::Key::KEY_R_SHIFT, Key::RShift),
            (protos::keys::Key::KEY_SHIFT, Key::Shift),
            (protos::keys::Key::KEY_SPACE, Key::Space),
            (protos::keys::Key::KEY_SUBTRACT, Key::Subtract),
            (protos::keys::Key::KEY_TAB, Key::Tab),
            (protos::keys::Key::KEY_UP_ARROW, Key::UpArrow),
            (protos::keys::Key::KEY_VOLUME_DOWN, Key::VolumeDown),
            (protos::keys::Key::KEY_VOLUME_MUTE, Key::VolumeMute),
            (protos::keys::Key::KEY_VOLUME_UP, Key::VolumeUp),
        ]
    }
}

#[derive(Debug, PartialEq)]
pub struct DisplayZoneWrapper(DisplayZones);

/// Util struct to convert from the protobuf format to our application model
#[derive(Debug, PartialEq)]
pub struct InputActionWrapper(InputActions);

impl InputActionWrapper {
    fn get_mappings() -> &'static [(protos::inputs::InputId, InputActions)] {
        &[
            (
                protos::inputs::InputId::INPUT_ACTION_UNSPECIFIED,
                InputActions::Unknown,
            ),
            (
                protos::inputs::InputId::BUTTON_1_PRESSED,
                InputActions::Button(ButtonActions::Button1Pressed),
            ),
            (
                protos::inputs::InputId::BUTTON_2_PRESSED,
                InputActions::Button(ButtonActions::Button2Pressed),
            ),
            (
                protos::inputs::InputId::BUTTON_3_PRESSED,
                InputActions::Button(ButtonActions::Button3Pressed),
            ),
            (
                protos::inputs::InputId::BUTTON_4_PRESSED,
                InputActions::Button(ButtonActions::Button4Pressed),
            ),
            (
                protos::inputs::InputId::BUTTON_5_PRESSED,
                InputActions::Button(ButtonActions::Button5Pressed),
            ),
            (
                protos::inputs::InputId::BUTTON_6_PRESSED,
                InputActions::Button(ButtonActions::Button6Pressed),
            ),
            (
                protos::inputs::InputId::BUTTON_7_PRESSED,
                InputActions::Button(ButtonActions::Button7Pressed),
            ),
            (
                protos::inputs::InputId::BUTTON_8_PRESSED,
                InputActions::Button(ButtonActions::Button8Pressed),
            ),
            (
                protos::inputs::InputId::BUTTON_9_PRESSED,
                InputActions::Button(ButtonActions::Button9Pressed),
            ),
            (
                protos::inputs::InputId::BUTTON_10_PRESSED,
                InputActions::Button(ButtonActions::Button10Pressed),
            ),
            (
                protos::inputs::InputId::BUTTON_1_RELEASED,
                InputActions::Button(ButtonActions::Button1Released),
            ),
            (
                protos::inputs::InputId::BUTTON_2_RELEASED,
                InputActions::Button(ButtonActions::Button2Released),
            ),
            (
                protos::inputs::InputId::BUTTON_3_RELEASED,
                InputActions::Button(ButtonActions::Button3Released),
            ),
            (
                protos::inputs::InputId::BUTTON_4_RELEASED,
                InputActions::Button(ButtonActions::Button4Released),
            ),
            (
                protos::inputs::InputId::BUTTON_5_RELEASED,
                InputActions::Button(ButtonActions::Button5Released),
            ),
            (
                protos::inputs::InputId::BUTTON_6_RELEASED,
                InputActions::Button(ButtonActions::Button6Released),
            ),
            (
                protos::inputs::InputId::BUTTON_7_RELEASED,
                InputActions::Button(ButtonActions::Button7Released),
            ),
            (
                protos::inputs::InputId::BUTTON_8_RELEASED,
                InputActions::Button(ButtonActions::Button8Released),
            ),
            (
                protos::inputs::InputId::BUTTON_9_RELEASED,
                InputActions::Button(ButtonActions::Button9Released),
            ),
            (
                protos::inputs::InputId::BUTTON_10_RELEASED,
                InputActions::Button(ButtonActions::Button10Released),
            ),
            (
                protos::inputs::InputId::KNOB_1_CLOCKWISE,
                InputActions::Knob(KnobActions::Knob1Clockwise),
            ),
            (
                protos::inputs::InputId::KNOB_2_CLOCKWISE,
                InputActions::Knob(KnobActions::Knob2Clockwise),
            ),
            (
                protos::inputs::InputId::KNOB_3_CLOCKWISE,
                InputActions::Knob(KnobActions::Knob3Clockwise),
            ),
            (
                protos::inputs::InputId::KNOB_4_CLOCKWISE,
                InputActions::Knob(KnobActions::Knob4Clockwise),
            ),
            (
                protos::inputs::InputId::KNOB_1_COUNTER_CLOCKWISE,
                InputActions::Knob(KnobActions::Knob1CounterClockwise),
            ),
            (
                protos::inputs::InputId::KNOB_2_COUNTER_CLOCKWISE,
                InputActions::Knob(KnobActions::Knob2CounterClockwise),
            ),
            (
                protos::inputs::InputId::KNOB_3_COUNTER_CLOCKWISE,
                InputActions::Knob(KnobActions::Knob3CounterClockwise),
            ),
            (
                protos::inputs::InputId::KNOB_4_COUNTER_CLOCKWISE,
                InputActions::Knob(KnobActions::Knob4CounterClockwise),
            ),
            (
                protos::inputs::InputId::KNOB_1_PRESSED,
                InputActions::Knob(KnobActions::Knob1Pressed),
            ),
            (
                protos::inputs::InputId::KNOB_2_PRESSED,
                InputActions::Knob(KnobActions::Knob2Pressed),
            ),
            (
                protos::inputs::InputId::KNOB_3_PRESSED,
                InputActions::Knob(KnobActions::Knob3Pressed),
            ),
            (
                protos::inputs::InputId::KNOB_4_PRESSED,
                InputActions::Knob(KnobActions::Knob4Pressed),
            ),
            (
                protos::inputs::InputId::TOUCHSCREEN_ZONE_1_PRESSED,
                InputActions::Touchscreen(TouchscreenAction::Zone1Pressed),
            ),
            (
                protos::inputs::InputId::TOUCHSCREEN_ZONE_2_PRESSED,
                InputActions::Touchscreen(TouchscreenAction::Zone2Pressed),
            ),
            (
                protos::inputs::InputId::TOUCHSCREEN_ZONE_3_PRESSED,
                InputActions::Touchscreen(TouchscreenAction::Zone3Pressed),
            ),
            (
                protos::inputs::InputId::TOUCHSCREEN_ZONE_4_PRESSED,
                InputActions::Touchscreen(TouchscreenAction::Zone4Pressed),
            ),
            (
                protos::inputs::InputId::TOUCHSCREEN_SWIPED_LEFT,
                InputActions::Touchscreen(TouchscreenAction::SwipedLeft),
            ),
            (
                protos::inputs::InputId::TOUCHSCREEN_SWIPED_RIGHT,
                InputActions::Touchscreen(TouchscreenAction::SwipedRight),
            ),
        ]
    }
}

impl From<InputActionWrapper> for protos::inputs::InputId {
    fn from(wrapper: InputActionWrapper) -> Self {
        InputActionWrapper::get_mappings()
            .iter()
            .find(|(_, action)| *action == wrapper.0)
            .map(|(input_id, _)| *input_id)
            .unwrap_or(protos::inputs::InputId::INPUT_ACTION_UNSPECIFIED)
    }
}

impl From<protos::inputs::InputId> for InputActionWrapper {
    fn from(value: protos::inputs::InputId) -> InputActionWrapper {
        let action = Self::get_mappings()
            .iter()
            .find(|(input_id, _)| *input_id == value)
            .map(|(_, action)| action.clone())
            .unwrap_or(InputActions::Unknown);

        InputActionWrapper(action)
    }
}

impl From<InputMapping> for protos::key_config::KeyConfig {
    fn from(input: InputMapping) -> Self {
        protos::key_config::KeyConfig {
            input_id: EnumOrUnknown::new(protos::inputs::InputId::from(InputActionWrapper(
                input.input(),
            ))),
            actions: input
                .actions()
                .iter()
                .map(|action| protos::key_config::Action::from(action.clone()))
                .collect(),
            ..protos::key_config::KeyConfig::default()
        }
    }
}

impl From<Action> for protos::key_config::Action {
    fn from(action: Action) -> Self {
        protos::key_config::Action {
            action_data: match action {
                Action::Command(command, args) => {
                    Some(protos::key_config::action::Action_data::CommandAction(
                        protos::key_config::CommandAction {
                            command: Some(
                                protos::key_config::command_action::Command::FreeformCommand(
                                    protos::key_config::FreeformCommand {
                                        command,
                                        args,
                                        ..protos::key_config::FreeformCommand::default()
                                    },
                                ),
                            ),
                            ..protos::key_config::CommandAction::default()
                        },
                    ))
                }
                Action::Key(key, modifiers) => {
                    Some(protos::key_config::action::Action_data::KeyAction(
                        protos::key_config::KeyAction {
                            key: EnumOrUnknown::new(protos::keys::Key::from(KeyWrapper(key))),
                            modifier: modifiers
                                .iter()
                                .map(|m| {
                                    EnumOrUnknown::new(protos::keys::Key::from(KeyWrapper(*m)))
                                })
                                .collect(),
                            unicode: match key {
                                Key::Unicode(char) => Some(u32::from(char)),
                                _ => None,
                            },
                            ..protos::key_config::KeyAction::default()
                        },
                    ))
                }
                Action::Noop => None,
            },
            ..protos::key_config::Action::default()
        }
    }
}

impl From<ImageMapping> for protos::server_config::DisplayImage {
    fn from(mapping: ImageMapping) -> Self {
        protos::server_config::DisplayImage {
            display_zone: EnumOrUnknown::new(protos::display_zones::DisplayZone::from(
                DisplayZoneWrapper(mapping.display_zone),
            )),
            path: mapping.image_path,
            ..protos::server_config::DisplayImage::default()
        }
    }
}

impl TryFrom<protos::key_config::KeyConfig> for InputMapping {
    type Error = ();

    fn try_from(value: protos::key_config::KeyConfig) -> Result<Self, Self::Error> {
        let input_id: InputActionWrapper = value.input_id.enum_value().unwrap().into();
        let actions = value
            .actions
            .iter()
            .map(|a| {
                match a.clone().action_data {
                    Some(item) => match item {
                        protos::key_config::action::Action_data::CommandAction(command) => {
                            if let Some(command_type) = command.command {
                                return match command_type {
                                    protos::key_config::command_action::Command::FreeformCommand(command) => {
                                        Action::Command(command.command, command.args)
                                    }
                                    protos::key_config::command_action::Command::OpenAppCommand(command) => {
                                        Action::Command(
                                            String::from("Open command"),
                                            vec![command.app_path],
                                        )
                                    }
                                    _ => Action::Noop,
                                };
                            }
                            Action::Noop
                        }
                        protos::key_config::action::Action_data::KeyAction(key) => {
                            if let Ok(key_val) = KeyWrapper::try_from(key.clone()) {
                                return Action::Key(
                                    key_val.0,
                                    key.modifier
                                        .iter()
                                        .filter_map(|modifier_key| {
                                            match modifier_key.enum_value() {
                                                Ok(modifier_key_value) => {
                                                    Some(KeyWrapper::from(modifier_key_value).0)
                                                }
                                                Err(_) => None,
                                            }
                                        })
                                        .collect::<Vec<Key>>(),
                                );
                            }
                            // Stub out invalid values
                            Action::Noop
                        }
                        _ => Action::Noop,
                    },
                    _ => Action::Noop,
                }
            })
            .collect::<Vec<Action>>();

        Ok(InputMapping::new(input_id.0, actions))
    }
}

impl TryFrom<protos::display_zone_image::SetDisplayZoneImage> for ImageMapping {
    type Error = Error;
    fn try_from(
        value: protos::display_zone_image::SetDisplayZoneImage,
    ) -> Result<Self, Self::Error> {
        let display_zone_id: DisplayZoneWrapper =
            value.display_zone.enum_value().unwrap().try_into()?;

        Ok(ImageMapping {
            display_zone: display_zone_id.0,
            image_path: value.image_path.clone(),
        })
    }
}

impl DisplayZoneWrapper {
    pub fn display_zone(&self) -> DisplayZones {
        self.0
    }
}

impl DisplayZoneWrapper {
    fn get_mappings() -> &'static [(protos::display_zones::DisplayZone, DisplayZones)] {
        &[
            (
                protos::display_zones::DisplayZone::BUTTON_1,
                DisplayZones::Button1,
            ),
            (
                protos::display_zones::DisplayZone::BUTTON_2,
                DisplayZones::Button2,
            ),
            (
                protos::display_zones::DisplayZone::BUTTON_3,
                DisplayZones::Button3,
            ),
            (
                protos::display_zones::DisplayZone::BUTTON_4,
                DisplayZones::Button4,
            ),
            (
                protos::display_zones::DisplayZone::BUTTON_5,
                DisplayZones::Button5,
            ),
            (
                protos::display_zones::DisplayZone::BUTTON_6,
                DisplayZones::Button6,
            ),
            (
                protos::display_zones::DisplayZone::BUTTON_7,
                DisplayZones::Button7,
            ),
            (
                protos::display_zones::DisplayZone::BUTTON_8,
                DisplayZones::Button8,
            ),
            (
                protos::display_zones::DisplayZone::BUTTON_9,
                DisplayZones::Button9,
            ),
            (
                protos::display_zones::DisplayZone::BUTTON_10,
                DisplayZones::Button10,
            ),
            (
                protos::display_zones::DisplayZone::TOUCHSCREEN_ZONE_1,
                DisplayZones::Touchscreen1,
            ),
            (
                protos::display_zones::DisplayZone::TOUCHSCREEN_ZONE_2,
                DisplayZones::Touchscreen2,
            ),
            (
                protos::display_zones::DisplayZone::TOUCHSCREEN_ZONE_3,
                DisplayZones::Touchscreen3,
            ),
            (
                protos::display_zones::DisplayZone::TOUCHSCREEN_ZONE_4,
                DisplayZones::Touchscreen4,
            ),
        ]
    }
}

impl From<DisplayZoneWrapper> for protos::display_zones::DisplayZone {
    fn from(value: DisplayZoneWrapper) -> Self {
        DisplayZoneWrapper::get_mappings()
            .iter()
            .find(|(_, zone)| *zone == value.display_zone())
            .map(|(proto_zone, _)| *proto_zone)
            .unwrap_or(protos::display_zones::DisplayZone::DISPLAY_ZONE_UNSPECIFIED) // or whatever default you prefer
    }
}

impl TryFrom<protos::display_zones::DisplayZone> for DisplayZoneWrapper {
    type Error = Error;

    fn try_from(value: protos::display_zones::DisplayZone) -> Result<Self, Self::Error> {
        Self::get_mappings()
            .iter()
            .find(|(proto_zone, _)| *proto_zone == value)
            .map(|(_, zone)| DisplayZoneWrapper(*zone))
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Not a valid display zone"))
    }
}

impl From<protos::keys::Key> for KeyWrapper {
    fn from(value: protos::keys::Key) -> Self {
        if let Some((_, key)) = Self::get_mappings()
            .iter()
            .find(|(proto_key, _)| *proto_key == value)
        {
            return KeyWrapper(*key);
        }
        match value {
            protos::keys::Key::KEY_UNICODE => KeyWrapper(Key::Unicode(char::default())),
            protos::keys::Key::KEY_OTHER => KeyWrapper(Key::Other(u32::default())),
            _ => KeyWrapper(Key::Other(u32::default())),
        }
    }
}
impl From<KeyWrapper> for protos::keys::Key {
    fn from(wrapper: KeyWrapper) -> Self {
        if let Some((proto_key, _)) = KeyWrapper::get_mappings()
            .iter()
            .find(|(_, key)| *key == wrapper.0)
        {
            return *proto_key;
        }
        match wrapper.0 {
            Key::Unicode(_) => protos::keys::Key::KEY_UNICODE,
            Key::Other(_) => protos::keys::Key::KEY_OTHER,
            _ => protos::keys::Key::KEY_OTHER,
        }
    }
}

impl TryFrom<protos::key_config::KeyAction> for KeyWrapper {
    type Error = String;
    fn try_from(value: protos::key_config::KeyAction) -> Result<Self, Self::Error> {
        match value.key.enum_value() {
            Ok(key) => {
                let key_wrapper = KeyWrapper::from(key);
                match key_wrapper {
                    KeyWrapper(Key::Unicode(_)) => match value.unicode {
                        Some(unicode) => match char::try_from(unicode) {
                            Ok(c) => Ok(KeyWrapper(Key::Unicode(c))),
                            Err(e) => Err(e.to_string()),
                        },
                        None => {
                            Err("Unicode value not found when unicode key provided".to_string())
                        }
                    },
                    KeyWrapper(Key::Other(_)) => match value.other_key_code {
                        Some(key_code) => Ok(KeyWrapper(Key::Other(key_code))),
                        None => Err("Other key code not found when key is other".to_string()),
                    },
                    _ => Ok(key_wrapper),
                }
            }
            Err(e) => Err(format!(
                "Error matching key, an unsupported format may have been provided: {:?}",
                e.to_string()
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use firmware_api::inputs::InputActions::{Knob, Unknown};
    use messaging::protos::key_config::FreeformCommand;

    #[test]
    fn parse_key_action_properly() {
        let proto = protos::key_config::KeyAction {
            key: protobuf::EnumOrUnknown::from(protos::keys::Key::KEY_ADD),
            ..protos::key_config::KeyAction::default()
        };

        assert_eq!(KeyWrapper::try_from(proto).unwrap(), KeyWrapper(Key::Add));
    }

    #[test]
    fn currently_unsupported_keys_give_error() {
        let proto = protos::key_config::KeyAction {
            key: protobuf::EnumOrUnknown::from(protos::keys::Key::KEY_HANGUL),
            ..protos::key_config::KeyAction::default()
        };

        assert_eq!(
            KeyWrapper::try_from(proto).err().unwrap(),
            "Other key code not found when key is other"
        );
    }

    #[test]
    fn converts_input_id_to_action() {
        let proto = protos::inputs::InputId::KNOB_1_PRESSED;

        assert_eq!(
            InputActionWrapper::from(proto),
            InputActionWrapper(Knob(KnobActions::Knob1Pressed))
        );
    }

    #[test]
    fn handles_unknown_input_action() {
        let proto = protos::inputs::InputId::INPUT_ACTION_UNSPECIFIED;

        assert_eq!(InputActionWrapper::from(proto), InputActionWrapper(Unknown))
    }

    fn create_proto_fixture(
        proto_input_id: protos::inputs::InputId,
        action_data: Vec<protos::key_config::action::Action_data>,
    ) -> protos::key_config::KeyConfig {
        protos::key_config::KeyConfig {
            input_id: protobuf::EnumOrUnknown::new(proto_input_id),
            actions: action_data
                .iter()
                .map(|item| protos::key_config::Action {
                    action_data: Some(item.clone()),
                    ..protos::key_config::Action::default()
                })
                .collect(),
            ..protos::key_config::KeyConfig::default()
        }
    }
    #[test]
    fn converts_mapping_into_model() {
        let proto = create_proto_fixture(
            protos::inputs::InputId::KNOB_1_CLOCKWISE,
            vec![protos::key_config::action::Action_data::KeyAction(
                protos::key_config::KeyAction {
                    key: protos::keys::Key::KEY_ADD.into(),
                    ..protos::key_config::KeyAction::default()
                },
            )],
        );

        assert_eq!(
            InputMapping::try_from(proto).unwrap(),
            InputMapping::new(
                Knob(KnobActions::Knob1Clockwise),
                vec![Action::Key(Key::Add, vec![])]
            )
        )
    }

    #[test]
    fn converts_mapping_into_model_with_invalid_input() {
        let proto = create_proto_fixture(
            protos::inputs::InputId::INPUT_ACTION_UNSPECIFIED,
            vec![
                protos::key_config::action::Action_data::KeyAction(protos::key_config::KeyAction {
                    key: protos::keys::Key::KEY_ADD.into(),
                    ..protos::key_config::KeyAction::default()
                }),
                protos::key_config::action::Action_data::CommandAction(
                    protos::key_config::CommandAction {
                        command: Some(
                            protos::key_config::command_action::Command::FreeformCommand(
                                FreeformCommand {
                                    command: String::from("command"),
                                    args: vec![String::from("arg1"), String::from("arg2")],
                                    ..FreeformCommand::default()
                                },
                            ),
                        ),
                        ..protos::key_config::CommandAction::default()
                    },
                ),
            ],
        );

        assert_eq!(
            InputMapping::try_from(proto).unwrap(),
            InputMapping::new(
                Unknown,
                vec![
                    Action::Key(Key::Add, vec![]),
                    Action::Command(
                        String::from("command"),
                        vec![String::from("arg1"), String::from("arg2")]
                    )
                ]
            )
        )
    }
}
