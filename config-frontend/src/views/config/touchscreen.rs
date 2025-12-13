use crate::common::{ConfigurableZones, ExtraConfigMode, TouchscreenInput, TouchscreenZoneInput};
use crate::messages::Messages;
use crate::views::config::TOUCHSCREEN_ZONES;
use crate::views::config::saved_config_display::{current_image, current_key_config};
use iced::widget;
use iced::widget::row;
use messaging::protos::display_zones::DisplayZone;
use messaging::protos::inputs::InputId;
use messaging::protos::key_config::KeyConfig;
use messaging::protos::server_config::DisplayImage;

pub fn touchscreen_zones_row<'a>() -> widget::Row<'a, Messages> {
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

pub fn touchscreen_zone_config_settings<'a>(
    zone: ConfigurableZones,
    image_config: Vec<DisplayImage>,
    key_config: Vec<KeyConfig>,
) -> widget::Column<'a, Messages> {
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

    let key_mapping_config_button =
        widget::button("On pressed").on_press(Messages::OpenInputMappingConfigurationPanel(
            touchscreen_zone_mapping.2.clone(),
            ExtraConfigMode::KeyRecording,
        ));

    let clear_image_button = widget::button("Clear Image")
        .on_press(Messages::ClearDisplayZoneImage(touchscreen_zone_mapping.1));

    iced::widget::column![
        widget::text!("{} config", touchscreen_zone_mapping.0),
        display_zone_config,
        current_key_config(&key_config, InputId::from(touchscreen_zone_mapping.2)),
        key_mapping_config_button,
        current_image(&image_config, touchscreen_zone_mapping.1),
        clear_image_button
    ]
}

pub fn touchscreen_extra<'a>() -> widget::Row<'a, Messages> {
    let button = widget::button("Touchscreen Extra").on_press(Messages::OpenConfigurationPanel(
        ConfigurableZones::TouchscreenExtra(TouchscreenInput::None),
    ));

    row![button]
}
pub fn touchscreen_swipe_settings<'a>(key_config: Vec<KeyConfig>) -> widget::Column<'a, Messages> {
    let left_swipe_config_button =
        widget::button("On left swipe").on_press(Messages::OpenInputMappingConfigurationPanel(
            ConfigurableZones::TouchscreenExtra(TouchscreenInput::SwipeLeft),
            ExtraConfigMode::KeyRecording,
        ));

    let right_swipe_config_button =
        widget::button("On right swipe").on_press(Messages::OpenInputMappingConfigurationPanel(
            ConfigurableZones::TouchscreenExtra(TouchscreenInput::SwipeRight),
            ExtraConfigMode::KeyRecording,
        ));

    iced::widget::column![row![
        current_key_config(&key_config, InputId::TOUCHSCREEN_SWIPED_LEFT),
        left_swipe_config_button,
        current_key_config(&key_config, InputId::TOUCHSCREEN_SWIPED_RIGHT),
        right_swipe_config_button
    ]]
}
