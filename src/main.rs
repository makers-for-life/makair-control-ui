// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

// Notice: increasing the recursion limit is required for all display identifiers to be used
#![recursion_limit = "256"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rust_embed;
#[macro_use]
extern crate conrod_core;

mod chip;
mod config;
mod display;
mod locale;
#[cfg(feature = "lora")]
mod lora;
mod serial;
mod utilities;
mod widget;

use std::ops::Deref;
use std::str::FromStr;

use log::LevelFilter;

use crate::chip::Chip;
#[cfg(feature = "lora")]
use crate::lora::controller::LoraController;

use config::arguments::ConfigArguments;
use config::logger::ConfigLogger;
use display::window::DisplayWindowBuilder;
use locale::accessor::LocaleAccessor;
use locale::loader::LocaleLoader;

#[derive(RustEmbed)]
#[folder = "res/images/"]
pub struct EmbeddedImages;

#[derive(RustEmbed)]
#[folder = "res/fonts/"]
pub struct EmbeddedFonts;

#[derive(RustEmbed)]
#[folder = "res/locales/"]
pub struct EmbeddedLocales;

lazy_static! {
    static ref APP_ARGS: ConfigArguments = make_app_args();
    static ref APP_I18N: LocaleAccessor = make_app_i18n();
}

fn make_app_args() -> ConfigArguments {
    ConfigArguments::read()
}

fn make_app_i18n() -> LocaleAccessor {
    LocaleLoader::new(&APP_ARGS.translation).into_accessor()
}

fn ensure_states() {
    // Ensure all statics are valid (a `deref` is enough to lazily initialize them)
    let (_, _) = (APP_ARGS.deref(), APP_I18N.deref());
}

fn main() {
    let _logger =
        ConfigLogger::init(LevelFilter::from_str(&APP_ARGS.log).expect("invalid log level"));

    info!("starting up");

    // Ensure all states are bound
    ensure_states();

    // Launch LORA init and get Sender for chip
    #[cfg(feature = "lora")]
    let lora_sender = if APP_ARGS.lora {
        Some(LoraController::new())
    } else {
        None
    };

    #[cfg(not(feature = "lora"))]
    let lora_sender = None;

    // Create our "Chip" that will store all the data
    let chip = Chip::new(lora_sender);

    // Spawn window manager
    DisplayWindowBuilder::new().spawn(chip);

    info!("stopped");
}
