// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_winit::WinitWindow;

pub struct GliumDisplayWinitWrapper(pub glium::Display);

impl WinitWindow for GliumDisplayWinitWrapper {
    fn get_inner_size(&self) -> Option<(u32, u32)> {
        let s = self.0.gl_window().window().inner_size();
        Some((s.width, s.height))
    }

    fn hidpi_factor(&self) -> f32 {
        self.0.gl_window().window().scale_factor() as f32
    }
}

// Conversion functions for converting between types from glium's version of `winit` and \
//   `conrod_core`.
conrod_winit::v025_conversion_fns!();
