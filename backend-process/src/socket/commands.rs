use crate::database::models::ImageMapping;
use crate::input_handler::InputMapping;

pub enum IncomingCommands {
    SetKeyConfig(InputMapping),
    SetDisplayZoneImage,
    ClearDisplayZoneImage,
    ClearAllDisplayZoneImages(ImageMapping),
    SetBootLogo,
    SetBrightness,
}
