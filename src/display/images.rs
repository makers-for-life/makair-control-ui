// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::borrow::Cow;

use image::load_from_memory;

use crate::config::environment::*;
use crate::utilities::image::reverse_resize_rgba;
use crate::EmbeddedImages;

use super::support::GliumDisplayWinitWrapper;

pub struct DisplayImages;

lazy_static! {
    static ref IMAGE_BOOTLOADER_LOGO_RGBA_RAW: Vec<u8> = gen_load_image_reverse!(
        "bootloader-logo",
        BOOTLOADER_LOGO_IMAGE_WIDTH,
        BOOTLOADER_LOGO_IMAGE_HEIGHT
    );
    static ref IMAGE_ERROR_ICON_RGBA_RAW: Vec<u8> = gen_load_image_reverse!(
        "error-icon",
        ERROR_ICON_IMAGE_WIDTH,
        ERROR_ICON_IMAGE_HEIGHT
    );
    static ref IMAGE_END_OF_LINE_ONGOING_ICON_RGBA_RAW: Vec<u8> = gen_load_image_reverse!(
        "end-of-line-ongoing-icon",
        END_OF_LINE_CONTENT_ICON_IMAGE_WIDTH,
        END_OF_LINE_CONTENT_ICON_IMAGE_HEIGHT
    );
    static ref IMAGE_END_OF_LINE_SUCCESS_ICON_RGBA_RAW: Vec<u8> = gen_load_image_reverse!(
        "end-of-line-success-icon",
        END_OF_LINE_CONTENT_ICON_IMAGE_WIDTH,
        END_OF_LINE_CONTENT_ICON_IMAGE_HEIGHT
    );
    static ref IMAGE_END_OF_LINE_ERROR_ICON_RGBA_RAW: Vec<u8> = gen_load_image_reverse!(
        "end-of-line-error-icon",
        END_OF_LINE_CONTENT_ICON_IMAGE_WIDTH,
        END_OF_LINE_CONTENT_ICON_IMAGE_HEIGHT
    );
    static ref IMAGE_HEADER_STOPPED_RGBA_RAW: Vec<u8> = gen_load_image_reverse!(
        "header-stopped",
        LAYOUT_TEXTURE_HEADER_IMAGE_WIDTH,
        LAYOUT_TEXTURE_HEADER_IMAGE_HEIGHT
    );
    static ref IMAGE_HEADER_STOPPED_SNOOZED_RGBA_RAW: Vec<u8> = gen_load_image_reverse!(
        "header-stopped-snoozed",
        LAYOUT_TEXTURE_HEADER_IMAGE_WIDTH,
        LAYOUT_TEXTURE_HEADER_IMAGE_HEIGHT
    );
    static ref IMAGE_HEADER_RUNNING_RGBA_RAW: Vec<u8> = gen_load_image_reverse!(
        "header-running",
        LAYOUT_TEXTURE_HEADER_IMAGE_WIDTH,
        LAYOUT_TEXTURE_HEADER_IMAGE_HEIGHT
    );
    static ref IMAGE_HEADER_RUNNING_SNOOZED_RGBA_RAW: Vec<u8> = gen_load_image_reverse!(
        "header-running-snoozed",
        LAYOUT_TEXTURE_HEADER_IMAGE_WIDTH,
        LAYOUT_TEXTURE_HEADER_IMAGE_HEIGHT
    );
    static ref IMAGE_PATIENT_CHILD_RGBA_RAW: Vec<u8> = gen_load_image_reverse!(
        "patient-child",
        PRESET_SETTINGS_MODAL_TEXTURE_IMAGE_WIDTH,
        PRESET_SETTINGS_MODAL_TEXTURE_IMAGE_HEIGHT
    );
    static ref IMAGE_PATIENT_TEENAGER_RGBA_RAW: Vec<u8> = gen_load_image_reverse!(
        "patient-teenager",
        PRESET_SETTINGS_MODAL_TEXTURE_IMAGE_WIDTH,
        PRESET_SETTINGS_MODAL_TEXTURE_IMAGE_HEIGHT
    );
    static ref IMAGE_PATIENT_ADULT_RGBA_RAW: Vec<u8> = gen_load_image_reverse!(
        "patient-adult",
        PRESET_SETTINGS_MODAL_TEXTURE_IMAGE_WIDTH,
        PRESET_SETTINGS_MODAL_TEXTURE_IMAGE_HEIGHT
    );
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

    pub fn end_of_line_ongoing_icon(
        display: &GliumDisplayWinitWrapper,
    ) -> glium::texture::SrgbTexture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_END_OF_LINE_ONGOING_ICON_RGBA_RAW[
                END_OF_LINE_CONTENT_ICON_WIDTH, END_OF_LINE_CONTENT_ICON_HEIGHT
            ]
        )
    }

    pub fn end_of_line_success_icon(
        display: &GliumDisplayWinitWrapper,
    ) -> glium::texture::SrgbTexture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_END_OF_LINE_SUCCESS_ICON_RGBA_RAW[
                END_OF_LINE_CONTENT_ICON_WIDTH, END_OF_LINE_CONTENT_ICON_HEIGHT
            ]
        )
    }

    pub fn end_of_line_error_icon(
        display: &GliumDisplayWinitWrapper,
    ) -> glium::texture::SrgbTexture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_END_OF_LINE_ERROR_ICON_RGBA_RAW[
                END_OF_LINE_CONTENT_ICON_WIDTH, END_OF_LINE_CONTENT_ICON_HEIGHT
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

    pub fn header_stopped_snoozed(
        display: &GliumDisplayWinitWrapper,
    ) -> glium::texture::SrgbTexture2d {
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

    pub fn header_running_snoozed(
        display: &GliumDisplayWinitWrapper,
    ) -> glium::texture::SrgbTexture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_HEADER_RUNNING_SNOOZED_RGBA_RAW[
                LAYOUT_TEXTURE_HEADER_WIDTH, LAYOUT_TEXTURE_HEADER_HEIGHT
            ]
        )
    }

    pub fn patient_child(display: &GliumDisplayWinitWrapper) -> glium::texture::SrgbTexture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_PATIENT_CHILD_RGBA_RAW[
                PRESET_SETTINGS_MODAL_TEXTURE_WIDTH, PRESET_SETTINGS_MODAL_TEXTURE_HEIGHT
            ]
        )
    }

    pub fn patient_teenager(display: &GliumDisplayWinitWrapper) -> glium::texture::SrgbTexture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_PATIENT_TEENAGER_RGBA_RAW[
                PRESET_SETTINGS_MODAL_TEXTURE_WIDTH, PRESET_SETTINGS_MODAL_TEXTURE_HEIGHT
            ]
        )
    }

    pub fn patient_adult(display: &GliumDisplayWinitWrapper) -> glium::texture::SrgbTexture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_PATIENT_ADULT_RGBA_RAW[
                PRESET_SETTINGS_MODAL_TEXTURE_WIDTH, PRESET_SETTINGS_MODAL_TEXTURE_HEIGHT
            ]
        )
    }
}
