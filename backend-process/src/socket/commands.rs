use crate::database::models::ImageMapping;
use crate::input_handler::InputMapping;
use firmware_api::display_zones::DisplayZones;

pub enum IncomingCommands {
    SetKeyConfig(InputMapping),
    SetDisplayZoneImage(ImageMapping),
    ClearDisplayZoneImage(DisplayZones),
    ClearAllDisplayZoneImages,
    SetBootLogo(String),
    SetBrightness(u8),
}
