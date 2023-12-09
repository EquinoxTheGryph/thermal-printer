use std::io::prelude::*;
use std::{fs::File, time::Duration};

use serial2::{FlowControl, SerialPort, Settings};

pub trait WriteBuffer {
    fn write_buffer(&self, buf: &[u8]) -> std::io::Result<()>;
}

/// Write data to a file
pub struct FileDestination<'a> {
    pub path: &'a str,
}

impl<'a> WriteBuffer for FileDestination<'a> {
    fn write_buffer(&self, buf: &[u8]) -> std::io::Result<()> {
        let mut file = File::options().append(true).open(self.path)?;
        file.write(buf)?;
        Ok(())
    }
}

/// Write data to a serial port
pub struct SerialDestination<'a> {
    pub port: &'a str,
    pub baud: u32,
    // Note: DTR/CTS not implemented yet
}

impl<'a> WriteBuffer for SerialDestination<'a> {
    fn write_buffer(&self, buf: &[u8]) -> std::io::Result<()> {
        println!("Opening serial port");
        let port = SerialPort::open(self.port, self.baud)?;
        print!("Writing data");

        for bytes in buf.chunks(8) {
            print!(".");
            port.write(bytes)?;
            port.flush()?;
            std::thread::sleep(Duration::from_millis(10))
        }

        println!();
        println!("Done!");

        Ok(())
    }
}
