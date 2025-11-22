use crate::common::{
    ButtonInput, ConfigurableZones, KnobInput, TouchscreenInput, TouchscreenZoneInput,
};
use crate::components::modal::modal;
use crate::messages::Messages;
use iced::keyboard::{Key, Modifiers};
use iced::widget::{column, row};
use iced::{Length, widget};
use messaging::protos::display_zones::DisplayZone;
use messaging::protos::inputs::InputId;

pub struct Config;

const BUTTON_COUNT: u8 = 10;
const KNOB_COUNT: u8 = 4;

const TOUCHSCREEN_ZONES: u8 = 4;

impl Into<InputId> for ConfigurableZones {
    fn into(self) -> InputId {
        match self {
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

impl<'a> Config {
    fn input_capture_modal(
        input: ConfigurableZones,
        current_key_sequence: Vec<(Key, Modifiers)>,
    ) -> widget::Column<'a, Messages> {
        let input_id: InputId = input.into();

        let current_display = widget::text!("{:?}", current_key_sequence);

        let submit_button =
            widget::button("OK").on_press(Messages::SetKeyConfig(input_id, current_key_sequence));

        column![current_display, submit_button]
    }
    fn button_grid_first_row() -> widget::Row<'a, Messages> {
        (0..BUTTON_COUNT / 2).fold(row![], |row, i| {
            let button_mapping = match i {
                0 => ("Button1", ConfigurableZones::Button1(ButtonInput::None)),
                1 => ("Button2", ConfigurableZones::Button2(ButtonInput::None)),
                2 => ("Button3", ConfigurableZones::Button3(ButtonInput::None)),
                3 => ("Button4", ConfigurableZones::Button4(ButtonInput::None)),
                4 => ("Button5", ConfigurableZones::Button5(ButtonInput::None)),
                _ => ("Unsupported Button", ConfigurableZones::None),
            };

            let button = widget::button(button_mapping.0)
                .on_press(Messages::OpenConfigurationPanel(button_mapping.1));

            row.push(button)
        })
    }
    fn button_config_settings(button: ConfigurableZones) -> widget::Column<'a, Messages> {
        let button_mapping = match button {
            ConfigurableZones::Button1(_) => (
                "Button1",
                DisplayZone::BUTTON_1,
                ConfigurableZones::Button1(ButtonInput::Pressed),
                ConfigurableZones::Button1(ButtonInput::Released),
            ),
            ConfigurableZones::Button2(_) => (
                "Button2",
                DisplayZone::BUTTON_2,
                ConfigurableZones::Button2(ButtonInput::Pressed),
                ConfigurableZones::Button2(ButtonInput::Released),
            ),
            ConfigurableZones::Button3(_) => (
                "Button3",
                DisplayZone::BUTTON_3,
                ConfigurableZones::Button3(ButtonInput::Pressed),
                ConfigurableZones::Button3(ButtonInput::Released),
            ),
            ConfigurableZones::Button4(_) => (
                "Button4",
                DisplayZone::BUTTON_4,
                ConfigurableZones::Button4(ButtonInput::Pressed),
                ConfigurableZones::Button4(ButtonInput::Released),
            ),
            ConfigurableZones::Button5(_) => (
                "Button5",
                DisplayZone::BUTTON_5,
                ConfigurableZones::Button5(ButtonInput::Pressed),
                ConfigurableZones::Button5(ButtonInput::Released),
            ),
            ConfigurableZones::Button6(_) => (
                "Button6",
                DisplayZone::BUTTON_6,
                ConfigurableZones::Button6(ButtonInput::Pressed),
                ConfigurableZones::Button6(ButtonInput::Released),
            ),
            ConfigurableZones::Button7(_) => (
                "Button7",
                DisplayZone::BUTTON_7,
                ConfigurableZones::Button7(ButtonInput::Pressed),
                ConfigurableZones::Button7(ButtonInput::Released),
            ),
            ConfigurableZones::Button8(_) => (
                "Button8",
                DisplayZone::BUTTON_8,
                ConfigurableZones::Button8(ButtonInput::Pressed),
                ConfigurableZones::Button8(ButtonInput::Released),
            ),
            ConfigurableZones::Button9(_) => (
                "Button9",
                DisplayZone::BUTTON_9,
                ConfigurableZones::Button9(ButtonInput::Pressed),
                ConfigurableZones::Button9(ButtonInput::Released),
            ),
            ConfigurableZones::Button10(_) => (
                "Button10",
                DisplayZone::BUTTON_10,
                ConfigurableZones::Button10(ButtonInput::Pressed),
                ConfigurableZones::Button10(ButtonInput::Released),
            ),
            _ => (
                "Unsupported Button",
                DisplayZone::DISPLAY_ZONE_UNSPECIFIED,
                ConfigurableZones::None,
                ConfigurableZones::None,
            ),
        };
        let title = widget::text!("{} config", button_mapping.0);

