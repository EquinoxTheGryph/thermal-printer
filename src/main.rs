mod printer;

// Programmed specifically for EM5820 Thermal Printer Modules
use std::fs::File;
use std::io::prelude::*;

use printer::*;

// enum Command<'a> {
//     /// Any normal string
//     Text(&'a str),

//     // Print commands
//     /// `[0A]` Print and line feed
//     LineFeed,
//     /// `[0D]` Print and carriage return
//     CarriageReturn,
//     /// `[09]` Horizontal tab
//     HorizontalTab,
//     /// `[0C]` Print the data in buffer and locate to the next black mark
//     FormFeed, // ?
//     /// `[1B, 44, n<0..32>, 00]` Set horizontal tab positions
//     SetHTabPositions(&'a [u8]),
//     /// `[1B, 4A, n0]` Print and feed paper (`n0` × 0.125mm)
//     PrintAndFeedMillimeters(u8),
//     /// `[1B, 64, n0]` Print and feed `n0` lines
//     PrintAndFeedLines(u8),
//     /// `[1B, 3D, n0]` Set peripheral device online or offline
//     SetPrinterOnline(bool),

//     // Line spacing
//     /// `[1B, 32]` Reset line spacing (3.75mm)
//     SetLineSpacingDefault,
//     /// `[1B, 33, n0]` Set line spacing (`n0` × 0.125mm)
//     SetLineSpacing(u8),
//     /// `[1B, 32, n0]` Select justification for the following text
//     SetJustify(Justify),
//     /// `[1D, 4C, nL, nH]` Set left margin ((nL + nH × 256) × 0.125mm)
//     SetMarginLeft(u16),
//     // SetLeftBlankCharNum(?) // Unknown
//     /// `[1B, 24, nL, nH]` Set Absolute Print Position ((nL + nH × 256) × 0.125mm)
//     SetAbsolutePosition(u16)

//     // Character settings
//     // Bitmap Image
//     // Init
//     // Status
//     // Bar code
//     // Miscallaneous functions
//     // New
// }

// enum Justify {
//     Left = 0x00,
//     Center = 0x01,
//     Right = 0x02,
// }

trait WriteBuffer {
    fn write_buffer(&self, buf: &[u8]) -> std::io::Result<()>;
}

struct FileDestination<'a>(&'a str);

impl<'a> WriteBuffer for FileDestination<'a> {
    fn write_buffer(&self, buf: &[u8]) -> std::io::Result<()> {
        let mut file = File::options().append(true).open(self.0)?;
        file.write(buf)?;
        Ok(())
    }
}

/// Send the given payload to the printer
fn send_data(destination: &dyn WriteBuffer, buf: &[u8]) -> std::io::Result<()> {
    destination.write_buffer(buf)
}

fn main() -> std::io::Result<()> {
    let destination = FileDestination("/dev/usb/lp0");

    let contents = &[0x12,0x54,b'\n'];

    let a = commands::HTabPositionsSet(&[1, 2, 3,5,5,5,5,7,3,2]);

    print!("{:?}", a.encode());

    // send_data(&destination, contents)?;

    Ok(())
}
