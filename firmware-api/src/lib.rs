//! ## Overview
//!
//! Contains the required APIs to interact with the launchpad
//!
//! Most of the APIs needed are from the Device crate.
//!
//! Note for images: when passing the image `File` as an argument, it MUST
//! be a JPG or JPEG format, or it will not be handled properly by the device
//! (an exception for background image which somehow works with PNG, but JPG still preferred)
//!
//! ## Display zone Layout
//!
//! ### Main Button Panel (5×2)
//! | button1 | button2 | button3 | button4 | button5 |
//! |---------|---------|---------|---------|---------|
//! | button6 | button7 | button8 | button9 | button10|
//!
//! ### Touchscreen Panel (4×1)
//! | touchscreen1 | touchscreen2 | touchscreen3 | touchscreen4 |
//! |--------------|--------------|--------------|--------------|
//!
//! This layout provides a comprehensive control interface with 10 physical buttons
//! arranged in a 2-row grid, and 4 touchscreen controls in a single row below.
pub struct ControlPanel {
    // Your struct implementation here
}
mod commands;

mod inputs;

mod common;

pub mod display_zones;

pub mod device;
