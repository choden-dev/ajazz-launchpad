use crate::database::models::InputMapping;
use enigo::Key;
use firmware_api::inputs::InputActions;
use firmware_api::inputs::InputActions::Unknown;
use firmware_api::inputs::buttons::ButtonActions;
use firmware_api::inputs::knobs::KnobActions;
use firmware_api::inputs::touchscreen::TouchscreenAction;
use messaging::protos;
use protobuf::Enum;

/// Util struct for mapping the protobuf key into an `Enigo` key
#[derive(Debug, PartialEq)]
pub struct KeyWrapper(Key);

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
        let actions: Vec<KeyWrapper> = value
            .actions
            .iter()
            .map(|a| a.key_action().clone().try_into().map_err(|_| ()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(InputMapping::new(
            input_id.0,
            actions.iter().map(|a| a.0).collect(),
        ))
    }
}

impl TryFrom<protos::key_config::KeyAction> for KeyWrapper {
    type Error = String;
    fn try_from(value: protos::key_config::KeyAction) -> Result<Self, Self::Error> {
        match value.key.enum_value() {
            Ok(key) => match key {
                protos::keys::Key::KEY_ADD => Ok(KeyWrapper(Key::Add)),
                protos::keys::Key::KEY_ALT => Ok(KeyWrapper(Key::Alt)),
                protos::keys::Key::KEY_BACKSPACE => Ok(KeyWrapper(Key::Backspace)),
                protos::keys::Key::KEY_CAPS_LOCK => Ok(KeyWrapper(Key::CapsLock)),
                protos::keys::Key::KEY_CONTROL => Ok(KeyWrapper(Key::Control)),
                protos::keys::Key::KEY_DECIMAL => Ok(KeyWrapper(Key::Decimal)),
                protos::keys::Key::KEY_DELETE => Ok(KeyWrapper(Key::Delete)),
                protos::keys::Key::KEY_DIVIDE => Ok(KeyWrapper(Key::Divide)),
                protos::keys::Key::KEY_DOWN_ARROW => Ok(KeyWrapper(Key::DownArrow)),
                protos::keys::Key::KEY_END => Ok(KeyWrapper(Key::End)),
                protos::keys::Key::KEY_ESCAPE => Ok(KeyWrapper(Key::Escape)),
                protos::keys::Key::KEY_F1 => Ok(KeyWrapper(Key::F1)),
                protos::keys::Key::KEY_F2 => Ok(KeyWrapper(Key::F2)),
                protos::keys::Key::KEY_F3 => Ok(KeyWrapper(Key::F3)),
                protos::keys::Key::KEY_F4 => Ok(KeyWrapper(Key::F4)),
                protos::keys::Key::KEY_F5 => Ok(KeyWrapper(Key::F5)),
                protos::keys::Key::KEY_F6 => Ok(KeyWrapper(Key::F6)),
                protos::keys::Key::KEY_F7 => Ok(KeyWrapper(Key::F7)),
                protos::keys::Key::KEY_F8 => Ok(KeyWrapper(Key::F8)),
                protos::keys::Key::KEY_F9 => Ok(KeyWrapper(Key::F9)),
                protos::keys::Key::KEY_F10 => Ok(KeyWrapper(Key::F10)),
                protos::keys::Key::KEY_F11 => Ok(KeyWrapper(Key::F11)),
                protos::keys::Key::KEY_F12 => Ok(KeyWrapper(Key::F12)),
                protos::keys::Key::KEY_F13 => Ok(KeyWrapper(Key::F13)),
                protos::keys::Key::KEY_F14 => Ok(KeyWrapper(Key::F14)),
                protos::keys::Key::KEY_F15 => Ok(KeyWrapper(Key::F15)),
                protos::keys::Key::KEY_F16 => Ok(KeyWrapper(Key::F16)),
                protos::keys::Key::KEY_F17 => Ok(KeyWrapper(Key::F17)),
                protos::keys::Key::KEY_F18 => Ok(KeyWrapper(Key::F18)),
                protos::keys::Key::KEY_F19 => Ok(KeyWrapper(Key::F19)),
                protos::keys::Key::KEY_F20 => Ok(KeyWrapper(Key::F20)),
                protos::keys::Key::KEY_HELP => Ok(KeyWrapper(Key::Help)),
                protos::keys::Key::KEY_HOME => Ok(KeyWrapper(Key::Home)),
                protos::keys::Key::KEY_L_CONTROL => Ok(KeyWrapper(Key::LControl)),
                protos::keys::Key::KEY_LEFT_ARROW => Ok(KeyWrapper(Key::LeftArrow)),
                protos::keys::Key::KEY_L_SHIFT => Ok(KeyWrapper(Key::LShift)),
                protos::keys::Key::KEY_MEDIA_NEXT_TRACK => Ok(KeyWrapper(Key::MediaNextTrack)),
                protos::keys::Key::KEY_MEDIA_PLAY_PAUSE => Ok(KeyWrapper(Key::MediaPlayPause)),
                protos::keys::Key::KEY_MEDIA_PREV_TRACK => Ok(KeyWrapper(Key::MediaPrevTrack)),
                protos::keys::Key::KEY_META => Ok(KeyWrapper(Key::Meta)),
                protos::keys::Key::KEY_MULTIPLY => Ok(KeyWrapper(Key::Multiply)),
                protos::keys::Key::KEY_NUMPAD0 => Ok(KeyWrapper(Key::Numpad0)),
                protos::keys::Key::KEY_NUMPAD1 => Ok(KeyWrapper(Key::Numpad1)),
                protos::keys::Key::KEY_NUMPAD2 => Ok(KeyWrapper(Key::Numpad2)),
                protos::keys::Key::KEY_NUMPAD3 => Ok(KeyWrapper(Key::Numpad3)),
                protos::keys::Key::KEY_NUMPAD4 => Ok(KeyWrapper(Key::Numpad4)),
                protos::keys::Key::KEY_NUMPAD5 => Ok(KeyWrapper(Key::Numpad5)),
                protos::keys::Key::KEY_NUMPAD6 => Ok(KeyWrapper(Key::Numpad6)),
                protos::keys::Key::KEY_NUMPAD7 => Ok(KeyWrapper(Key::Numpad7)),
                protos::keys::Key::KEY_NUMPAD8 => Ok(KeyWrapper(Key::Numpad8)),
                protos::keys::Key::KEY_NUMPAD9 => Ok(KeyWrapper(Key::Numpad9)),
                protos::keys::Key::KEY_OPTION => Ok(KeyWrapper(Key::Option)),
                protos::keys::Key::KEY_PAGE_DOWN => Ok(KeyWrapper(Key::PageDown)),
                protos::keys::Key::KEY_PAGE_UP => Ok(KeyWrapper(Key::PageUp)),
                protos::keys::Key::KEY_R_CONTROL => Ok(KeyWrapper(Key::RControl)),
                protos::keys::Key::KEY_RETURN => Ok(KeyWrapper(Key::Return)),
                protos::keys::Key::KEY_RIGHT_ARROW => Ok(KeyWrapper(Key::RightArrow)),
                protos::keys::Key::KEY_R_SHIFT => Ok(KeyWrapper(Key::RShift)),
                protos::keys::Key::KEY_SHIFT => Ok(KeyWrapper(Key::Shift)),
                protos::keys::Key::KEY_SPACE => Ok(KeyWrapper(Key::Space)),
                protos::keys::Key::KEY_SUBTRACT => Ok(KeyWrapper(Key::Subtract)),
                protos::keys::Key::KEY_TAB => Ok(KeyWrapper(Key::Tab)),
                protos::keys::Key::KEY_UP_ARROW => Ok(KeyWrapper(Key::UpArrow)),
                protos::keys::Key::KEY_VOLUME_DOWN => Ok(KeyWrapper(Key::VolumeDown)),
                protos::keys::Key::KEY_VOLUME_MUTE => Ok(KeyWrapper(Key::VolumeMute)),
                protos::keys::Key::KEY_VOLUME_UP => Ok(KeyWrapper(Key::VolumeUp)),
                protos::keys::Key::KEY_UNICODE => match value.unicode {
                    Some(unicode) => match char::try_from(unicode) {
                        Ok(c) => Ok(KeyWrapper(Key::Unicode(c))),
                        Err(e) => Err(e.to_string()),
                    },
                    None => Err("Unicode value not found".to_string()),
                },
                protos::keys::Key::KEY_OTHER => match value.other_key_code {
                    Some(key_code) => Ok(KeyWrapper(Key::Other(key_code))),
                    None => Err("Other key code not found".to_string()),
                },
                _ => Err(format!("Unsupported key format {}", key.value())),
            },
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
            "Unsupported key format 54"
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
        proto_key: protos::keys::Key,
    ) -> protos::key_config::KeyConfig {
        protos::key_config::KeyConfig {
            input_id: protobuf::EnumOrUnknown::new(proto_input_id),
            actions: vec![protos::key_config::Action {
                action_data: Some(protos::key_config::action::Action_data::KeyAction(
                    protos::key_config::KeyAction {
                        key: protobuf::EnumOrUnknown::from(proto_key),
                        ..protos::key_config::KeyAction::default()
                    },
                )),
                ..protos::key_config::Action::default()
            }],
            ..protos::key_config::KeyConfig::default()
        }
    }
    #[test]
    fn converts_mapping_into_model() {
        let proto = create_proto_fixture(
            protos::inputs::InputId::KNOB_1_CLOCKWISE,
            protos::keys::Key::KEY_ADD,
        );

        assert_eq!(
            InputMapping::try_from(proto).unwrap(),
            InputMapping::new(
                InputActions::Knob(KnobActions::Knob1Clockwise),
                vec![Key::Add]
            )
        )
    }

    #[test]
    fn converts_mapping_into_model_with_invalid_input() {
        let proto = create_proto_fixture(
            protos::inputs::InputId::INPUT_ACTION_UNSPECIFIED,
            protos::keys::Key::KEY_ADD,
        );

        assert_eq!(
            InputMapping::try_from(proto).unwrap(),
            InputMapping::new(Unknown, vec![Key::Add])
        )
    }
}
