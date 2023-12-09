use image::DynamicImage;
use image_effects::{
    colour::colours::srgb::{BLACK, WHITE},
    dither,
    effect::Affectable,
};

use self::commands_extend::{BitmapData, BitmapDensity};

pub mod bitmap;
pub mod commands;
pub mod commands_extend;

pub trait PrinterCommand<'a> {
    fn encode(&self) -> Vec<u8>;
}

/// Combines three arrays into one, used for composing an encoded command
pub fn compose<T>(prefix: &[T], middle: &[T], suffix: &[T], middle_limit: usize) -> Vec<T>
where
    T: Clone,
{
    let mut params = middle.to_vec();
    params.truncate(middle_limit);
    [prefix.to_vec(), params, suffix.to_vec()].concat()
}

/// Only allow printable characters through
pub fn sanitize<'a>(input: &'a str) -> Vec<u8> {
    input
        .as_bytes()
        .iter()
        .filter(|c| **c >= 0x20u8 && **c != 0x7F) // 0x20 = Space, 0x7F = DEL
        .copied()
        .collect()
}

/// Split a u16 into two u8s
pub fn split(value: u16) -> [u8; 2] {
    [(value >> 8) as u8, (value & 0xff) as u8]
}

/// Apply an algorithm to get the byte value of a certain chunk on an image
fn to_bitmap<'a>(density: BitmapDensity, image_input: DynamicImage) -> BitmapData {
    println!("Processing image...");
    // Process image
    let mut image = image_input;

    let palette = [WHITE, BLACK];
    let max_width = 384u32;
    let filter = image::imageops::FilterType::Lanczos3;

    // Limit width
    if image.width() >= max_width {
        println!("Resizing image...");
        image = image.resize(max_width, u32::MAX, filter);
    }

    // Account for vertical stretching, depending on the density
    match density {
        BitmapDensity::Single8Bit => {
            image = image.resize_exact(image.width() / 2, image.height() / 3, filter);
        }
        BitmapDensity::Double8Bit => {
            image = image.resize_exact(image.width(), image.height() / 3, filter);
        }
        BitmapDensity::Single24Bit => {
            image = image.resize_exact(image.width() / 2, image.height(), filter);
        }
        BitmapDensity::Double24Bit => {}
    }

    // Apply the dither filter
    println!("Applying dither...");
    image = image.apply(&dither::ATKINSON.with_palette(palette.to_vec()));

    // Convert to Luma8 (Alpha)
    let buffer = image.to_luma_alpha8();

    // Convert buffer
    let bytes_per_chunk = density.bytes() as u32;
    let chunk_size = bytes_per_chunk as u32 * 8;
    let width = image.width();
    let height = image.height();
    let mut out: Vec<u8> = vec![];

    // For each {chunk_size} rows
    for row_chunk in 0..=((height - 1) / chunk_size) {
        // For each horizontal pixel
        for column in 0..width {
            // For each sub-chunk
            for sub_chunk in 0..bytes_per_chunk {
                let start = ((row_chunk) * 8 * bytes_per_chunk) + (sub_chunk * 8);
                let end = start + 8;

                if start >= height {
                    out.push(0);
                } else {
                    let y_range = start..height.min(end);
                    let x = column;
                    let mut result = 0;

                    for y in y_range.rev() {
                        if let Some(pixel) = buffer.get_pixel_checked(x, y) {
                            if pixel[0] <= 128 && pixel[1] >= 128 {
                                result |= 0b10000000 >> (y - start);
                            }
                        }
                    }

                    out.push(result);
                }
            }
        }
    }

    println!("Image processed.");
    BitmapData {
        width: width as u16,
        height: height as u16,
        data: out,
        density,
    }
}
