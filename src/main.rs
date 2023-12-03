mod printer;

// Programmed specifically for EM5820 Thermal Printer Modules
use std::fs::File;
use std::io::prelude::*;

use printer::*;

use crate::printer::commands_extend::QrCodeData;

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
    use commands::Command;

    let a: Vec<Command> = vec![
        Command::Reset,
        // Command::Raw(&[27, 55, 1, 1, 1]),
        // Command::Raw(&qr),
        Command::QrCode(QrCodeData {
            data: b"Hello world!",
            error_correction: commands_extend::QrCodeErrorCorrection::Medium,
            size: 3
        })
        // // Command::InvertSet(true),
        // // Command::RotateSet(true),
        // Command::UpsideDownSet(true),
        // Command::Text("Hello!"),
        // Command::LineFeed, // Command::FeedMillimeters(Millimeters(1)),
    ];

    let destination = FileDestination("/dev/usb/lp0");

    let mut data: Vec<u8> = vec![];

    for i in a.iter() {
        data.append(&mut i.encode());
    }

    print!("{:?}", data);

    send_data(&destination, &data)?;

    Ok(())
}
