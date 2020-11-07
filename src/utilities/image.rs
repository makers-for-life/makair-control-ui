// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

pub fn reverse_rgb_fast(image: &[u8], width: u32, height: u32) -> Vec<u8> {
    // Reverses an image over the Y axis, so that it is displayed on screen correctly, as the \
    //   renderer works on an inverted Y axis.
    // Notice: this is a more efficient implementation for RGB images, which is not the norm over \
    //   here, but is useful as to reverse frequently updated images like the graphs.
    let (width_value, height_value) = (width as usize, height as usize);

    let mut buffer_reversed: Vec<u8> = vec![0; width_value * height_value * 3];

    for row in 0..(height_value - 1) {
        let (row_start_start, row_start_end) =
            (row * width_value, (height_value - row - 1) * width_value);

        for column in 0..(width_value - 1) {
            let (start_index, end_index) =
                ((row_start_start + column) * 3, (row_start_end + column) * 3);

            buffer_reversed[end_index] = image[start_index];
            buffer_reversed[end_index + 1] = image[start_index + 1];
            buffer_reversed[end_index + 2] = image[start_index + 2];
        }
    }

    buffer_reversed
}

pub fn reverse_rgba(image: &[u8], width: u32) -> Vec<u8> {
    // Reverses an image over the Y axis, so that it is displayed on screen correctly, as the \
    //   renderer works on an inverted Y axis.
    image
        .chunks((width as usize) * 4)
        .rev()
        .flat_map(|row| row.iter())
        .copied()
        .collect()
}
