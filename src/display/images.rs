// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::borrow::Cow;

use image::load_from_memory;

use crate::chip::ChipDataPressure;
use crate::config::environment::*;
use crate::utilities::image::reverse_rgba;
use crate::EmbeddedImages;

use super::graph::DisplayGraph;
use super::support::GliumDisplayWinitWrapper;

pub struct DisplayImages;

lazy_static! {
    static ref IMAGE_TOP_LOGO_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("top-logo", BRANDING_WIDTH);
    static ref IMAGE_BOOTLOADER_LOGO_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("bootloader-logo", BOOTLOADER_LOGO_WIDTH);
    static ref IMAGE_ERROR_ICON_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("error-icon", ERROR_ICON_WIDTH);
    static ref IMAGE_TELEMETRY_ARROW_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("telemetry-arrow", TELEMETRY_ARROW_WIDTH);
    static ref IMAGE_CONTROLS_RUN_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("controls-run", CONTROLS_BUTTON_ICON_WIDTH);
    static ref IMAGE_CONTROLS_SNOOZE_INACTIVE_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("controls-snooze-inactive", CONTROLS_BUTTON_ICON_WIDTH);
    static ref IMAGE_CONTROLS_SNOOZE_ACTIVE_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("controls-snooze-active", CONTROLS_BUTTON_ICON_WIDTH);
    static ref IMAGE_CONTROLS_ADVANCED_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("controls-advanced", CONTROLS_BUTTON_ICON_WIDTH);
    static ref IMAGE_STATUS_SAVE_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("save", STATUS_SAVE_ICON_WIDTH);
}

impl DisplayImages {
    pub fn bootloader_logo(display: &GliumDisplayWinitWrapper) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_BOOTLOADER_LOGO_RGBA_RAW[
                BOOTLOADER_LOGO_WIDTH, BOOTLOADER_LOGO_HEIGHT
            ]
        )
    }

    pub fn error_icon(display: &GliumDisplayWinitWrapper) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_ERROR_ICON_RGBA_RAW[
                ERROR_ICON_WIDTH, ERROR_ICON_HEIGHT
            ]
        )
    }

    pub fn telemetry_arrow(display: &GliumDisplayWinitWrapper) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_TELEMETRY_ARROW_RGBA_RAW[
                TELEMETRY_ARROW_WIDTH, TELEMETRY_ARROW_HEIGHT
            ]
        )
    }

    pub fn controls_run_icon(display: &GliumDisplayWinitWrapper) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_CONTROLS_RUN_RGBA_RAW[
                CONTROLS_BUTTON_ICON_WIDTH, CONTROLS_BUTTON_ICON_HEIGHT
            ]
        )
    }

    pub fn controls_snooze_inactive_icon(
        display: &GliumDisplayWinitWrapper,
    ) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_CONTROLS_SNOOZE_INACTIVE_RGBA_RAW[
                CONTROLS_BUTTON_ICON_WIDTH, CONTROLS_BUTTON_ICON_HEIGHT
            ]
        )
    }

    pub fn controls_snooze_active_icon(
        display: &GliumDisplayWinitWrapper,
    ) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_CONTROLS_SNOOZE_ACTIVE_RGBA_RAW[
                CONTROLS_BUTTON_ICON_WIDTH, CONTROLS_BUTTON_ICON_HEIGHT
            ]
        )
    }

    pub fn controls_advanced_icon(display: &GliumDisplayWinitWrapper) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_CONTROLS_ADVANCED_RGBA_RAW[
                CONTROLS_BUTTON_ICON_WIDTH, CONTROLS_BUTTON_ICON_HEIGHT
            ]
        )
    }

    pub fn status_save_icon(display: &GliumDisplayWinitWrapper) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_STATUS_SAVE_RGBA_RAW[
                STATUS_SAVE_ICON_WIDTH, STATUS_SAVE_ICON_HEIGHT
            ]
        )
    }

    pub fn branding(display: &GliumDisplayWinitWrapper) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_TOP_LOGO_RGBA_RAW[
                BRANDING_WIDTH, BRANDING_HEIGHT
            ]
        )
    }

    pub fn graph_pressure(display: &GliumDisplayWinitWrapper) -> glium::texture::Texture2d {
        // This draws an empty chart image, which is only used to reserve space in the shared \
        //   image map on start, and acquire a replaceable image identifier.
        // Notice: an empty pressure data vector is created, which is immediately dropped once \
        //   this is rendered.
        DisplayGraph::draw_pressure(display, &ChipDataPressure::new(), None)
    }
}
