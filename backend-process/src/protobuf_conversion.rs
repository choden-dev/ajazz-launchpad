use crate::database::models::{Action, ImageMapping, InputMapping};
use enigo::Key;
use firmware_api::display_zones::DisplayZones;
use firmware_api::inputs::InputActions;
use firmware_api::inputs::InputActions::Unknown;
use firmware_api::inputs::buttons::ButtonActions;
use firmware_api::inputs::knobs::KnobActions;
use firmware_api::inputs::touchscreen::TouchscreenAction;
use messaging::protos;
use messaging::protos::key_config::action::Action_data;
use messaging::protos::key_config::command_action;
use std::char;
use std::io::{Error, ErrorKind};

/// Util struct for mapping the protobuf key into an `Enigo` key
#[derive(Debug, PartialEq)]
pub struct KeyWrapper(Key);

#[derive(Debug, PartialEq)]
pub struct DisplayZoneWrapper(DisplayZones);

/// Util struct to convert from the protobuf format to our application model
#[derive(Debug, PartialEq)]
pub struct InputActionWrapper(InputActions);

impl From<protos::inputs::InputId> for InputActionWrapper {
    fn from(value: protos::inputs::InputId) -> InputActionWrapper {
        match value {
            protos::inputs::InputId::INPUT_ACTION_UNSPECIFIED => InputActionWrapper(Unknown),

            protos::inputs::InputId::BUTTON_1_PRESSED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button1Pressed))
            }
            protos::inputs::InputId::BUTTON_2_PRESSED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button2Pressed))
            }
            protos::inputs::InputId::BUTTON_3_PRESSED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button3Pressed))
            }
            protos::inputs::InputId::BUTTON_4_PRESSED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button4Pressed))
            }
            protos::inputs::InputId::BUTTON_5_PRESSED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button5Pressed))
            }
            protos::inputs::InputId::BUTTON_6_PRESSED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button6Pressed))
            }
            protos::inputs::InputId::BUTTON_7_PRESSED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button7Pressed))
            }
            protos::inputs::InputId::BUTTON_8_PRESSED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button8Pressed))
            }
            protos::inputs::InputId::BUTTON_9_PRESSED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button9Pressed))
            }
            protos::inputs::InputId::BUTTON_10_PRESSED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button10Pressed))
            }

            protos::inputs::InputId::BUTTON_1_RELEASED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button1Released))
            }
            protos::inputs::InputId::BUTTON_2_RELEASED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button2Released))
            }
            protos::inputs::InputId::BUTTON_3_RELEASED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button3Released))
            }
            protos::inputs::InputId::BUTTON_4_RELEASED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button4Released))
            }
            protos::inputs::InputId::BUTTON_5_RELEASED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button5Released))
            }
            protos::inputs::InputId::BUTTON_6_RELEASED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button6Released))
            }
            protos::inputs::InputId::BUTTON_7_RELEASED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button7Released))
            }
            protos::inputs::InputId::BUTTON_8_RELEASED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button8Released))
            }
            protos::inputs::InputId::BUTTON_9_RELEASED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button9Released))
            }
            protos::inputs::InputId::BUTTON_10_RELEASED => {
                InputActionWrapper(InputActions::Button(ButtonActions::Button10Released))
            }

            protos::inputs::InputId::KNOB_1_CLOCKWISE => {
                InputActionWrapper(InputActions::Knob(KnobActions::Knob1Clockwise))
            }
            protos::inputs::InputId::KNOB_2_CLOCKWISE => {
                InputActionWrapper(InputActions::Knob(KnobActions::Knob2Clockwise))
            }
            protos::inputs::InputId::KNOB_3_CLOCKWISE => {
                InputActionWrapper(InputActions::Knob(KnobActions::Knob3Clockwise))
            }
            protos::inputs::InputId::KNOB_4_CLOCKWISE => {
                InputActionWrapper(InputActions::Knob(KnobActions::Knob4Clockwise))
            }
            protos::inputs::InputId::KNOB_1_COUNTER_CLOCKWISE => {
                InputActionWrapper(InputActions::Knob(KnobActions::Knob1CounterClockwise))
            }
            protos::inputs::InputId::KNOB_2_COUNTER_CLOCKWISE => {
                InputActionWrapper(InputActions::Knob(KnobActions::Knob2CounterClockwise))
            }
            protos::inputs::InputId::KNOB_3_COUNTER_CLOCKWISE => {
                InputActionWrapper(InputActions::Knob(KnobActions::Knob3CounterClockwise))
            }
            protos::inputs::InputId::KNOB_4_COUNTER_CLOCKWISE => {
                InputActionWrapper(InputActions::Knob(KnobActions::Knob4CounterClockwise))
            }

            protos::inputs::InputId::KNOB_1_PRESSED => {
                InputActionWrapper(InputActions::Knob(KnobActions::Knob1Pressed))
            }
            protos::inputs::InputId::KNOB_2_PRESSED => {
                InputActionWrapper(InputActions::Knob(KnobActions::Knob2Pressed))
            }
            protos::inputs::InputId::KNOB_3_PRESSED => {
                InputActionWrapper(InputActions::Knob(KnobActions::Knob3Pressed))
            }
            protos::inputs::InputId::KNOB_4_PRESSED => {
                InputActionWrapper(InputActions::Knob(KnobActions::Knob4Pressed))
            }

            protos::inputs::InputId::TOUCHSCREEN_ZONE_1_PRESSED => {
                InputActionWrapper(InputActions::Touchscreen(TouchscreenAction::Zone1Pressed))
            }
            protos::inputs::InputId::TOUCHSCREEN_ZONE_2_PRESSED => {
                InputActionWrapper(InputActions::Touchscreen(TouchscreenAction::Zone2Pressed))
            }
            protos::inputs::InputId::TOUCHSCREEN_ZONE_3_PRESSED => {
                InputActionWrapper(InputActions::Touchscreen(TouchscreenAction::Zone3Pressed))
            }
            protos::inputs::InputId::TOUCHSCREEN_ZONE_4_PRESSED => {
                InputActionWrapper(InputActions::Touchscreen(TouchscreenAction::Zone4Pressed))
            }
            protos::inputs::InputId::TOUCHSCREEN_SWIPED_LEFT => {
                InputActionWrapper(InputActions::Touchscreen(TouchscreenAction::SwipedLeft))
            }
            protos::inputs::InputId::TOUCHSCREEN_SWIPED_RIGHT => {
                InputActionWrapper(InputActions::Touchscreen(TouchscreenAction::SwipedRight))
            }
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
                        Action_data::CommandAction(command) => {
                            if let Some(command_type) = command.command {
                                return match command_type {
                                    command_action::Command::FreeformCommand(command) => {
                                        Action::Command(command.command, command.args)
                                    }
                                    command_action::Command::OpenAppCommand(command) => {
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
                        Action_data::KeyAction(key) => {
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

impl TryFrom<protos::display_zones::DisplayZone> for DisplayZoneWrapper {
    type Error = Error;

    fn try_from(value: protos::display_zones::DisplayZone) -> Result<Self, Self::Error> {
        let value = match value {
            protos::display_zones::DisplayZone::BUTTON_1 => {
                DisplayZoneWrapper(DisplayZones::Button1)
            }
            protos::display_zones::DisplayZone::BUTTON_2 => {
                DisplayZoneWrapper(DisplayZones::Button2)
            }
            protos::display_zones::DisplayZone::BUTTON_3 => {
                DisplayZoneWrapper(DisplayZones::Button3)
            }
            protos::display_zones::DisplayZone::BUTTON_4 => {
                DisplayZoneWrapper(DisplayZones::Button4)
            }
            protos::display_zones::DisplayZone::BUTTON_5 => {
                DisplayZoneWrapper(DisplayZones::Button5)
            }
            protos::display_zones::DisplayZone::BUTTON_6 => {
                DisplayZoneWrapper(DisplayZones::Button6)
            }
            protos::display_zones::DisplayZone::BUTTON_7 => {
                DisplayZoneWrapper(DisplayZones::Button7)
            }
            protos::display_zones::DisplayZone::BUTTON_8 => {
                DisplayZoneWrapper(DisplayZones::Button8)
            }
            protos::display_zones::DisplayZone::BUTTON_9 => {
                DisplayZoneWrapper(DisplayZones::Button9)
            }
            protos::display_zones::DisplayZone::BUTTON_10 => {
                DisplayZoneWrapper(DisplayZones::Button10)
            }
            protos::display_zones::DisplayZone::TOUCHSCREEN_ZONE_1 => {
                DisplayZoneWrapper(DisplayZones::Touchscreen1)
            }
            protos::display_zones::DisplayZone::TOUCHSCREEN_ZONE_2 => {
                DisplayZoneWrapper(DisplayZones::Touchscreen2)
            }
            protos::display_zones::DisplayZone::TOUCHSCREEN_ZONE_3 => {
                DisplayZoneWrapper(DisplayZones::Touchscreen3)
            }
            protos::display_zones::DisplayZone::TOUCHSCREEN_ZONE_4 => {
                DisplayZoneWrapper(DisplayZones::Touchscreen4)
            }

            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Not a valid display zone",
            ))?,
        };

        Ok(value)
    }
}

impl From<protos::keys::Key> for KeyWrapper {
    fn from(value: protos::keys::Key) -> Self {
        match value {
            protos::keys::Key::KEY_ADD => KeyWrapper(Key::Add),
            protos::keys::Key::KEY_ALT => KeyWrapper(Key::Alt),
            protos::keys::Key::KEY_BACKSPACE => KeyWrapper(Key::Backspace),
            protos::keys::Key::KEY_CAPS_LOCK => KeyWrapper(Key::CapsLock),
            protos::keys::Key::KEY_CONTROL => KeyWrapper(Key::Control),
            protos::keys::Key::KEY_DECIMAL => KeyWrapper(Key::Decimal),
            protos::keys::Key::KEY_DELETE => KeyWrapper(Key::Delete),
            protos::keys::Key::KEY_DIVIDE => KeyWrapper(Key::Divide),
            protos::keys::Key::KEY_DOWN_ARROW => KeyWrapper(Key::DownArrow),
            protos::keys::Key::KEY_END => KeyWrapper(Key::End),
            protos::keys::Key::KEY_ESCAPE => KeyWrapper(Key::Escape),
            protos::keys::Key::KEY_F1 => KeyWrapper(Key::F1),
            protos::keys::Key::KEY_F2 => KeyWrapper(Key::F2),
            protos::keys::Key::KEY_F3 => KeyWrapper(Key::F3),
            protos::keys::Key::KEY_F4 => KeyWrapper(Key::F4),
            protos::keys::Key::KEY_F5 => KeyWrapper(Key::F5),
            protos::keys::Key::KEY_F6 => KeyWrapper(Key::F6),
            protos::keys::Key::KEY_F7 => KeyWrapper(Key::F7),
            protos::keys::Key::KEY_F8 => KeyWrapper(Key::F8),
            protos::keys::Key::KEY_F9 => KeyWrapper(Key::F9),
            protos::keys::Key::KEY_F10 => KeyWrapper(Key::F10),
            protos::keys::Key::KEY_F11 => KeyWrapper(Key::F11),
            protos::keys::Key::KEY_F12 => KeyWrapper(Key::F12),
            protos::keys::Key::KEY_F13 => KeyWrapper(Key::F13),
            protos::keys::Key::KEY_F14 => KeyWrapper(Key::F14),
            protos::keys::Key::KEY_F15 => KeyWrapper(Key::F15),
            protos::keys::Key::KEY_F16 => KeyWrapper(Key::F16),
            protos::keys::Key::KEY_F17 => KeyWrapper(Key::F17),
            protos::keys::Key::KEY_F18 => KeyWrapper(Key::F18),
            protos::keys::Key::KEY_F19 => KeyWrapper(Key::F19),
            protos::keys::Key::KEY_F20 => KeyWrapper(Key::F20),
            protos::keys::Key::KEY_HELP => KeyWrapper(Key::Help),
            protos::keys::Key::KEY_HOME => KeyWrapper(Key::Home),
            protos::keys::Key::KEY_L_CONTROL => KeyWrapper(Key::LControl),
            protos::keys::Key::KEY_LEFT_ARROW => KeyWrapper(Key::LeftArrow),
            protos::keys::Key::KEY_L_SHIFT => KeyWrapper(Key::LShift),
            protos::keys::Key::KEY_MEDIA_NEXT_TRACK => KeyWrapper(Key::MediaNextTrack),
            protos::keys::Key::KEY_MEDIA_PLAY_PAUSE => KeyWrapper(Key::MediaPlayPause),
            protos::keys::Key::KEY_MEDIA_PREV_TRACK => KeyWrapper(Key::MediaPrevTrack),
            protos::keys::Key::KEY_META => KeyWrapper(Key::Meta),
            protos::keys::Key::KEY_MULTIPLY => KeyWrapper(Key::Multiply),
            protos::keys::Key::KEY_NUMPAD0 => KeyWrapper(Key::Numpad0),
            protos::keys::Key::KEY_NUMPAD1 => KeyWrapper(Key::Numpad1),
            protos::keys::Key::KEY_NUMPAD2 => KeyWrapper(Key::Numpad2),
            protos::keys::Key::KEY_NUMPAD3 => KeyWrapper(Key::Numpad3),
            protos::keys::Key::KEY_NUMPAD4 => KeyWrapper(Key::Numpad4),
            protos::keys::Key::KEY_NUMPAD5 => KeyWrapper(Key::Numpad5),
            protos::keys::Key::KEY_NUMPAD6 => KeyWrapper(Key::Numpad6),
            protos::keys::Key::KEY_NUMPAD7 => KeyWrapper(Key::Numpad7),
            protos::keys::Key::KEY_NUMPAD8 => KeyWrapper(Key::Numpad8),
            protos::keys::Key::KEY_NUMPAD9 => KeyWrapper(Key::Numpad9),
            protos::keys::Key::KEY_OPTION => KeyWrapper(Key::Option),
            protos::keys::Key::KEY_PAGE_DOWN => KeyWrapper(Key::PageDown),
            protos::keys::Key::KEY_PAGE_UP => KeyWrapper(Key::PageUp),
            protos::keys::Key::KEY_R_CONTROL => KeyWrapper(Key::RControl),
            protos::keys::Key::KEY_RETURN => KeyWrapper(Key::Return),
            protos::keys::Key::KEY_RIGHT_ARROW => KeyWrapper(Key::RightArrow),
            protos::keys::Key::KEY_R_SHIFT => KeyWrapper(Key::RShift),
            protos::keys::Key::KEY_SHIFT => KeyWrapper(Key::Shift),
            protos::keys::Key::KEY_SPACE => KeyWrapper(Key::Space),
            protos::keys::Key::KEY_SUBTRACT => KeyWrapper(Key::Subtract),
            protos::keys::Key::KEY_TAB => KeyWrapper(Key::Tab),
            protos::keys::Key::KEY_UP_ARROW => KeyWrapper(Key::UpArrow),
            protos::keys::Key::KEY_VOLUME_DOWN => KeyWrapper(Key::VolumeDown),
            protos::keys::Key::KEY_VOLUME_MUTE => KeyWrapper(Key::VolumeMute),
            protos::keys::Key::KEY_VOLUME_UP => KeyWrapper(Key::VolumeUp),
            protos::keys::Key::KEY_UNICODE => KeyWrapper(Key::Unicode(char::default())),
            protos::keys::Key::KEY_OTHER => KeyWrapper(Key::Other(u32::default())),
            _ => KeyWrapper(Key::Other(u32::default())),
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
                        None => Err("Unicode value not found when unicode key provided".to_string()),
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
    use firmware_api::inputs::InputActions::Knob;
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
        action_data: Vec<Action_data>,
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
            vec![Action_data::KeyAction(protos::key_config::KeyAction {
                key: protos::keys::Key::KEY_ADD.into(),
                ..protos::key_config::KeyAction::default()
            })],
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
                Action_data::KeyAction(protos::key_config::KeyAction {
                    key: protos::keys::Key::KEY_ADD.into(),
                    ..protos::key_config::KeyAction::default()
                }),
                Action_data::CommandAction(protos::key_config::CommandAction {
                    command: Some(command_action::Command::FreeformCommand(FreeformCommand {
                        command: String::from("command"),
                        args: vec![String::from("arg1"), String::from("arg2")],
                        ..FreeformCommand::default()
                    })),
                    ..protos::key_config::CommandAction::default()
                }),
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
