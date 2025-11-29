use crate::common::{
    ButtonInput, ConfigurableZones, KnobInput, TouchscreenInput, TouchscreenZoneInput,
};
use iced::keyboard::key::Named;
use iced::keyboard::{Key as IcedKey, Modifiers};
use messaging::protos::inputs::InputId;
use messaging::protos::key_config::KeyAction;
use messaging::protos::keys::Key;
use protobuf::EnumOrUnknown;

/// Util type to assist with conversion to the protobuf Key format
pub struct ProtoKeyWrapper(Key);

pub struct ProtoModifierWrapper(Vec<Key>);

impl ProtoKeyWrapper {
    pub fn key(&self) -> Key {
        self.0
    }
}

pub struct ProtoKeyActionWrapper(KeyAction);

impl ProtoKeyActionWrapper {
    pub fn key_action(&self) -> KeyAction {
        self.0.to_owned()
    }
}

impl From<(IcedKey, Modifiers)> for ProtoKeyActionWrapper {
    fn from((key, modifiers): (IcedKey, Modifiers)) -> Self {
        ProtoKeyActionWrapper(KeyAction {
            key: EnumOrUnknown::from(ProtoKeyWrapper::from(key.clone()).key()),
            unicode: match key {
                IcedKey::Character(char) => {
                    char.as_bytes().first().map(|key_code| u32::from(*key_code))
                }
                _ => None,
            },
            modifier: ProtoModifierWrapper::from(modifiers)
                .0
                .iter()
                .map(|modifier_key| EnumOrUnknown::new(*modifier_key))
                .collect(),
            ..KeyAction::default()
        })
    }
}

impl From<Modifiers> for ProtoModifierWrapper {
    fn from(modifiers: Modifiers) -> Self {
        let mut list = vec![];

        if modifiers.command() {
            list.push(Key::KEY_COMMAND);
        }

        if modifiers.shift() {
            list.push(Key::KEY_SHIFT);
        }

        if modifiers.alt() {
            list.push(Key::KEY_ALT);
        }

        if modifiers.macos_command() {
            list.push(Key::KEY_COMMAND);
        }

        if modifiers.jump() {
            list.push(Key::KEY_CONTROL);
        }

        ProtoModifierWrapper(list)
    }
}

