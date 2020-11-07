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
    static ref IMAGE_BOOTLOADER_LOGO_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("bootloader-logo", BOOTLOADER_LOGO_WIDTH);
    static ref IMAGE_ERROR_ICON_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("error-icon", ERROR_ICON_WIDTH);
    static ref IMAGE_HEADER_STOPPED_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("header-stopped", LAYOUT_TEXTURE_HEADER_WIDTH);
    static ref IMAGE_HEADER_STOPPED_SNOOZED_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("header-stopped-snoozed", LAYOUT_TEXTURE_HEADER_WIDTH);
    static ref IMAGE_HEADER_RUNNING_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("header-running", LAYOUT_TEXTURE_HEADER_WIDTH);
    static ref IMAGE_HEADER_RUNNING_SNOOZED_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("header-running-snoozed", LAYOUT_TEXTURE_HEADER_WIDTH);
    static ref IMAGE_TELEMETRY_ARROW_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("telemetry-arrow", TELEMETRY_ARROW_WIDTH);
}

impl DisplayImages {
    pub fn bootloader_logo(display: &GliumDisplayWinitWrapper) -> glium::texture::SrgbTexture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_BOOTLOADER_LOGO_RGBA_RAW[
                BOOTLOADER_LOGO_WIDTH, BOOTLOADER_LOGO_HEIGHT
            ]
        )
    }

    pub fn error_icon(display: &GliumDisplayWinitWrapper) -> glium::texture::SrgbTexture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_ERROR_ICON_RGBA_RAW[
                ERROR_ICON_WIDTH, ERROR_ICON_HEIGHT
            ]
        )
    }

    pub fn header_stopped(display: &GliumDisplayWinitWrapper) -> glium::texture::SrgbTexture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_HEADER_STOPPED_RGBA_RAW[
                LAYOUT_TEXTURE_HEADER_WIDTH, LAYOUT_TEXTURE_HEADER_HEIGHT
            ]
        )
    }

    pub fn header_stopped_snoozed(display: &GliumDisplayWinitWrapper) -> glium::texture::SrgbTexture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_HEADER_STOPPED_SNOOZED_RGBA_RAW[
                LAYOUT_TEXTURE_HEADER_WIDTH, LAYOUT_TEXTURE_HEADER_HEIGHT
            ]
        )
    }

    pub fn header_running(display: &GliumDisplayWinitWrapper) -> glium::texture::SrgbTexture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_HEADER_RUNNING_RGBA_RAW[
                LAYOUT_TEXTURE_HEADER_WIDTH, LAYOUT_TEXTURE_HEADER_HEIGHT
            ]
        )
    }

    pub fn header_running_snoozed(display: &GliumDisplayWinitWrapper) -> glium::texture::SrgbTexture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_HEADER_RUNNING_SNOOZED_RGBA_RAW[
                LAYOUT_TEXTURE_HEADER_WIDTH, LAYOUT_TEXTURE_HEADER_HEIGHT
            ]
        )
    }

    pub fn telemetry_arrow(display: &GliumDisplayWinitWrapper) -> glium::texture::SrgbTexture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_TELEMETRY_ARROW_RGBA_RAW[
                TELEMETRY_ARROW_WIDTH, TELEMETRY_ARROW_HEIGHT
            ]
        )
    }

    pub fn graph_pressure(display: &GliumDisplayWinitWrapper) -> glium::texture::SrgbTexture2d {
        // This draws an empty chart image, which is only used to reserve space in the shared \
        //   image map on start, and acquire a replaceable image identifier.
        // Notice: an empty pressure data vector is created, which is immediately dropped once \
        //   this is rendered.
        DisplayGraph::draw_pressure(display, &ChipDataPressure::new(), None)
    }
}
