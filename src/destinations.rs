use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::{fs::File, time::Duration};

use serial2::SerialPort;

pub trait WriteBuffer {
    fn write_buffer(&self, buf: &[u8]) -> std::io::Result<()>;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Destination<'a> {
    #[serde(borrow)]
    File(FileDestination<'a>),
    #[serde(borrow)]
    Serial(SerialDestination<'a>),
}
impl<'a> WriteBuffer for Destination<'a> {
    fn write_buffer(&self, buf: &[u8]) -> std::io::Result<()> {
        match self {
            Destination::File(i) => i.write_buffer(buf),
            Destination::Serial(i) => i.write_buffer(buf),
        }
    }
}

/// Write data to a file
#[derive(Serialize, Deserialize, Debug)]
pub struct FileDestination<'a> {
    pub path: &'a str,
}

impl<'a> WriteBuffer for FileDestination<'a> {
    fn write_buffer(&self, buf: &[u8]) -> std::io::Result<()> {
        println!("Writing data to file {}", self.path);
        let mut file = File::options().append(true).open(self.path)?;
        file.write(buf)?;
        println!("Done!");
        Ok(())
    }
}

/// Write data to a serial port
#[derive(Serialize, Deserialize, Debug)]
pub struct SerialDestination<'a> {
    pub port: &'a str,
    pub baud: u32,
}

impl<'a> WriteBuffer for SerialDestination<'a> {
    fn write_buffer(&self, buf: &[u8]) -> std::io::Result<()> {
        println!(
            "Writing data to serial port {} at baud {}",
            self.port, self.baud
        );
        let port = SerialPort::open(self.port, self.baud)?;

        for bytes in buf.chunks(8) {
            port.write(bytes)?;
            port.flush()?;
            std::thread::sleep(Duration::from_millis(10))
        }

        println!("Done!");
        Ok(())
    }
}
