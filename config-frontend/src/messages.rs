use crate::common::{ConfigurableZones, ExtraConfigMode, KeyConfigOptions};
use iced::keyboard::{Key, Modifiers};
use messaging::protos::display_zones::DisplayZone;
use messaging::protos::inputs::InputId;
use messaging::protos::key_config::command_action::Command;
use messaging::protos::server_config::ServerConfig;

#[derive(Debug, Clone)]
pub enum Messages {
    SetKeyConfig(InputId, Vec<KeyConfigOptions>),
    SetDisplayZoneImage(DisplayZone),
    ClearDisplayZoneImage(DisplayZone),
    ClearAllDisplayZoneImages,
    SetBootLogo,
    SetBrightness(u8),

    InitialiseBackend,
    BackendInitialised,
    RequestBackendConfig,
    BackendConfigUpdated(Option<ServerConfig>),

    OpenConfigurationPanel(ConfigurableZones),
    OpenInputMappingConfigurationPanel(ConfigurableZones, ExtraConfigMode),
    CloseConfigurationPanel,

    ResetInputBuffer,
    ClearCommandInput,

    CommandInputChanged(String),
    CommandAdded(Command),
    KeyboardInput(Key, Modifiers),
    RemoveAction(usize),

    Tick,

    Noop,
}
