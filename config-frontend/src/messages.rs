use crate::common::ConfigurableZones;
use messaging::protos::display_zones::DisplayZone;
use messaging::protos::inputs::InputId;
use messaging::protos::key_config::KeyAction;

#[derive(Debug, Clone)]
pub enum Messages {
    SetKeyConfig(InputId, Vec<KeyAction>),
    SetDisplayZoneImage(DisplayZone),
    ClearDisplayZoneImage(DisplayZone),
    ClearAllDisplayZoneImages,
    SetBootLogo,
    SetBrightness(u8),

    InitialiseBackend,
    BackendInitialised,

    OpenConfigurationPanel(ConfigurableZones),
    CloseConfigurationPanel,

    Tick,
}