impl From<IcedKey> for ProtoKeyWrapper {
    fn from(value: IcedKey) -> Self {
        ProtoKeyWrapper(match value {
            IcedKey::Named(name) => match name {
                Named::Alt => Key::KEY_ALT,
                Named::CapsLock => Key::KEY_CAPS_LOCK,
                Named::Control => Key::KEY_CONTROL,
                Named::NumLock => Key::KEY_NUMLOCK,
                Named::ScrollLock => Key::KEY_SCROLL_LOCK,
                Named::Shift => Key::KEY_SHIFT,
                Named::Meta => Key::KEY_META,
                Named::Super => Key::KEY_SUPER,
                Named::Enter => Key::KEY_RETURN,
                Named::Tab => Key::KEY_TAB,
                Named::Space => Key::KEY_SPACE,
                Named::ArrowDown => Key::KEY_DOWN_ARROW,
                Named::ArrowLeft => Key::KEY_LEFT_ARROW,
                Named::ArrowRight => Key::KEY_RIGHT_ARROW,
                Named::ArrowUp => Key::KEY_UP_ARROW,
                Named::End => Key::KEY_END,
                Named::Home => Key::KEY_HOME,
                Named::PageDown => Key::KEY_PAGE_DOWN,
                Named::PageUp => Key::KEY_PAGE_UP,
                Named::Backspace => Key::KEY_BACKSPACE,
                Named::Clear => Key::KEY_CLEAR,
                Named::Delete => Key::KEY_DELETE,
                Named::Insert => Key::KEY_INSERT,
                Named::Redo => Key::KEY_REDO,
                Named::Undo => Key::KEY_UNDO,
                Named::Cancel => Key::KEY_CANCEL,
                Named::Escape => Key::KEY_ESCAPE,
                Named::Execute => Key::KEY_EXECUTE,
                Named::Find => Key::KEY_FIND,
                Named::Help => Key::KEY_HELP,
                Named::Pause => Key::KEY_PAUSE,
                Named::Select => Key::KEY_SELECT,
                Named::PrintScreen => Key::KEY_PRINT_SCR,
                Named::ModeChange => Key::KEY_MODE_CHANGE,
                Named::HangulMode => Key::KEY_HANGUL,
                Named::HanjaMode => Key::KEY_HANJA,
                Named::KanjiMode => Key::KEY_KANJI,
                Named::MediaPlayPause => Key::KEY_MEDIA_PLAY_PAUSE,
                Named::MediaStop => Key::KEY_MEDIA_STOP,
                Named::MediaTrackNext => Key::KEY_MEDIA_NEXT_TRACK,
                Named::MediaTrackPrevious => Key::KEY_MEDIA_PREV_TRACK,
                Named::Print => Key::KEY_PRINT,
                Named::AudioVolumeDown => Key::KEY_VOLUME_DOWN,
                Named::AudioVolumeUp => Key::KEY_VOLUME_UP,
                Named::AudioVolumeMute => Key::KEY_VOLUME_MUTE,
                Named::MicrophoneVolumeMute => Key::KEY_MIC_MUTE,
                Named::F1 => Key::KEY_F1,
                Named::F2 => Key::KEY_F2,
                Named::F3 => Key::KEY_F3,
                Named::F4 => Key::KEY_F4,
                Named::F5 => Key::KEY_F5,
                Named::F6 => Key::KEY_F6,
                Named::F7 => Key::KEY_F7,
                Named::F8 => Key::KEY_F8,
                Named::F9 => Key::KEY_F9,
                Named::F10 => Key::KEY_F10,
                Named::F11 => Key::KEY_F11,
                Named::F12 => Key::KEY_F12,
                Named::F13 => Key::KEY_F13,
                Named::F14 => Key::KEY_F14,
                Named::F15 => Key::KEY_F15,
                Named::F16 => Key::KEY_F16,
                Named::F17 => Key::KEY_F17,
                Named::F18 => Key::KEY_F18,
                Named::F19 => Key::KEY_F19,
                Named::F20 => Key::KEY_F20,
                Named::F21 => Key::KEY_F21,
                Named::F22 => Key::KEY_F22,
                Named::F23 => Key::KEY_F23,
                Named::F24 => Key::KEY_F24,
                Named::F25 => Key::KEY_F25,
                Named::F26 => Key::KEY_F26,
                Named::F27 => Key::KEY_F27,
                Named::F28 => Key::KEY_F28,
                Named::F29 => Key::KEY_F29,
                Named::F30 => Key::KEY_F30,
                Named::F31 => Key::KEY_F31,
                Named::F32 => Key::KEY_F32,
                Named::F33 => Key::KEY_F33,
                Named::F34 => Key::KEY_F34,
                Named::F35 => Key::KEY_F35,
                _ => Key::KEY_OTHER,
            },
            IcedKey::Character(_) => Key::KEY_UNICODE,
            IcedKey::Unidentified => Key::KEY_UNSPECIFIED,
        })
    }
}

