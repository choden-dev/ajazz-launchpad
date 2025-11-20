use crate::common::ConfigurableZones;
use crate::components::modal::modal;
use crate::messages::Messages;
use iced::widget::{column, row};
use iced::{Length, widget};
use messaging::protos::display_zones::DisplayZone;

pub struct Config;

const BUTTON_COUNT: u8 = 10;
const KNOB_COUNT: u8 = 4;

const TOUCHSCREEN_SWIPE_DIRECTIONS: u8 = 2;

const TOUCHSCREEN_ZONES: u8 = 4;

impl<'a> Config {
    fn button_grid_first_row() -> widget::Row<'a, Messages> {
        (0..BUTTON_COUNT / 2).fold(row![], |row, i| {
            let button_mapping = match i {
                0 => ("Button1", ConfigurableZones::Button1),
                1 => ("Button2", ConfigurableZones::Button2),
                2 => ("Button3", ConfigurableZones::Button3),
                3 => ("Button4", ConfigurableZones::Button4),
                4 => ("Button5", ConfigurableZones::Button5),
                _ => ("Unsupported Button", ConfigurableZones::None),
            };

            let button = widget::button(button_mapping.0)
                .on_press(Messages::OpenConfigurationPanel(button_mapping.1));

            row.push(button)
        })
    }
    fn button_config_settings(button: ConfigurableZones) -> widget::Column<'a, Messages> {
        let button_mapping = match button {
            ConfigurableZones::Button1 => ("Button1", DisplayZone::BUTTON_1),
            ConfigurableZones::Button2 => ("Button2", DisplayZone::BUTTON_2),
            ConfigurableZones::Button3 => ("Button3", DisplayZone::BUTTON_3),
            ConfigurableZones::Button4 => ("Button4", DisplayZone::BUTTON_4),
            ConfigurableZones::Button5 => ("Button5", DisplayZone::BUTTON_5),
            ConfigurableZones::Button6 => ("Button6", DisplayZone::BUTTON_6),
            ConfigurableZones::Button7 => ("Button7", DisplayZone::BUTTON_7),
            ConfigurableZones::Button8 => ("Button8", DisplayZone::BUTTON_8),
            ConfigurableZones::Button9 => ("Button9", DisplayZone::BUTTON_9),
            ConfigurableZones::Button10 => ("Button10", DisplayZone::BUTTON_10),
            _ => ("Unsupported Button", DisplayZone::DISPLAY_ZONE_UNSPECIFIED),
        };

        let button = widget::button(button_mapping.0)
            .on_press(Messages::SetDisplayZoneImage(button_mapping.1));

        column![button]
    }
    fn button_grid_second_row() -> widget::Row<'a, Messages> {
        (BUTTON_COUNT / 2..BUTTON_COUNT).fold(row![], |row, i| {
            let button_mapping = match i {
                5 => ("Button1", ConfigurableZones::Button6),
                6 => ("Button2", ConfigurableZones::Button7),
                7 => ("Button3", ConfigurableZones::Button8),
                8 => ("Button4", ConfigurableZones::Button9),
                9 => ("Button5", ConfigurableZones::Button10),
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
                0 => ("Zone 1", DisplayZone::TOUCHSCREEN_ZONE_1),
                1 => ("Zone 2", DisplayZone::TOUCHSCREEN_ZONE_2),
                2 => ("Zone 3", DisplayZone::TOUCHSCREEN_ZONE_3),
                3 => ("Zone 4", DisplayZone::TOUCHSCREEN_ZONE_4),
                _ => ("Unsupported Button", DisplayZone::DISPLAY_ZONE_UNSPECIFIED),
            };

            let button = widget::button(touchscreen_zone_mapping.0)
                .on_press(Messages::SetDisplayZoneImage(touchscreen_zone_mapping.1));

            row.push(button)
        })
    }

    fn touchscreen_swipe_row() -> widget::Row<'a, Messages> {
        (0..TOUCHSCREEN_SWIPE_DIRECTIONS).fold(row![], |row, i| {
            let button_text = match i {
                0 => "Swiped Left",
                1 => "Swiped Right",
                _ => "Unsupported Button",
            };

            let button = widget::button(button_text).on_press(Messages::ClearAllDisplayZoneImages);

            row.push(button)
        })
    }

    fn knob_row() -> widget::Row<'a, Messages> {
        (0..KNOB_COUNT).fold(row![], |row, i| {
            let control_label = match i {
                0 => "Knob 1",
                1 => "Knob 2",
                2 => "Knob 3",
                3 => "Knob 4",
                _ => "Unsupported Knob",
            };

            let item = widget::container(column![
                widget::text(control_label),
                row![widget::button("CCW"), widget::button("CW")]
            ]);

            row.push(item)
        })
    }
    pub fn view(
        &'_ self,
        brightness: u8,
        selected_config_zone: ConfigurableZones,
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
                Self::touchscreen_swipe_row(),
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
                    ConfigurableZones::Button1
                    | ConfigurableZones::Button2
                    | ConfigurableZones::Button3
                    | ConfigurableZones::Button4
                    | ConfigurableZones::Button5
                    | ConfigurableZones::Button6
                    | ConfigurableZones::Button7
                    | ConfigurableZones::Button8
                    | ConfigurableZones::Button9
                    | ConfigurableZones::Button10 => {
                        Self::button_config_settings(selected_config_zone)
                    }
                    _ => todo!(),
                },
                Messages::CloseConfigurationPanel,
            ),
        }
    }
}
