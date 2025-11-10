use std::io::{Error, ErrorKind};

pub enum IncomingCommands {
    SetKeyConfig,
    SetDisplayZoneImage,
    ClearDisplayZoneImage,
    ClearAllDisplayZoneImages,
    SetBootLogo,
    SetBrightness,
}

impl TryFrom<&str> for IncomingCommands {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "SetKeyConfig" => Ok(IncomingCommands::SetKeyConfig),
            "SetDisplayZoneImage" => Ok(IncomingCommands::SetDisplayZoneImage),
            "ClearDisplayZoneImage" => Ok(IncomingCommands::ClearDisplayZoneImage),
            "ClearAllDisplayZoneImages" => Ok(IncomingCommands::ClearAllDisplayZoneImages),
            "SetBootLogo" => Ok(IncomingCommands::SetBootLogo),
            "SetBrightness" => Ok(IncomingCommands::SetBrightness),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Unknown incoming command: {}", value),
            )),
        }
    }
}
