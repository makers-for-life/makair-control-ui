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
mod test_strategies;

use std::str::FromStr;

use clap::{App, Arg};
use log::LevelFilter;

use config::logger::ConfigLogger;
use display::window::DisplayWindowBuilder;
use locale::accessor::LocaleAccessor;
use locale::loader::LocaleLoader;
use telemetry::structures::TelemetryMessage;

#[derive(RustEmbed)]
#[folder = "res/images/"]
pub struct EmbeddedImages;

#[derive(RustEmbed)]
#[folder = "res/fonts/"]
pub struct EmbeddedFonts;

#[derive(RustEmbed)]
#[folder = "res/locales/"]
pub struct EmbeddedLocales;

#[derive(Clone, Debug)]
pub struct AppArgs {
    log: String,
    translation: String,
    mode: Mode,
    fullscreen: bool,
}

#[derive(Clone, Debug)]
pub enum Mode {
    Port {
        port: String,
        output_dir: Option<String>,
    },
    Input(String),
    Test(Vec<TelemetryMessage>),
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

fn make_app_i18n(args: &AppArgs) -> LocaleAccessor {
    LocaleLoader::new(&args.translation).into_accessor()
}

fn main() {
    let app_args = make_app_args();

    let app_i18n = make_app_i18n(&app_args);

    let _logger =
        ConfigLogger::init(LevelFilter::from_str(&app_args.log).expect("invalid log level"));

    info!("starting up");

    // Spawn window manager
    DisplayWindowBuilder::new(app_args, &app_i18n).spawn();

    info!("stopped");
}

#[cfg(test)]
mod tests {
    use crate::display::window::DisplayWindowBuilder;
    use crate::locale::loader::LocaleLoader;
    use crate::test_strategies::tests::TelemetryStrategies;
    use crate::AppArgs;
    use proptest::collection;
    use proptest::test_runner::TestRunner;
    use std::cell::Cell;
    use telemetry::structures::Phase::Inhalation;
    use telemetry::structures::SubPhase::Inspiration;
    use telemetry::structures::{DataSnapshot, TelemetryMessage};

    #[test]
    fn test_gui_with_telemetry_messages() {
        let test_counter = Cell::new(0);

        // With any sequence of TelemetryMessage, the GUI must not crash.
        TestRunner::default()
            .run(
                &collection::vec(
                    TelemetryStrategies::new().telemetry_message_strategy(),
                    1..20,
                ),
                |msgs| {
                    test_counter.set(&test_counter.get() + 1);
                    println!(
                        "Test counter:{}, msg.len:{}",
                        &test_counter.get(),
                        &msgs.len()
                    );
                    run_with_msgs(msgs);

                    Ok(())
                },
            )
            .unwrap();
    }

    #[test]
    fn specfic_failing_telemetry_messages() {
        run_with_msgs(vec![
            TelemetryMessage::DataSnapshot(DataSnapshot {
                version: "".to_string(),
                device_id: "0-0-0".to_string(),
                systick: 1_000_000,
                centile: 0,
                pressure: 100,
                phase: Inhalation,
                subphase: Inspiration,
                blower_valve_position: 0,
                patient_valve_position: 0,
                blower_rpm: 0,
                battery_level: 0,
            }),
            TelemetryMessage::DataSnapshot(DataSnapshot {
                version: "".to_string(),
                device_id: "0-0-0".to_string(),
                systick: 1_000_000,
                centile: 0,
                pressure: 0,
                phase: Inhalation,
                subphase: Inspiration,
                blower_valve_position: 0,
                patient_valve_position: 0,
                blower_rpm: 0,
                battery_level: 0,
            }),
            TelemetryMessage::DataSnapshot(DataSnapshot {
                version: "".to_string(),
                device_id: "0-0-0".to_string(),
                systick: 1_000_000,
                centile: 0,
                pressure: 50,
                phase: Inhalation,
                subphase: Inspiration,
                blower_valve_position: 0,
                patient_valve_position: 0,
                blower_rpm: 0,
                battery_level: 0,
            }),
        ]);
    }

    fn run_with_msgs(msgs: Vec<TelemetryMessage>) {
        DisplayWindowBuilder::new(
            AppArgs {
                log: "test".to_string(),
                translation: "en".to_string(),
                mode: super::Mode::Test(msgs),
                fullscreen: false,
            },
            &LocaleLoader::new("en").into_accessor(),
        )
        .spawn();
    }
}
