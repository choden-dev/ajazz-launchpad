mod common;
mod components;
mod mappers;
mod messages;
mod tasks;
mod views;

use crate::common::{ConfigurableZones, ExtraConfigMode};
use crate::mappers::{ProtoKeyActionWrapper};
use crate::messages::Messages;
use crate::tasks::{connect_to_backend, select_image_blocking};
use iced::keyboard::{Key, Modifiers};
use iced::task::Task;
use iced::{Element, Subscription};
use messaging::client_wrapper::{ClientCommands, ClientWrapper};
use std::cmp::PartialEq;
use std::time::Duration;

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
    connecting_to_backend: bool,
    brightness: u8,
    current_input_sequence: Vec<(Key, Modifiers)>,
}

impl LaunchpadConfigApp {
    fn get_client(&mut self) -> Option<&mut ClientWrapper> {
        match self.socket_client {
            Some(ref mut client) => Some(client),
            None => None,
        }
    }
}

fn view(application_state: &'_ LaunchpadConfigApp) -> Element<'_, Messages> {
    match &application_state.view {
        View::Initialise => views::initialise::Initialise.view(),
        View::Configure(modal_zone, _) => views::config::Config.view(
            application_state.brightness,
            modal_zone.clone(),
            application_state.current_input_sequence.clone(),
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
        Messages::Tick => match application_state.socket_client {
            None => {
                application_state.connecting_to_backend = true;
                return Task::done(Messages::InitialiseBackend);
            }
            _ => {}
        },
        Messages::InitialiseBackend => {
            application_state.socket_client = Some(connect_to_backend());
            return Task::done(Messages::BackendInitialised);
        }
        Messages::BackendInitialised => {
            application_state.connecting_to_backend = false;
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
                client.set_boot_logo(String::from(absolute_path)).ok();
            }
        }

        Messages::SetDisplayZoneImage(display_zone) => {
            let selected_image = select_image_blocking();

            if let Some(absolute_path) = selected_image
                && let Some(client) = application_state.get_client()
            {
                client
                    .set_display_zone_image(display_zone, String::from(absolute_path))
                    .ok();
            }
        }

        Messages::OpenConfigurationPanel(zone) => {
            application_state.view = View::Configure(zone, ExtraConfigMode::Default);
        }

        Messages::OpenInputMappingConfigurationPanel(zone) => {
            application_state.view = View::Configure(zone, ExtraConfigMode::KeyRecording);
            // We need a new set of inputs every time the panel is opened
            return Task::done(Messages::ResetInputBuffer);
        }

        Messages::CloseConfigurationPanel => {
            application_state.view =
                View::Configure(ConfigurableZones::None, ExtraConfigMode::Default);
        }

        Messages::ResetInputBuffer => {
            application_state.current_input_sequence.clear();
        }

        Messages::KeyboardInput(key, modifier) => {
            match application_state.view {
                View::Configure(ConfigurableZones::None, _) => {
                    // Do nothing if we aren't in a configurable state
                }
                View::Configure(_, ref config_mode) => {
                    match config_mode {
                        ExtraConfigMode::Default => {
                            // Default mode does not need to capture input
                        }
                        _ => application_state
                            .current_input_sequence
                            .push((key, modifier)),
                    }
                }
                _ => {}
            };
        }
        Messages::SetKeyConfig(input_id, sequence) => {
            let key_actions = sequence
                .iter()
                .map(|mapping| ProtoKeyActionWrapper::from(mapping.clone()).key_action())
                .collect::<Vec<_>>();

            let builder = messaging::proto_builders::KeyConfigActionBuilder::from(key_actions);

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
