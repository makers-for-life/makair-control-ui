// MakAir
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

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
extern crate conrod_winit;
extern crate fluent;
extern crate image;
extern crate inflate;
extern crate unic_langid;

mod chip;
mod config;
mod display;
mod locale;
mod physics;
mod serial;

use std::ops::Deref;
use std::str::FromStr;

use clap::{App, Arg};
use log::LevelFilter;

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

struct AppArgs {
    log: String,
    translation: String,
    mode: Mode,
    fullscreen: bool,
}

pub enum Mode {
    Port {
        port: String,
        output_dir: Option<String>,
    },
    Input(String),
}

lazy_static! {
    static ref APP_ARGS: AppArgs = make_app_args();
    static ref APP_I18N: LocaleAccessor = make_app_i18n();
}

fn make_app_args() -> AppArgs {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("log")
                .short("l")
                .long("log")
                .help("Log level")
                .default_value("debug")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("Serial port ID")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .help("Path to a recorded input file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("Path to a directory where to record telemetry")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("fullscreen")
                .short("f")
                .long("fullscreen")
                .help("Launch in fullscreen mode"),
        )
        .arg(
            Arg::with_name("translation")
                .short("t")
                .long("translation")
                .help("Translation locale ISO code")
                .default_value("en")
                .takes_value(true),
        )
        .get_matches();

    // Parse input mode
    let mode = match (matches.value_of("port"), matches.value_of("input")) {
        (Some(p), _) => Mode::Port {
            port: p.to_string(),
            output_dir: matches.value_of("output").map(|str| str.to_string()),
        },
        (None, Some(i)) => Mode::Input(i.to_string()),
        (None, None) => {
            eprintln!("You should provide either a serial port (-p) or an input file (-i)");
            std::process::exit(1);
        }
    };

    // Generate owned app arguments
    AppArgs {
        log: String::from(matches.value_of("log").expect("invalid log value")),
        translation: String::from(
            matches
                .value_of("translation")
                .expect("invalid translation value"),
        ),
        mode,
        fullscreen: matches.is_present("fullscreen"),
    }
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

    // Spawn window manager
    DisplayWindowBuilder::new().spawn();

    info!("stopped");
}
