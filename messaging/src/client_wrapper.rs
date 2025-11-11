use crate::protos::boot_logo::SetBootLogo;
use crate::protos::brightness::SetBrightness;
use crate::protos::display_zone_image::{
    ClearAllDisplayZoneImages, ClearDisplayZoneImage, SetDisplayZoneImage,
};
use crate::protos::display_zones::DisplayZone;
use crate::protos::inputs::InputId;
use crate::protos::key_config::{Action, KeyConfig};
use crate::protos::top_level::TopLevel;
use crate::protos::top_level::top_level::Command;
use crate::socket;
use crate::socket::MessageSender;
use protobuf::{EnumOrUnknown, Message};
use std::io::Error;

pub struct ClientWrapper {
    client: socket::Client,
}

/// This trait defines all known operations.
pub trait ClientCommands {
    /// Sets the corresponding action for an input (button, knob, touchscreen) from the launchpad
    ///
    /// * `input_id` - the type of action to associate the actions with
    /// * `actions` - the sequence of actions that should occur when input happens
    fn send_key_config(&mut self, input_id: InputId, actions: Vec<Action>) -> Result<(), Error>;

    /// Sets the boot logo image that displays when the device starts up
    ///
    /// * `image_path` - path to the image file to use as the boot logo, **it should be an absolute path**
    fn set_boot_logo(&mut self, image_path: String) -> Result<(), Error>;

    /// Adjusts the overall brightness of the device display
    ///
    /// * `brightness_percentage` - brightness level as a percentage (0-100)
    fn set_brightness(&mut self, brightness_percentage: u8) -> Result<(), Error>;

    /// Sets an image for a specific display zone on the device
    ///
    /// * `display_zone` - the specific area/zone of the display to update
    /// * `image_path` - path to the image file to display in the specified zone - **it should be an absolute path**
    fn set_display_zone_image(
        &mut self,
        display_zone: DisplayZone,
        image_path: String,
    ) -> Result<(), Error>;

    /// Clears all images from all display zones, resetting them to default/blank state
    fn clear_all_images(&mut self) -> Result<(), Error>;

    /// Clears the image from a specific display zone, resetting it to default/blank state
    ///
    /// * `display_zone` - the specific area/zone of the display to clear
    fn clear_display_zone_image(&mut self, display_zone: DisplayZone) -> Result<(), Error>;
}

/// To be used by any client that wants to communicate with the server
impl ClientWrapper {
    pub fn new(client: socket::Client) -> Self {
        Self { client }
    }
}

impl ClientCommands for ClientWrapper {
    fn send_key_config(&mut self, input_id: InputId, actions: Vec<Action>) -> Result<(), Error> {
        let protobuf = KeyConfig {
            input_id: EnumOrUnknown::new(input_id),
            actions,
            ..KeyConfig::default()
        };
        self.client.send_message(
            create_command(Command::KeyConfigCommand(protobuf))
                .write_to_bytes()?
                .as_slice(),
        )
    }

    fn set_boot_logo(&mut self, image_path: String) -> Result<(), Error> {
        self.client.send_message(
            create_command(Command::SetBootLogoCommand(SetBootLogo {
                image_path,
                ..SetBootLogo::default()
            }))
            .write_to_bytes()?
            .as_slice(),
        )
    }

    fn set_brightness(&mut self, brightness_percentage: u8) -> Result<(), Error> {
        self.client.send_message(
            create_command(Command::SetBrightnessCommand(SetBrightness {
                brightness_value: brightness_percentage.into(),
                ..SetBrightness::default()
            }))
            .write_to_bytes()?
            .as_slice(),
        )
    }

    fn set_display_zone_image(
        &mut self,
        display_zone: DisplayZone,
        image_path: String,
    ) -> Result<(), Error> {
        self.client.send_message(
            create_command(Command::SetDisplayZoneImageCommand(SetDisplayZoneImage {
                display_zone: EnumOrUnknown::from(display_zone),
                image_path,
                ..SetDisplayZoneImage::default()
            }))
            .write_to_bytes()?
            .as_slice(),
        )
    }

    fn clear_all_images(&mut self) -> Result<(), Error> {
        self.client.send_message(
            create_command(Command::ClearAllDisplayZoneImagesCommand(
                ClearAllDisplayZoneImages {
                    ..ClearAllDisplayZoneImages::default()
                },
            ))
            .write_to_bytes()?
            .as_slice(),
        )
    }

    fn clear_display_zone_image(&mut self, display_zone: DisplayZone) -> Result<(), Error> {
        self.client.send_message(
            create_command(Command::ClearDisplayZoneImageCommand(
                ClearDisplayZoneImage {
                    display_zone: EnumOrUnknown::from(display_zone),
                    ..ClearDisplayZoneImage::default()
                },
            ))
            .write_to_bytes()?
            .as_slice(),
        )
    }
}

fn create_command(command: Command) -> TopLevel {
    TopLevel {
        command: Some(command),
        ..TopLevel::default()
    }
}
