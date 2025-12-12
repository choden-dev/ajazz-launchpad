use iced::widget;
use messaging::protos::display_zones::DisplayZone;
use messaging::protos::inputs::InputId;
use messaging::protos::key_config::KeyConfig;
use messaging::protos::server_config::DisplayImage;

pub fn current_image<'a>(images: &[DisplayImage], display_zone: DisplayZone) -> widget::Text<'a> {
    let current_image = images.iter().find(|image| {
        image
            .display_zone
            .enum_value_or(DisplayZone::DISPLAY_ZONE_UNSPECIFIED)
            == display_zone
    });
    widget::text!(
        "{}",
        current_image.map_or("Not specified", |image| { image.path.as_str() })
    )
}

pub fn current_key_config<'a>(
    key_configs: &[KeyConfig],
    input_action: InputId,
) -> widget::Text<'a> {
    let current_key_config = key_configs.iter().find(|key_config| {
        key_config
            .input_id
            .enum_value_or(InputId::INPUT_ACTION_UNSPECIFIED)
            == input_action
    });

    widget::text!(
        "{}",
        current_key_config.map_or(String::from("Not specified"), |key_config| {
            format!("{:?}", key_config.actions)
        })
    )
}