        let button =
            widget::button("Set Image").on_press(Messages::SetDisplayZoneImage(button_mapping.1));

        let pressed_action_button = widget::button("On pressed").on_press(
            Messages::OpenInputMappingConfigurationPanel(button_mapping.2),
        );
        let released_action_button = widget::button("On released").on_press(
            Messages::OpenInputMappingConfigurationPanel(button_mapping.3),
        );

        column![title, button, pressed_action_button, released_action_button]
    }
    fn button_grid_second_row() -> widget::Row<'a, Messages> {
        (BUTTON_COUNT / 2..BUTTON_COUNT).fold(row![], |row, i| {
            let button_mapping = match i {
                5 => ("Button1", ConfigurableZones::Button6(ButtonInput::None)),
                6 => ("Button2", ConfigurableZones::Button7(ButtonInput::None)),
                7 => ("Button3", ConfigurableZones::Button8(ButtonInput::None)),
                8 => ("Button4", ConfigurableZones::Button9(ButtonInput::None)),
                9 => ("Button5", ConfigurableZones::Button10(ButtonInput::None)),
                _ => ("Unsupported Button", ConfigurableZones::None),
            };

            let button = widget::button(button_mapping.0)
                .on_press(Messages::OpenConfigurationPanel(button_mapping.1));

            row.push(button)
        })
    }

    fn touchscreen_zones_row() -> widget::Row<'a, Messages> {
        (0..TOUCHSCREEN_ZONES).fold(row![], |row, i| {
            let touchscreen_zone_mapping = match i {
                0 => (
                    "Zone 1",
                    ConfigurableZones::Touchscreen1(TouchscreenZoneInput::None),
                ),
                1 => (
                    "Zone 2",
                    ConfigurableZones::Touchscreen2(TouchscreenZoneInput::None),
                ),
                2 => (
                    "Zone 3",
                    ConfigurableZones::Touchscreen3(TouchscreenZoneInput::None),
                ),
                3 => (
                    "Zone 4",
                    ConfigurableZones::Touchscreen4(TouchscreenZoneInput::None),
                ),
                _ => ("Unsupported Button", ConfigurableZones::None),
            };

            let button = widget::button(touchscreen_zone_mapping.0)
                .on_press(Messages::OpenConfigurationPanel(touchscreen_zone_mapping.1));

            row.push(button)
        })
    }

    fn touchscreen_zone_config_settings(zone: ConfigurableZones) -> widget::Column<'a, Messages> {
        let touchscreen_zone_mapping = match zone {
            ConfigurableZones::Touchscreen1(_) => (
                "Zone 1",
                DisplayZone::TOUCHSCREEN_ZONE_1,
                ConfigurableZones::Touchscreen1(TouchscreenZoneInput::Pressed),
            ),
            ConfigurableZones::Touchscreen2(_) => (
                "Zone 2",
                DisplayZone::TOUCHSCREEN_ZONE_2,
                ConfigurableZones::Touchscreen2(TouchscreenZoneInput::Pressed),
            ),
            ConfigurableZones::Touchscreen3(_) => (
                "Zone 3",
                DisplayZone::TOUCHSCREEN_ZONE_3,
                ConfigurableZones::Touchscreen3(TouchscreenZoneInput::Pressed),
            ),
            ConfigurableZones::Touchscreen4(_) => (
                "Zone 4",
                DisplayZone::TOUCHSCREEN_ZONE_4,
                ConfigurableZones::Touchscreen4(TouchscreenZoneInput::Pressed),
            ),
            _ => (
                "Unsupported Button",
                DisplayZone::DISPLAY_ZONE_UNSPECIFIED,
                ConfigurableZones::None,
            ),
        };

        let display_zone_config = widget::button("Set Image")
            .on_press(Messages::SetDisplayZoneImage(touchscreen_zone_mapping.1));

        let key_mapping_config_button = widget::button("On pressed").on_press(
            Messages::OpenInputMappingConfigurationPanel(touchscreen_zone_mapping.2),
        );

        column![
            widget::text!("{} config", touchscreen_zone_mapping.0),
            display_zone_config,
            key_mapping_config_button
        ]
        .into()
    }

    fn touchscreen_extra() -> widget::Row<'a, Messages> {
        let button =
            widget::button("Touchscreen Extra").on_press(Messages::OpenConfigurationPanel(
                ConfigurableZones::TouchscreenExtra(TouchscreenInput::None),
            ));

        row![button]
    }

    fn knob_row() -> widget::Row<'a, Messages> {
        (0..KNOB_COUNT).fold(row![], |row, i| {
            let control_label = match i {
                0 => ("Knob 1", ConfigurableZones::Knob1(KnobInput::None)),
                1 => ("Knob 2", ConfigurableZones::Knob2(KnobInput::None)),
                2 => ("Knob 3", ConfigurableZones::Knob3(KnobInput::None)),
                3 => ("Knob 4", ConfigurableZones::Knob4(KnobInput::None)),
                _ => ("Unsupported Knob", ConfigurableZones::None),
            };

            row.push(
                widget::button(control_label.0)
                    .on_press(Messages::OpenConfigurationPanel(control_label.1)),
            )
        })
    }
    fn knob_config_settings(zone: ConfigurableZones) -> widget::Column<'a, Messages> {
        let touchscreen_zone_mapping = match zone {
            ConfigurableZones::Knob1(_) => (
                "Knob 1",
                ConfigurableZones::Knob1(KnobInput::Clockwise),
                ConfigurableZones::Knob1(KnobInput::CounterClockwise),
                ConfigurableZones::Knob1(KnobInput::Pressed),
            ),
            ConfigurableZones::Knob2(_) => (
                "Knob 2",
                ConfigurableZones::Knob2(KnobInput::Clockwise),
                ConfigurableZones::Knob2(KnobInput::CounterClockwise),
                ConfigurableZones::Knob2(KnobInput::Pressed),
            ),
            ConfigurableZones::Knob3(_) => (
                "Knob 3",
                ConfigurableZones::Knob3(KnobInput::Clockwise),
                ConfigurableZones::Knob3(KnobInput::CounterClockwise),
                ConfigurableZones::Knob3(KnobInput::Pressed),
            ),
            ConfigurableZones::Knob4(_) => (
                "Knob 4",
                ConfigurableZones::Knob4(KnobInput::Clockwise),
                ConfigurableZones::Knob4(KnobInput::CounterClockwise),
                ConfigurableZones::Knob4(KnobInput::Pressed),
            ),
            _ => (
                "Unsupported Knob",
                ConfigurableZones::None,
                ConfigurableZones::None,
                ConfigurableZones::None,
            ),
        };

        let knob_clockwise_config = widget::button("Clockwise").on_press(
            Messages::OpenInputMappingConfigurationPanel(touchscreen_zone_mapping.1),
        );
        let knob_counter_clockwise_config = widget::button("Counter Clockwise").on_press(
            Messages::OpenInputMappingConfigurationPanel(touchscreen_zone_mapping.2),
        );
        let knob_pressed_config = widget::button("Pressed").on_press(
            Messages::OpenInputMappingConfigurationPanel(touchscreen_zone_mapping.3),
        );

        column![
            widget::text!("{} config", touchscreen_zone_mapping.0),
            knob_clockwise_config,
            knob_counter_clockwise_config,
            knob_pressed_config
        ]
        .into()
    }

    fn touchscreen_swipe_settings() -> widget::Column<'a, Messages> {
        let left_swipe_config_button =
            widget::button("On left swipe").on_press(Messages::OpenInputMappingConfigurationPanel(
                ConfigurableZones::TouchscreenExtra(TouchscreenInput::SwipeLeft),
            ));

        let right_swipe_config_button = widget::button("On right swipe").on_press(
            Messages::OpenInputMappingConfigurationPanel(ConfigurableZones::TouchscreenExtra(
                TouchscreenInput::SwipeRight,
            )),
        );

        column![row![left_swipe_config_button, right_swipe_config_button]].into()
    }
    pub fn view(
        &'_ self,
        brightness: u8,
        selected_config_zone: ConfigurableZones,
        current_key_sequence: Vec<(Key, Modifiers)>,
    ) -> iced::Element<'_, Messages> {
        let base = widget::container(
            column![
                widget::button("Clear all images").on_press(Messages::ClearAllDisplayZoneImages),
                widget::button("Pick boot logo").on_press(Messages::SetBootLogo),
                widget::slider(0..=100, brightness, Messages::SetBrightness),
                widget::container(column![
                    Self::button_grid_first_row(),
                    Self::button_grid_second_row(),
                ]),
                Self::touchscreen_zones_row(),
                Self::touchscreen_extra(),
                Self::knob_row()
            ]
            .spacing(10),
        )
        .center(Length::Fill)
        .padding(10);

        match selected_config_zone {
            ConfigurableZones::None => base.into(),
            _ => modal(
                base,
                match selected_config_zone {
                    ConfigurableZones::Button1(ref selected_input)
                    | ConfigurableZones::Button2(ref selected_input)
                    | ConfigurableZones::Button3(ref selected_input)
                    | ConfigurableZones::Button4(ref selected_input)
                    | ConfigurableZones::Button5(ref selected_input)
                    | ConfigurableZones::Button6(ref selected_input)
                    | ConfigurableZones::Button7(ref selected_input)
                    | ConfigurableZones::Button8(ref selected_input)
                    | ConfigurableZones::Button9(ref selected_input)
                    | ConfigurableZones::Button10(ref selected_input) => match selected_input {
                        // Open the capture modal
                        ButtonInput::Released | ButtonInput::Pressed => {
                            Self::input_capture_modal(selected_config_zone, current_key_sequence)
                        }
                        _ => Self::button_config_settings(selected_config_zone),
                    },
                    ConfigurableZones::Touchscreen1(ref selected_input)
                    | ConfigurableZones::Touchscreen2(ref selected_input)
                    | ConfigurableZones::Touchscreen3(ref selected_input)
                    | ConfigurableZones::Touchscreen4(ref selected_input) => match selected_input {
                        TouchscreenZoneInput::Pressed => {
                            Self::input_capture_modal(selected_config_zone, current_key_sequence)
                        }
                        _ => Self::touchscreen_zone_config_settings(selected_config_zone),
                    },
                    ConfigurableZones::TouchscreenExtra(ref selected_input) => match selected_input
                    {
                        TouchscreenInput::SwipeLeft | TouchscreenInput::SwipeRight => {
                            Self::input_capture_modal(selected_config_zone, current_key_sequence)
                        }
                        _ => Self::touchscreen_swipe_settings(),
                    },
                    ConfigurableZones::Knob1(ref selected_input)
                    | ConfigurableZones::Knob2(ref selected_input)
                    | ConfigurableZones::Knob3(ref selected_input)
                    | ConfigurableZones::Knob4(ref selected_input) => match selected_input {
                        KnobInput::CounterClockwise | KnobInput::Clockwise | KnobInput::Pressed => {
                            Self::input_capture_modal(selected_config_zone, current_key_sequence)
                        }
                        _ => Self::knob_config_settings(selected_config_zone),
                    },
                    _ => column![],
                },
                Messages::CloseConfigurationPanel,
            ),
        }
    }
}
