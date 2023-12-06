#![allow(dead_code)]
// Programmed specifically for EM5820 Thermal Printer Modules
mod printer;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use image::DynamicImage;
use image_effects::colour::colours::srgb::{BLACK, WHITE};
use image_effects::dither;
use image_effects::effect::Affectable;
use printer::commands_extend::{BitmapData, BitmapDensity};
use printer::*;

use crate::printer::commands_extend::QrCodeData;

trait WriteBuffer {
    fn write_buffer(&self, buf: &[u8]) -> std::io::Result<()>;
}

struct FileDestination<'a>(&'a str);

impl<'a> WriteBuffer for FileDestination<'a> {
    fn write_buffer(&self, buf: &[u8]) -> std::io::Result<()> {
        let mut file = File::options().append(true).open(self.0)?;

        for chunk in buf.chunks(1) {
            file.write(chunk)?;
            // println!("Printed Chunk {chunk:?}");
            // std::thread::sleep(std::time::Duration::from_millis(1));
        }

        Ok(())
    }
}

/// Send the given payload to the printer
fn send_data(destination: &dyn WriteBuffer, buf: &[u8]) -> std::io::Result<()> {
    destination.write_buffer(buf)
}

fn print() -> Result<(), Box<dyn Error>> {
    use commands::Command;

    let density = BitmapDensity::Double24Bit;
    let bitmap_data = to_bitmap(density, image::open("sample/in.png")?);

    let payload: Vec<Command> = vec![
        // Reset
        Command::Reset,
        // Simple Text
        Command::Text("Demo Page"),
        Command::LineFeed,
        // Simple Text
        Command::InvertSet(true),
        Command::Text("Invert"),
        Command::LineFeed,
        // Double Height
        Command::InvertSet(false),
        Command::FontSizeSet(commands_extend::FontSize {
            width: 0,
            height: 2,
        }),
        Command::Text("Double Height"),
        Command::LineFeed,
        // Justify Right
        Command::FontSizeSet(commands_extend::FontSize {
            width: 0,
            height: 0,
        }),
        Command::JustifySet(commands_extend::Justify::Right),
        Command::Text("Right Justified"),
        Command::LineFeed,
        // Justify Center
        Command::JustifySet(commands_extend::Justify::Center),
        Command::Text("Center Justified"),
        Command::LineFeed,
        // Justify Left
        Command::JustifySet(commands_extend::Justify::Left),
        Command::Text("Left Justified"),
        Command::LineFeed,
        // Bold
        Command::EmphasizeSet(true),
        Command::Text("Bold Text"),
        Command::LineFeed,
        // Underline
        Command::EmphasizeSet(false),
        Command::UnderlineSet(commands_extend::Underline::Thin),
        Command::Text("Underlined Text"),
        Command::LineFeed,
        // Extra Large
        Command::UnderlineSet(commands_extend::Underline::Off),
        Command::FontSizeSet(commands_extend::FontSize {
            width: 4,
            height: 4,
        }),
        Command::Text("XLg"),
        Command::LineFeed,
        // Large
        Command::FontSizeSet(commands_extend::FontSize {
            width: 2,
            height: 2,
        }),
        Command::Text("Large"),
        Command::LineFeed,
        // Medium
        Command::FontSizeSet(commands_extend::FontSize {
            width: 1,
            height: 1,
        }),
        Command::Text("Medium"),
        Command::LineFeed,
        Command::LineFeed,
        // Reset
        Command::Reset,
        // QR Code
        Command::QrCode(QrCodeData {
            data: b"Hello world!",
            error_correction: commands_extend::QrCodeErrorCorrection::Medium,
            size: 3,
        }),
        // Bitmap
        Command::Bitmap(bitmap_data),
        Command::LineFeed,
    ];

    let destination = FileDestination("/dev/usb/lp0");

    let mut data: Vec<u8> = vec![];

    for i in payload.iter() {
        data.append(&mut i.encode());
    }

    // print!("{:?}", data);

    send_data(&destination, &data)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    print()?;

    Ok(())
}

// Apply an algorithm to get the byte value of a certain chunk on an image
fn to_bitmap<'a>(density: BitmapDensity, image_input: DynamicImage) -> BitmapData {
    // Process image
    let mut image = image_input;

    let palette = [WHITE, BLACK];
    let max_width = 384u32;
    let filter = image::imageops::FilterType::Lanczos3;

    if image.width() >= max_width {
        println!("Resizing image due to it being to big");
        image = image.resize(max_width, u32::MAX, filter);
    }

    image = image.apply(&dither::ATKINSON.with_palette(palette.to_vec()));
    let buffer = image.to_luma_alpha8();

    // Convert buffer
    let bytes_per_chunk = density.bytes() as u32;
    let chunk_size = bytes_per_chunk as u32 * 8;
    let width = image.width();
    let height = image.height();
    let mut out: Vec<u8> = vec![];

    for row_chunk in 0..=((height - 1) / chunk_size) {
        for column in 0..width {
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

    BitmapData {
        width: width as u16,
        height: height as u16,
        data: out,
        density,
    }
}
