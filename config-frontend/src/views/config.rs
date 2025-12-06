mod buttons;
mod knobs;
mod touchscreen;

use crate::common::{
    ButtonInput, ConfigurableZones, ExtraConfigMode, KeyConfigOptions, KnobInput, TouchscreenInput,
    TouchscreenZoneInput,
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
use iced::widget::{column, row};
use iced::{Length, widget};
use messaging::protos::inputs::InputId;
use messaging::protos::key_config::FreeformCommand;
use messaging::protos::key_config::command_action::Command;

pub struct Config;

const BUTTON_COUNT: u8 = 10;
const KNOB_COUNT: u8 = 4;

const TOUCHSCREEN_ZONES: u8 = 4;

impl<'a> Config {
    fn input_capture_modal(
        input: ConfigurableZones,
        current_key_sequence: Vec<KeyConfigOptions>,
        current_mode: ExtraConfigMode,
        current_command_input: String, // TODO: extend to support undo and command actions
    ) -> widget::Column<'a, Messages> {
        let options = row![
            widget::radio(
                "Key",
                ExtraConfigMode::KeyRecording,
                Some(current_mode),
                |value| { Messages::OpenInputMappingConfigurationPanel(input.clone(), value) }
            ),
            widget::radio(
                "Command",
                ExtraConfigMode::Command,
                Some(current_mode),
                |value| { Messages::OpenInputMappingConfigurationPanel(input.clone(), value) }
            ),
        ];

        let input_id: InputId = input.into();

        let actions_display: widget::Column<'a, Messages> = current_key_sequence
            .iter()
            .enumerate()
            .fold(column![], |item, (index, action)| {
                item.push(
                    widget::button(widget::text(format!("{:?}", action)))
                        .on_press(Messages::RemoveAction(index)),
                )
            });

        let submit_button = widget::button("OK").on_press(Messages::SetKeyConfig(
            input_id,
            current_key_sequence
                .iter()
                .map(|mapping| match mapping {
                    KeyConfigOptions::Key(key) => KeyConfigOptions::Key(key.to_owned()),
                    KeyConfigOptions::Command(command) => {
                        KeyConfigOptions::Command(command.to_owned())
                    }
                })
                .collect(),
        ));

        let reset_button = widget::button("Reset").on_press(Messages::ResetInputBuffer);

        let ctas = row![submit_button, reset_button];

        if current_mode == ExtraConfigMode::Command {
            let command_input = widget::text_input("Input command", &current_command_input)
                .on_input(Messages::CommandInputChanged);
            let add_command_button = widget::button("Add Command").on_press(
                Messages::CommandAdded(Command::FreeformCommand(FreeformCommand {
                    command: current_command_input.clone(),
                    ..FreeformCommand::default()
                })),
            );
            return column![
                options,
                actions_display,
                command_input,
                add_command_button,
                ctas
            ];
        }
        column![options, actions_display, ctas]
    }

    pub fn view(
        &'_ self,
        brightness: u8,
        selected_config_zone: ConfigurableZones,
        current_key_sequence: Vec<KeyConfigOptions>,
        selected_mode: ExtraConfigMode,
        current_command_input: String,
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
                        ButtonInput::Released | ButtonInput::Pressed => Self::input_capture_modal(
                            selected_config_zone,
                            current_key_sequence,
                            selected_mode,
                            current_command_input,
                        ),
                        _ => button_config_settings(selected_config_zone),
                    },
                    ConfigurableZones::Touchscreen1(ref selected_input)
                    | ConfigurableZones::Touchscreen2(ref selected_input)
                    | ConfigurableZones::Touchscreen3(ref selected_input)
                    | ConfigurableZones::Touchscreen4(ref selected_input) => match selected_input {
                        TouchscreenZoneInput::Pressed => Self::input_capture_modal(
                            selected_config_zone,
                            current_key_sequence,
                            selected_mode,
                            current_command_input,
                        ),
                        _ => touchscreen_zone_config_settings(selected_config_zone),
                    },
                    ConfigurableZones::TouchscreenExtra(ref selected_input) => match selected_input
                    {
                        TouchscreenInput::SwipeLeft | TouchscreenInput::SwipeRight => {
                            Self::input_capture_modal(
                                selected_config_zone,
                                current_key_sequence,
                                selected_mode,
                                current_command_input,
                            )
                        }
                        _ => touchscreen_swipe_settings(),
                    },
                    ConfigurableZones::Knob1(ref selected_input)
                    | ConfigurableZones::Knob2(ref selected_input)
                    | ConfigurableZones::Knob3(ref selected_input)
                    | ConfigurableZones::Knob4(ref selected_input) => match selected_input {
                        KnobInput::CounterClockwise | KnobInput::Clockwise | KnobInput::Pressed => {
                            Self::input_capture_modal(
                                selected_config_zone,
                                current_key_sequence,
                                selected_mode,
                                current_command_input,
                            )
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
