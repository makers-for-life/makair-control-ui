// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use clap::{App, Arg};

pub enum RunMode {
    Port {
        port: String,
        output_dir: Option<String>,
    },
    Input(String),
}

pub struct ConfigArguments {
    pub log: String,
    pub translation: String,
    pub mode: RunMode,
    pub fullscreen: bool,
    #[cfg(feature = "lora")]
    pub lora: bool,
    #[cfg(feature = "lora")]
    pub lora_device: String,
}

impl ConfigArguments {
    pub fn read() -> Self {
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
                    .env("OUTPUT_DIR")
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
            .arg(
                Arg::with_name("disable-lora")
                    .long("disable-lora")
                    .help("Disable LORA support"),
            )
            .arg(
                Arg::with_name("lora-device")
                    .long("lora-device")
                    .default_value("/dev/ttyAMA0")
                    .help("Path to the LORA device"),
            )
            .get_matches();

        // Parse input mode
        let mode = match (matches.value_of("port"), matches.value_of("input")) {
            (Some(p), _) => RunMode::Port {
                port: p.to_string(),
                output_dir: matches.value_of("output").map(|str| str.to_string()),
            },
            (None, Some(i)) => RunMode::Input(i.to_string()),
            (None, None) => {
                eprintln!("you should provide either a serial port (-p) or an input file (-i)");

                std::process::exit(1);
            }
        };

        // Generate owned app arguments
        ConfigArguments {
            log: String::from(matches.value_of("log").expect("invalid log value")),
            translation: String::from(
                matches
                    .value_of("translation")
                    .expect("invalid translation value"),
            ),
            mode,
            fullscreen: matches.is_present("fullscreen"),
            #[cfg(feature = "lora")]
            lora: !matches.is_present("disable-lora"),
            #[cfg(feature = "lora")]
            lora_device: String::from(
                matches
                    .value_of("lora-device")
                    .expect("invalid lora-device value"),
            ),
        }
    }

    pub fn is_recording(&self) -> bool {
        match &self.mode {
            RunMode::Port { output_dir, .. } => output_dir.is_some(),
            _ => false,
        }
    }
}
