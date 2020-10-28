// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

pub fn reverse_rgba(image: &Vec<u8>, width: u32) -> Vec<u8> {
    // Reverses an image over the Y axis, so that it is displayed on screen correctly, as the \
    //   renderer works on an inverted Y axis.
    image
        .chunks((width as usize) * 4)
        .rev()
        .flat_map(|row| row.iter())
        .map(|p| p.clone())
        .collect()
}
