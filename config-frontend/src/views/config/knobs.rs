use crate::common::{ConfigurableZones, KnobInput};
use crate::messages::Messages;
use crate::views::config::KNOB_COUNT;
use iced::widget;
use iced::widget::row;

pub fn knob_row<'a>() -> widget::Row<'a, Messages> {
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
pub fn knob_config_settings<'a>(zone: ConfigurableZones) -> widget::Column<'a, Messages> {
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

    iced::widget::column![
        widget::text!("{} config", touchscreen_zone_mapping.0),
        knob_clockwise_config,
        knob_counter_clockwise_config,
        knob_pressed_config
    ]
}
