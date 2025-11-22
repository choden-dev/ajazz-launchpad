use crate::common::ConfigurableZones;
use iced::keyboard::{Key, Modifiers};
use messaging::protos::display_zones::DisplayZone;
use messaging::protos::inputs::InputId;

#[derive(Debug, Clone)]
pub enum Messages {
    SetKeyConfig(InputId, Vec<(Key, Modifiers)>),
    SetDisplayZoneImage(DisplayZone),
    ClearDisplayZoneImage(DisplayZone),
    ClearAllDisplayZoneImages,
    SetBootLogo,
    SetBrightness(u8),

    InitialiseBackend,
    BackendInitialised,

    OpenConfigurationPanel(ConfigurableZones),
    OpenInputMappingConfigurationPanel(ConfigurableZones),
    CloseConfigurationPanel,

    ResetInputBuffer,

    KeyboardInput(Key, Modifiers),

    Tick,
}