impl From<ConfigurableZones> for InputId {
    fn from(zones: ConfigurableZones) -> Self {
        match zones {
            ConfigurableZones::Button1(ButtonInput::Pressed) => InputId::BUTTON_1_PRESSED,
            ConfigurableZones::Button1(ButtonInput::Released) => InputId::BUTTON_1_RELEASED,
            ConfigurableZones::Button2(ButtonInput::Pressed) => InputId::BUTTON_2_PRESSED,
            ConfigurableZones::Button2(ButtonInput::Released) => InputId::BUTTON_2_RELEASED,
            ConfigurableZones::Button3(ButtonInput::Pressed) => InputId::BUTTON_3_PRESSED,
            ConfigurableZones::Button3(ButtonInput::Released) => InputId::BUTTON_3_RELEASED,
            ConfigurableZones::Button4(ButtonInput::Pressed) => InputId::BUTTON_4_PRESSED,
            ConfigurableZones::Button4(ButtonInput::Released) => InputId::BUTTON_4_RELEASED,
            ConfigurableZones::Button5(ButtonInput::Pressed) => InputId::BUTTON_5_PRESSED,
            ConfigurableZones::Button5(ButtonInput::Released) => InputId::BUTTON_5_RELEASED,
            ConfigurableZones::Button6(ButtonInput::Pressed) => InputId::BUTTON_6_PRESSED,
            ConfigurableZones::Button6(ButtonInput::Released) => InputId::BUTTON_6_RELEASED,
            ConfigurableZones::Button7(ButtonInput::Pressed) => InputId::BUTTON_7_PRESSED,
            ConfigurableZones::Button7(ButtonInput::Released) => InputId::BUTTON_7_RELEASED,
            ConfigurableZones::Button8(ButtonInput::Pressed) => InputId::BUTTON_8_PRESSED,
            ConfigurableZones::Button8(ButtonInput::Released) => InputId::BUTTON_8_RELEASED,
            ConfigurableZones::Button9(ButtonInput::Pressed) => InputId::BUTTON_9_PRESSED,
            ConfigurableZones::Button9(ButtonInput::Released) => InputId::BUTTON_9_RELEASED,
            ConfigurableZones::Button10(ButtonInput::Pressed) => InputId::BUTTON_10_PRESSED,
            ConfigurableZones::Button10(ButtonInput::Released) => InputId::BUTTON_10_RELEASED,

            ConfigurableZones::Knob1(KnobInput::Pressed) => InputId::KNOB_1_PRESSED,
            ConfigurableZones::Knob1(KnobInput::Clockwise) => InputId::KNOB_1_CLOCKWISE,
            ConfigurableZones::Knob1(KnobInput::CounterClockwise) => {
                InputId::KNOB_1_COUNTER_CLOCKWISE
            }
            ConfigurableZones::Knob2(KnobInput::Pressed) => InputId::KNOB_2_PRESSED,
            ConfigurableZones::Knob2(KnobInput::Clockwise) => InputId::KNOB_2_CLOCKWISE,
            ConfigurableZones::Knob2(KnobInput::CounterClockwise) => {
                InputId::KNOB_2_COUNTER_CLOCKWISE
            }
            ConfigurableZones::Knob3(KnobInput::Pressed) => InputId::KNOB_3_PRESSED,
            ConfigurableZones::Knob3(KnobInput::Clockwise) => InputId::KNOB_3_CLOCKWISE,
            ConfigurableZones::Knob3(KnobInput::CounterClockwise) => {
                InputId::KNOB_3_COUNTER_CLOCKWISE
            }
            ConfigurableZones::Knob4(KnobInput::Pressed) => InputId::KNOB_4_PRESSED,
            ConfigurableZones::Knob4(KnobInput::Clockwise) => InputId::KNOB_4_CLOCKWISE,
            ConfigurableZones::Knob4(KnobInput::CounterClockwise) => {
                InputId::KNOB_4_COUNTER_CLOCKWISE
            }

            ConfigurableZones::Touchscreen1(TouchscreenZoneInput::Pressed) => {
                InputId::TOUCHSCREEN_ZONE_1_PRESSED
            }
            ConfigurableZones::Touchscreen2(TouchscreenZoneInput::Pressed) => {
                InputId::TOUCHSCREEN_ZONE_2_PRESSED
            }
            ConfigurableZones::Touchscreen3(TouchscreenZoneInput::Pressed) => {
                InputId::TOUCHSCREEN_ZONE_3_PRESSED
            }
            ConfigurableZones::Touchscreen4(TouchscreenZoneInput::Pressed) => {
                InputId::TOUCHSCREEN_ZONE_4_PRESSED
            }

            ConfigurableZones::TouchscreenExtra(TouchscreenInput::SwipeLeft) => {
                InputId::TOUCHSCREEN_SWIPED_LEFT
            }

            ConfigurableZones::TouchscreenExtra(TouchscreenInput::SwipeRight) => {
                InputId::TOUCHSCREEN_SWIPED_RIGHT
            }

            _ => InputId::INPUT_ACTION_UNSPECIFIED,
        }
    }
}
