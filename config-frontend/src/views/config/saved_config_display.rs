use crate::messages::Messages;
use iced::widget;
use iced::widget::column;
use messaging::protos::display_zones::DisplayZone;
use messaging::protos::inputs::InputId;
use messaging::protos::key_config::KeyConfig;
use messaging::protos::server_config::DisplayImage;

pub fn current_image<'a>(
    images: &[DisplayImage],
    display_zone: DisplayZone,
) -> iced::Element<'a, Messages> {
    let current_image = images.iter().find(|image| {
        image
            .display_zone
            .enum_value_or(DisplayZone::DISPLAY_ZONE_UNSPECIFIED)
            == display_zone
    });
    let display_text = widget::text!(
        "{}",
        current_image.map_or("Not specified", |image| { image.path.as_str() })
    );

    if let Some(display_image) = current_image {
        let image = widget::image(display_image.path.as_str());
        return column![display_text, image].into();
    }

    column![display_text].into()
}

pub fn current_key_config<'a>(
    key_configs: &[KeyConfig],
    input_action: InputId,
) -> iced::Element<'a, Messages> {
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
    .into()
}
