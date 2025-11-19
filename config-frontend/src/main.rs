mod messages;
mod tasks;
mod views;

use crate::messages::Messages;
use crate::tasks::{connect_to_backend, select_image_blocking};
use iced::task::Task;
use iced::{Element, Subscription};
use messaging::client_wrapper::{ClientCommands, ClientWrapper};
use std::cmp::PartialEq;
use std::time::Duration;

#[derive(Default, PartialEq)]
enum View {
    #[default]
    Initialise,
    Configure,
    Settings,
}

#[derive(Default)]
struct LaunchpadConfigApp {
    view: View,
    socket_client: Option<ClientWrapper>,
    connecting_to_backend: bool,
    brightness: u8,
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
    match application_state.view {
        View::Initialise => views::initialise::Initialise.view(),
        View::Configure => views::config::Config.view(application_state.brightness),
        _ => todo!(),
    }
}

fn subscriptions(_: &LaunchpadConfigApp) -> Subscription<Messages> {
    let tick_subscription = iced::time::every(Duration::from_secs(2)).map(|_| Messages::Tick);
    let subscriptions = vec![tick_subscription];

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
            application_state.view = View::Configure;
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

        _ => todo!(),
    }
    Task::none()
}

pub fn main() -> iced::Result {
    iced::application("Launchpad Config", update, view)
        .subscription(subscriptions)
        .run()
}
