// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::UiBuilder;
use glium::glutin::Icon;
use glium::glutin::{ContextBuilder, EventsLoop, WindowBuilder};
use image::load_from_memory;
use inflate::inflate_bytes_zlib;

use crate::chip::Chip;
use crate::config::environment::*;
use crate::EmbeddedFonts;
use crate::EmbeddedImages;
use crate::APP_ARGS;

use super::drawer::DisplayDrawerBuilder;
use super::fonts::Fonts;

pub struct DisplayWindowBuilder;
pub struct DisplayWindow;

impl DisplayWindowBuilder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> DisplayWindow {
        DisplayWindow {}
    }
}

lazy_static! {
    static ref IMAGE_WINDOW_ICON_RGBA_RAW: Vec<u8> =
        load_from_memory(EmbeddedImages::get("window-icon.png").unwrap().to_mut())
            .unwrap()
            .into_rgba()
            .into_raw();
    static ref FONT_DEFAULT_NOTOSANS_REGULAR: conrod_core::text::Font =
        gen_load_font!("notosans_regular");
    static ref FONT_DEFAULT_NOTOSANS_BOLD: conrod_core::text::Font =
        gen_load_font!("notosans_bold");
    static ref FONT_CJK_NOTOSANS_ALL: conrod_core::text::Font =
        gen_load_font!("notosans_cjk_regular");
}

impl DisplayWindow {
    pub fn spawn(&self, chip: Chip) {
        debug!("spawning window...");

        // Create event loop
        let events_loop = EventsLoop::new();

        // Create window
        let window = WindowBuilder::new()
            .with_title("MakAir Control UI")
            .with_window_icon(Some(
                Icon::from_rgba(
                    IMAGE_WINDOW_ICON_RGBA_RAW.to_vec(),
                    WINDOW_ICON_WIDTH,
                    WINDOW_ICON_HEIGHT,
                )
                .unwrap(),
            ))
            .with_dimensions((DISPLAY_WINDOW_SIZE_WIDTH, DISPLAY_WINDOW_SIZE_HEIGHT).into())
            .with_decorations(!APP_ARGS.fullscreen)
            .with_resizable(false)
            .with_always_on_top(APP_ARGS.fullscreen);

        let window = if APP_ARGS.fullscreen {
            let primary_monitor = events_loop.get_primary_monitor();

            window.with_fullscreen(Some(primary_monitor))
        } else {
            window
        };

        // Create context
        let context = ContextBuilder::new().with_multisampling(4);

        // Create the interface
        let mut interface = UiBuilder::new([
            DISPLAY_WINDOW_SIZE_WIDTH as f64,
            DISPLAY_WINDOW_SIZE_HEIGHT as f64,
        ])
        .build();

        // Load all required fonts to interface
        // Notice: this depends on the in-use translation, as eg. CJK glyphs are not included in \
        //   the default font.
        let fonts = match APP_ARGS.translation.as_str() {
            "zh" | "ko" | "ja" => Fonts::new(
                interface.fonts.insert(FONT_CJK_NOTOSANS_ALL.clone()),
                interface.fonts.insert(FONT_CJK_NOTOSANS_ALL.clone()),
            ),
            _ => Fonts::new(
                interface
                    .fonts
                    .insert(FONT_DEFAULT_NOTOSANS_REGULAR.clone()),
                interface.fonts.insert(FONT_DEFAULT_NOTOSANS_BOLD.clone()),
            ),
        };

        // Create window contents drawer
        let mut drawer =
            DisplayDrawerBuilder::new(window, context, events_loop, &mut interface, fonts, chip);

        debug!("window built, will spawn now");

        drawer.run();
    }
}
