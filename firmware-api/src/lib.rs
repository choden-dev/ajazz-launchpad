//! Contains the required APIs to interact with the launchpad
//!
//! Most of the APIs needed are from the Device crate.
//!
//! Note for images: when passing the image `File` as an argument, it MUST
//! be a JPG or JPEG format, or it will not be handled properly by the device
//! (an exception for background image which somehow works with PNGs, but JPG still preferred)
mod commands;

mod inputs;

mod common;

pub mod display_zones;

pub mod device;
