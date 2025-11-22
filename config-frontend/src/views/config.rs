mod buttons;
mod knobs;
mod touchscreen;

use crate::common::{
    ButtonInput, ConfigurableZones, KnobInput, TouchscreenInput, TouchscreenZoneInput,
};
use crate::components::modal::modal;
use crate::messages::Messages;
use crate::views::config::buttons::{
    button_config_settings, button_grid_first_row, button_grid_second_row,
};
use crate::views::config::knobs::{knob_config_settings, knob_row};
use crate::views::config::touchscreen::{
    touchscreen_extra, touchscreen_swipe_settings, touchscreen_zone_config_settings,
    touchscreen_zones_row,
};
use iced::keyboard::{Key, Modifiers};
use iced::widget::column;
use iced::{Length, widget};
use messaging::protos::inputs::InputId;

pub struct Config;

const BUTTON_COUNT: u8 = 10;
const KNOB_COUNT: u8 = 4;

const TOUCHSCREEN_ZONES: u8 = 4;

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
                widget::container(column![button_grid_first_row(), button_grid_second_row(),]),
                touchscreen_zones_row(),
                touchscreen_extra(),
                knob_row()
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
                        _ => button_config_settings(selected_config_zone),
                    },
                    ConfigurableZones::Touchscreen1(ref selected_input)
                    | ConfigurableZones::Touchscreen2(ref selected_input)
                    | ConfigurableZones::Touchscreen3(ref selected_input)
                    | ConfigurableZones::Touchscreen4(ref selected_input) => match selected_input {
                        TouchscreenZoneInput::Pressed => {
                            Self::input_capture_modal(selected_config_zone, current_key_sequence)
                        }
                        _ => touchscreen_zone_config_settings(selected_config_zone),
                    },
                    ConfigurableZones::TouchscreenExtra(ref selected_input) => match selected_input
                    {
                        TouchscreenInput::SwipeLeft | TouchscreenInput::SwipeRight => {
                            Self::input_capture_modal(selected_config_zone, current_key_sequence)
                        }
                        _ => touchscreen_swipe_settings(),
                    },
                    ConfigurableZones::Knob1(ref selected_input)
                    | ConfigurableZones::Knob2(ref selected_input)
                    | ConfigurableZones::Knob3(ref selected_input)
                    | ConfigurableZones::Knob4(ref selected_input) => match selected_input {
                        KnobInput::CounterClockwise | KnobInput::Clockwise | KnobInput::Pressed => {
                            Self::input_capture_modal(selected_config_zone, current_key_sequence)
                        }
                        _ => knob_config_settings(selected_config_zone),
                    },
                    _ => column![],
                },
                Messages::CloseConfigurationPanel,
            ),
        }
    }
}
