#![allow(dead_code)]
// Programmed specifically for EM5820 Thermal Printer Modules
mod destinations;
mod printer;

use destinations::WriteBuffer;
use printer::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::destinations::{Destination, SerialDestination};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Printer<'a> {
    #[serde(borrow)]
    pub data: Vec<printer::commands::Command<'a>>,
    #[serde(borrow)]
    pub target: destinations::Destination<'a>,
}
impl<'a> Printer<'a> {
    pub fn print(&self) -> Result<(), Box<dyn Error>> {
        let mut output: Vec<u8> = vec![];

        for i in self.data.iter() {
            output.append(&mut i.encode()?);
        }

        self.target.write_buffer(&output)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // use commands::Command;

    // let printer = Printer {
    //     data: vec![
    //         // Reset
    //         Command::Reset,
    //         // Simple Text
    //         Command::Text("Demo Page"),
    //         // Command::LineFeed,
    //         // Simple Text
    //         Command::InvertSet(true),
    //         Command::Text("Invert"),
    //         Command::LineFeed,
    //         // Double Height
    //         Command::InvertSet(false),
    //         Command::FontSizeSet(commands_extend::FontSize {
    //             width: 0,
    //             height: 2,
    //         }),
    //         Command::Text("Double Height"),
    //         Command::LineFeed,
    //         // Justify Right
    //         Command::FontSizeSet(commands_extend::FontSize {
    //             width: 0,
    //             height: 0,
    //         }),
    //         Command::JustifySet(commands_extend::Justify::Right),
    //         Command::Text("Right Justified"),
    //         Command::LineFeed,
    //         // Justify Center
    //         Command::JustifySet(commands_extend::Justify::Center),
    //         Command::Text("Center Justified"),
    //         Command::LineFeed,
    //         // Justify Left
    //         Command::JustifySet(commands_extend::Justify::Left),
    //         Command::Text("Left Justified"),
    //         Command::LineFeed,
    //         // Bold
    //         Command::EmphasizeSet(true),
    //         Command::Text("Bold Text"),
    //         Command::LineFeed,
    //         // Underline
    //         Command::EmphasizeSet(false),
    //         Command::UnderlineSet(commands_extend::Underline::Thin),
    //         Command::Text("Underlined Text"),
    //         Command::LineFeed,
    //         // Extra Large
    //         Command::UnderlineSet(commands_extend::Underline::Off),
    //         Command::FontSizeSet(commands_extend::FontSize {
    //             width: 4,
    //             height: 4,
    //         }),
    //         Command::Text("XLg"),
    //         Command::LineFeed,
    //         // Large
    //         Command::FontSizeSet(commands_extend::FontSize {
    //             width: 2,
    //             height: 2,
    //         }),
    //         Command::Text("Large"),
    //         Command::LineFeed,
    //         // Medium
    //         Command::FontSizeSet(commands_extend::FontSize {
    //             width: 1,
    //             height: 1,
    //         }),
    //         Command::Text("Medium"),
    //         Command::LineFeed,
    //         Command::LineFeed,
    //         // Reset
    //         Command::Reset,
    //         // QR Code
    //         Command::QrCode(commands_extend::QrCodeData {
    //             data: b"Hello world!",
    //             error_correction: commands_extend::QrCodeErrorCorrection::Medium,
    //             size: 3,
    //         }),
    //         // Bitmap
    //         // Command::Bitmap(bitmap_data),
    //         Command::Image(commands_extend::ImageData {
    //             path: "sample/in.png",
    //             density: commands_extend::BitmapDensity::Double8Bit,
    //         }),
    //         Command::LineFeed,
    //     ],
    //     target: Destination::Serial(SerialDestination {
    //         port: "/dev/ttyUSB0",
    //         baud: 9600,
    //     }),
    // };

    let b: Printer = serde_yaml::from_str(include_str!("../data.yaml"))?;

    b.print()?;
    // let a = serde_yaml::to_string(&printer)?;
    // println!("{}", a);
    // printer.print()?;

    Ok(())
}
