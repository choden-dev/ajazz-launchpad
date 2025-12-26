use crate::common::{ConfigurableZones, ExtraConfigMode, KnobInput};
use crate::messages::Messages;
use crate::views::config::KNOB_COUNT;
use crate::views::config::saved_config_display::current_key_config;
use iced::widget::row;
use iced::{Length, widget};
use messaging::protos::inputs::InputId;
use messaging::protos::key_config::KeyConfig;

pub fn knob_row<'a>() -> widget::Row<'a, Messages> {
    (0..KNOB_COUNT).fold(row![], |row, i| {
        let control_label = match i {
            0 => ("Knob 1", ConfigurableZones::Knob1(KnobInput::None)),
            1 => ("Knob 2", ConfigurableZones::Knob2(KnobInput::None)),
            2 => ("Knob 3", ConfigurableZones::Knob3(KnobInput::None)),
            3 => ("Knob 4", ConfigurableZones::Knob4(KnobInput::None)),
            _ => ("Unsupported Knob", ConfigurableZones::None),
        };

        let is_first = i == 0;
        let is_last = i == KNOB_COUNT - 1;

        let button = widget::button(control_label.0)
            .on_press(Messages::OpenConfigurationPanel(control_label.1));

        let row_with_spacer = row
            .push(iced::widget::column![].width(Length::FillPortion(if is_first { 3 } else { 1 })));
        let row_with_button = row_with_spacer.push(button.width(Length::FillPortion(4)));

        if is_last {
            row_with_button.push(iced::widget::column![].width(Length::FillPortion(3)))
        } else {
            row_with_button
        }
    })
}
pub fn knob_config_settings<'a>(
    zone: ConfigurableZones,
    key_config: Vec<KeyConfig>,
) -> widget::Column<'a, Messages> {
    let knob_mapping = match zone {
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

    let knob_clockwise_config =
        widget::button("Clockwise").on_press(Messages::OpenInputMappingConfigurationPanel(
            knob_mapping.1.clone(),
            ExtraConfigMode::KeyRecording,
        ));
    let knob_counter_clockwise_config =
        widget::button("Counter Clockwise").on_press(Messages::OpenInputMappingConfigurationPanel(
            knob_mapping.2.clone(),
            ExtraConfigMode::KeyRecording,
        ));
    let knob_pressed_config =
        widget::button("Pressed").on_press(Messages::OpenInputMappingConfigurationPanel(
            knob_mapping.3.clone(),
            ExtraConfigMode::KeyRecording,
        ));

    iced::widget::column![
        widget::text!("{} config", knob_mapping.0),
        current_key_config(&key_config, InputId::from(knob_mapping.1)),
        knob_clockwise_config,
        current_key_config(&key_config, InputId::from(knob_mapping.2)),
        knob_counter_clockwise_config,
        current_key_config(&key_config, InputId::from(knob_mapping.3)),
        knob_pressed_config
    ]
}
