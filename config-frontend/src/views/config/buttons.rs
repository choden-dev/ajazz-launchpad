use crate::common::{ButtonInput, ConfigurableZones, ExtraConfigMode};
use crate::messages::Messages;
use crate::views::config::BUTTON_COUNT;
use crate::views::config::saved_config_display::{current_image, current_key_config};
use iced::widget;
use iced::widget::row;
use messaging::protos::display_zones::DisplayZone;
use messaging::protos::inputs::InputId;
use messaging::protos::key_config::KeyConfig;
use messaging::protos::server_config::DisplayImage;

pub fn button_grid_first_row<'a>() -> widget::Row<'a, Messages> {
    (0..BUTTON_COUNT / 2).fold(row![], |row, i| {
        let button_mapping = match i {
            0 => ("Button 1", ConfigurableZones::Button1(ButtonInput::None)),
            1 => ("Button 2", ConfigurableZones::Button2(ButtonInput::None)),
            2 => ("Button 3", ConfigurableZones::Button3(ButtonInput::None)),
            3 => ("Button 4", ConfigurableZones::Button4(ButtonInput::None)),
            4 => ("Button 5", ConfigurableZones::Button5(ButtonInput::None)),
            _ => ("Unsupported Button", ConfigurableZones::None),
        };

        let button = widget::button(button_mapping.0)
            .on_press(Messages::OpenConfigurationPanel(button_mapping.1));

        row.push(button)
    })
}
pub fn button_grid_second_row<'a>() -> widget::Row<'a, Messages> {
    (BUTTON_COUNT / 2..BUTTON_COUNT).fold(row![], |row, i| {
        let button_mapping = match i {
            5 => ("Button 6", ConfigurableZones::Button6(ButtonInput::None)),
            6 => ("Button 7", ConfigurableZones::Button7(ButtonInput::None)),
            7 => ("Button 8", ConfigurableZones::Button8(ButtonInput::None)),
            8 => ("Button 9", ConfigurableZones::Button9(ButtonInput::None)),
            9 => ("Button 10", ConfigurableZones::Button10(ButtonInput::None)),
            _ => ("Unsupported Button", ConfigurableZones::None),
        };

        let button = widget::button(button_mapping.0)
            .on_press(Messages::OpenConfigurationPanel(button_mapping.1));

        row.push(button)
    })
}
pub fn button_config_settings<'a>(
    button: ConfigurableZones,
    image_config: Vec<DisplayImage>,
    key_config: Vec<KeyConfig>,
) -> widget::Column<'a, Messages> {
    let button_mapping = match button {
        ConfigurableZones::Button1(_) => (
            "Button 1",
            DisplayZone::BUTTON_1,
            ConfigurableZones::Button1(ButtonInput::Pressed),
            ConfigurableZones::Button1(ButtonInput::Released),
        ),
        ConfigurableZones::Button2(_) => (
            "Button 2",
            DisplayZone::BUTTON_2,
            ConfigurableZones::Button2(ButtonInput::Pressed),
            ConfigurableZones::Button2(ButtonInput::Released),
        ),
        ConfigurableZones::Button3(_) => (
            "Button 3",
            DisplayZone::BUTTON_3,
            ConfigurableZones::Button3(ButtonInput::Pressed),
            ConfigurableZones::Button3(ButtonInput::Released),
        ),
        ConfigurableZones::Button4(_) => (
            "Button 4",
            DisplayZone::BUTTON_4,
            ConfigurableZones::Button4(ButtonInput::Pressed),
            ConfigurableZones::Button4(ButtonInput::Released),
        ),
        ConfigurableZones::Button5(_) => (
            "Button 5",
            DisplayZone::BUTTON_5,
            ConfigurableZones::Button5(ButtonInput::Pressed),
            ConfigurableZones::Button5(ButtonInput::Released),
        ),
        ConfigurableZones::Button6(_) => (
            "Button 6",
            DisplayZone::BUTTON_6,
            ConfigurableZones::Button6(ButtonInput::Pressed),
            ConfigurableZones::Button6(ButtonInput::Released),
        ),
        ConfigurableZones::Button7(_) => (
            "Button 7",
            DisplayZone::BUTTON_7,
            ConfigurableZones::Button7(ButtonInput::Pressed),
            ConfigurableZones::Button7(ButtonInput::Released),
        ),
        ConfigurableZones::Button8(_) => (
            "Button 8",
            DisplayZone::BUTTON_8,
            ConfigurableZones::Button8(ButtonInput::Pressed),
            ConfigurableZones::Button8(ButtonInput::Released),
        ),
        ConfigurableZones::Button9(_) => (
            "Button 9",
            DisplayZone::BUTTON_9,
            ConfigurableZones::Button9(ButtonInput::Pressed),
            ConfigurableZones::Button9(ButtonInput::Released),
        ),
        ConfigurableZones::Button10(_) => (
            "Button 10",
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

    let pressed_action_button =
        widget::button("On pressed").on_press(Messages::OpenInputMappingConfigurationPanel(
            button_mapping.2.clone(),
            ExtraConfigMode::KeyRecording,
        ));
    let released_action_button =
        widget::button("On released").on_press(Messages::OpenInputMappingConfigurationPanel(
            button_mapping.3.clone(),
            ExtraConfigMode::KeyRecording,
        ));

    let clear_image_button =
        widget::button("Clear Image").on_press(Messages::ClearDisplayZoneImage(button_mapping.1));

    iced::widget::column![
        title,
        button,
        current_key_config(&key_config, InputId::from(button_mapping.2)),
        pressed_action_button,
        current_key_config(&key_config, InputId::from(button_mapping.3)),
        released_action_button,
        current_image(&image_config, button_mapping.1),
        clear_image_button
    ]
}
