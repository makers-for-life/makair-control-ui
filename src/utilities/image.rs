// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use crate::config::environment::*;
use resize::px::RGBA;
use resize::Pixel::RGBA8;
use resize::Type::Mitchell;
use rgb::FromSlice;

pub fn reverse_rgba(image: &[u8], width: u32, height: u32) -> Vec<u8> {
    // Reverses an image over the Y axis, so that it is displayed on screen correctly, as the \
    //   renderer works on an inverted Y axis.

    if FACTOR == 1 {
        image
            .chunks((width as usize) * 4)
            .rev()
            .flat_map(|row| row.iter())
            .copied()
            .collect()
    } else {
        let (w1, h1) = (width / FACTOR, height / FACTOR);
        let (w2, h2) = (width, height);
        let src = (image).as_rgba();

        let mut dst = vec![RGBA::new(0, 0, 0, 0); (w2 * h2).try_into().unwrap()];
        // Create resizer instance.
        let mut resizer = resize::new(
            w1.try_into().unwrap(),
            h1.try_into().unwrap(),
            w2.try_into().unwrap(),
            h2.try_into().unwrap(),
            RGBA8,
            Mitchell,
        )
        .unwrap();
        resizer.resize(src, &mut dst).ok();

        let export: Vec<u8> = dst.iter().flat_map(|rgba| rgba.iter()).collect();
        export
            .chunks((width as usize) * 4)
            .rev()
            .flat_map(|row| row.iter())
            .copied()
            .collect()
    }
}
