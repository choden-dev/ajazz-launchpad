mod common;
mod components;
mod mappers;
mod messages;
mod tasks;
mod views;

use crate::common::{ConfigurableZones, ExtraConfigMode, KeyConfigOptions};
use crate::mappers::ProtoKeyActionWrapper;
use crate::messages::Messages;
use crate::tasks::{connect_to_backend, select_image_blocking};
use iced::task::Task;
use iced::{Element, Subscription};
use messaging::client_wrapper::{ClientCommands, ClientWrapper};
use messaging::protos::key_config::command_action::Command;
use messaging::protos::server_config::ServerConfig;
use std::cmp::PartialEq;
use std::time::Duration;

#[allow(dead_code)]
#[derive(Default, PartialEq)]
enum View {
    #[default]
    Initialise,
    Configure(ConfigurableZones, ExtraConfigMode),
    Settings,
}

#[derive(Default)]
struct LaunchpadConfigApp {
    view: View,
    socket_client: Option<ClientWrapper>,
    /// Used to determine if we need to try and connect to server again
    connecting_to_backend: bool,
    brightness: u8,
    current_input_sequence: Vec<KeyConfigOptions>,
    current_command_input_value: String,
    current_server_config: Option<ServerConfig>,
}

impl LaunchpadConfigApp {
    fn get_client(&mut self) -> Option<&mut ClientWrapper> {
        self.socket_client.as_mut()
    }
}

fn view(application_state: &'_ LaunchpadConfigApp) -> Element<'_, Messages> {
    match &application_state.view {
        View::Initialise => views::initialise::Initialise.view(),
        View::Configure(modal_zone, mode) => views::config::Config.view(
            application_state.brightness,
            modal_zone.to_owned(),
            application_state.current_input_sequence.to_owned(),
            mode.to_owned(),
            application_state.current_command_input_value.to_owned(),
            application_state
                .current_server_config
                .to_owned()
                .map_or_else(ServerConfig::default, |cfg| cfg),
        ),
        _ => todo!(),
    }
}

fn subscriptions(_: &LaunchpadConfigApp) -> Subscription<Messages> {
    let tick_subscription = iced::time::every(Duration::from_secs(2)).map(|_| Messages::Tick);
    let keyboard_subscription = iced::keyboard::on_key_press::<Messages>(|key, modifier| {
        Some(Messages::KeyboardInput(key, modifier))
    });
    let subscriptions = vec![tick_subscription, keyboard_subscription];

    Subscription::batch(subscriptions)
}

fn update(application_state: &mut LaunchpadConfigApp, message: Messages) -> Task<Messages> {
    match message {
        Messages::Tick => {
            if application_state.socket_client.is_none() {
                application_state.connecting_to_backend = true;
                return Task::done(Messages::InitialiseBackend);
            }
        }
        Messages::InitialiseBackend => {
            application_state.socket_client = Some(connect_to_backend());
            if let Some(client) = application_state.get_client() {
                client.request_server_config().ok();
                let current_config = client.check_for_server_config().ok();
                if let Some(current_config) = current_config {
                    application_state.brightness = current_config.brightness as u8;
                    return Task::done(Messages::BackendInitialised(Some(current_config)));
                }
            }

            return Task::done(Messages::BackendInitialised(None));
        }
        Messages::BackendInitialised(config) => {
            application_state.connecting_to_backend = false;
            application_state.current_server_config = config;
            application_state.view =
                View::Configure(ConfigurableZones::None, ExtraConfigMode::Default);
        }

        Messages::SetBrightness(new_brightness) => {
            application_state.brightness = new_brightness;
            if let Some(client) = application_state.get_client() {
                client.set_brightness(new_brightness).ok();
            }
        }

        Messages::ClearAllDisplayZoneImages => {
            if let Some(client) = application_state.get_client() {
                client.clear_all_images(true).ok();
            }
        }

        Messages::ClearDisplayZoneImage(display_zone) => {
            if let Some(client) = application_state.get_client() {
                client.clear_display_zone_image(display_zone).ok();
            }
        }

        Messages::SetBootLogo => {
            let selected_image = select_image_blocking();

            if let Some(absolute_path) = selected_image
                && let Some(client) = application_state.get_client()
            {
                client.set_boot_logo(absolute_path).ok();
            }
        }

        Messages::SetDisplayZoneImage(display_zone) => {
            let selected_image = select_image_blocking();

            if let Some(absolute_path) = selected_image
                && let Some(client) = application_state.get_client()
            {
                client
                    .set_display_zone_image(display_zone, absolute_path)
                    .ok();
            }
        }

        Messages::OpenConfigurationPanel(zone) => {
            application_state.view = View::Configure(zone, ExtraConfigMode::Default);
        }

        Messages::OpenInputMappingConfigurationPanel(zone, mode) => match mode {
            ExtraConfigMode::Command | ExtraConfigMode::KeyRecording => {
                application_state.view = View::Configure(zone, mode);
            }
            _ => (),
        },

        Messages::RemoveAction(index) => {
            if application_state
                .current_input_sequence
                .get(index)
                .is_some()
            {
                application_state.current_input_sequence.remove(index);
            }
        }

        Messages::CloseConfigurationPanel => {
            application_state.view =
                View::Configure(ConfigurableZones::None, ExtraConfigMode::Default);
            // We need a new set of inputs every time the panel is opened
            return Task::done(Messages::ResetInputBuffer);
        }

        Messages::ResetInputBuffer => {
            application_state.current_input_sequence.clear();
        }

        Messages::ClearCommandInput => {
            application_state.current_command_input_value.clear();
        }

        Messages::KeyboardInput(key, modifier) => {
            match application_state.view {
                View::Configure(ConfigurableZones::None, _) => {
                    // Do nothing if we aren't in a configurable state
                }
                View::Configure(_, ref config_mode) => {
                    match config_mode {
                        ExtraConfigMode::Default | ExtraConfigMode::Command => {
                            // Default or command mode does not need to capture input
                        }
                        _ => application_state
                            .current_input_sequence
                            .push(KeyConfigOptions::Key((key, modifier))),
                    }
                }
                _ => {}
            };
        }
        Messages::CommandInputChanged(text) => {
            application_state.current_command_input_value = text;
        }
        Messages::CommandAdded(command) => {
            if let Command::FreeformCommand(ref freeform) = command {
                application_state.current_command_input_value = freeform.clone().command;
            }
            application_state
                .current_input_sequence
                .push(KeyConfigOptions::Command(command));
            return Task::done(Messages::ClearCommandInput);
        }
        Messages::SetKeyConfig(input_id, sequence) => {
            let mut builder = messaging::proto_builders::KeyConfigActionBuilder::new();
            sequence.iter().for_each(|action| match action {
                common::KeyConfigOptions::Key(key_action) => {
                    builder.add_prebuilt_key_action(
                        ProtoKeyActionWrapper::from(key_action.to_owned()).key_action(),
                    );
                }
                common::KeyConfigOptions::Command(command_action) => {
                    builder.add_command_action(command_action.to_owned())
                }
            });

            if let Some(client) = application_state.get_client() {
                client
                    .send_key_config(input_id, builder.actions().to_owned())
                    .ok();
            }

            return Task::done(Messages::ResetInputBuffer);
        }
    }
    Task::none()
}

pub fn main() -> iced::Result {
    iced::application("Launchpad Config", update, view)
        .subscription(subscriptions)
        .run()
}
