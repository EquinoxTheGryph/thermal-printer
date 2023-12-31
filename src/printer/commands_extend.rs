#![allow(dead_code)]

use std::{error::Error, process::Command};

use crate::printer::split;
use serde::{Deserialize, Serialize};

// TODO: Custom Error Enum
pub type EncodeError = Box<dyn Error>;

/// The main trait to allow for turning a command to a set of device instructions
pub trait Encode {
    fn encode(&self) -> Result<Vec<u8>, EncodeError>;
}

/// Utility to convert millimeters to device units
#[derive(Serialize, Deserialize, Debug)]
pub struct Millimeters(pub u16);
impl Millimeters {
    /// Used for encoding the value into two parts
    pub fn encode_double(&self) -> Vec<u8> {
        let [high, low] = super::split((self.0 as f32 / 0.125f32) as u16);
        vec![low, high]
    }
}
impl Encode for Millimeters {
    fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        Ok(vec![(self.0 as f32 / 0.125f32) as u8])
    }
}

/// Font size settings
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct FontSize {
    /// The width of the font (0 to 4, 0 = default)
    pub width: u8,
    /// The height of the font (0 to 4, 0 = default)
    pub height: u8,
}
impl Encode for FontSize {
    fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        Ok(vec![((self.width % 5) << 4) + (self.height % 5)])
    }
}

/// Justification settings (Horizontal alignment)
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Justify {
    Left = 0,
    Center = 1,
    Right = 2,
}

/// Underline settings
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Underline {
    Off = 0,
    Thin = 1,
    Bold = 2,
}

/// International character set selection
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum IntlCharset {
    America = 0,
    France = 1,
    Germany = 2,
    England = 3,
    Denmark1 = 4,
    Sweden = 5,
    Italy = 6,
    Spain1 = 7,
    Japan = 8,
    Norway = 9,
    Denmark2 = 10,
    Spain2 = 11,
    LatinAmerica = 12,
    Korea = 13,
    SloveniaCroatia = 14,
    China = 15,
}

/// Print mode settings
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PrintMode {
    alt_font: bool,
    reverse: bool,
    upside_down: bool,
    emphasize: bool,
    double_height: bool,
    double_width: bool,
    delete_line: bool,
    _undefined: bool,
}
impl Encode for PrintMode {
    fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        Ok(vec![
            (self.alt_font as u8)
                | ((self.reverse as u8) << 1)
                | ((self.upside_down as u8) << 2)
                | ((self.emphasize as u8) << 3)
                | ((self.double_height as u8) << 4)
                | ((self.double_width as u8) << 5)
                | ((self.delete_line as u8) << 6)
                | ((self._undefined as u8) << 7),
        ])
    }
}

/// Heating Settings
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PrintSettings {
    /// Maximum number of heating points (n × 8 points, defaults to 80 points (n = 9)))
    pub points: u8,
    /// Heating time (n × 10µs, defaults to 800µs (n = 80))
    pub time: u8,
    /// Printing Interval (n × 10µs, defaults to 20µs (n = 2))
    pub interval: u8,
}
impl Encode for PrintSettings {
    fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        Ok(vec![self.points, self.time, self.interval])
    }
}

/// Printing mode for Chinese characters
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PrintModeChinese {
    double_height: bool,
    double_width: bool,
    underline: bool,
}
impl Encode for PrintModeChinese {
    fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        Ok(vec![
            ((self.double_height as u8) << 2)
                | ((self.double_width as u8) << 3)
                | ((self.underline as u8) << 7),
        ])
    }
}

/// Code page settings
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum CodePage {
    CP437_USA_STANDARD_EUROPE = 0,
    KATAKANA = 1,
    CP850_MULTILINGUAL = 2,
    CP860_PORTUGUESE = 3,
    CP863_CANADIAN_FRENCH = 4,
    CP865_NORDIC = 5,
    WCP1251_CYRILLIC = 6,
    CP866_CYRILLIEC_2 = 7,
    MIK_CYRILLIC_BULGARIAN = 8,
    CP755_EAST_EUROPE_LATVIAN_2 = 9,
    IRAN = 10,
    RESERVED1 = 11,
    RESERVED2 = 12,
    RESERVED3 = 13,
    RESERVED4 = 14,
    CP862_HEBREW = 15,
    WCP1252_LATIN_I = 16,
    WCP1253_GREEK = 17,
    CP852_LATINA_2 = 18,
    CP858_MULTILINGUAL_LATIN_I_EURO = 19,
    IRAN_II = 20,
    LATVIAN = 21,
    CP864_ARABIC = 22,
    ISO_8859_1_WEST_EUROPE = 23,
    CP737_GREEK = 24,
    WCP1257_BALTIC = 25,
    THAI = 26,
    CP720_ARABIC = 27,
    CP855 = 28,
    CP857_TURKISH = 29,
    WCP1250_CENTRAL_EURPOE = 30,
    CP775 = 31,
    WCP1254_TURKISH = 32,
    WCP1255_HEBREW = 33,
    WCP1256_ARABIC = 34,
    WCP1258_VIETNAM = 35,
    ISO_8859_2_LATIN_2 = 36,
    ISO_8859_3_LATIN_3 = 37,
    ISO_8859_4_BALTIC = 38,
    ISO_8859_5_CYRILLIC = 39,
    ISO_8859_6_ARABIC = 40,
    ISO_8859_7_GREEK = 41,
    ISO_8859_8_HEBREW = 42,
    ISO_8859_9_TURKISH = 43,
    ISO_8859_15_LATIN_3 = 44,
    THAI2 = 45,
    CP856 = 46,
    CP874 = 47,
}

/// QRCode processing
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct QrCodeData<'a> {
    /// Set the error correction factor
    pub error_correction: QrCodeErrorCorrection,
    /// Set the size (0-9)
    pub size: u8,
    /// The actual data
    pub data: &'a [u8],
}
impl<'a> Encode for QrCodeData<'a> {
    fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        // Note: Always add 3 to the size (Reason Unknown)
        let [len_h, len_l] = split(self.data.len() as u16 + 3);
        // println!("l {} h {}", len_l, len_h);

        // Set the size
        let v_size = vec![
            0x1d, // Command Part
            0x28, // Command Part
            0x6b, // Command Part
            0x03, // pL (Lower u16, Payload Size)
            0x00, // pH (Upper u16, Payload Size)
            0x31, // cn (Use Unknown)
            0x43, // fn (Function: Set Size)
            self.size,
        ];

        // Set the Error Correction level
        let v_error_correction = vec![
            0x1d, // Command Part
            0x28, // Command Part
            0x6b, // Command Part
            0x03, // pL (Lower u16, Payload Size)
            0x00, // pH (Upper u16, Payload Size)
            0x31, // cn (Use Unknown)
            0x45, // fn (Function: Set Error Correction)
            self.error_correction as u8,
        ];

        // Set the Data
        let v_data = vec![
            0x1d,  // Command Part
            0x28,  // Command Part
            0x6b,  // Command Part
            len_l, // pL (Lower u16, Payload Size)
            len_h, // pH (Upper u16, Payload Size)
            0x31,  // cn (Use Unknown)
            0x50,  // fn (Function: Set Data)
            0x30,  // Unknown
                   // ...data
        ];

        // Verify the QrCode
        let v_verify = vec![
            0x1d, // Command Part
            0x28, // Command Part
            0x6b, // Command Part
            0x03, // pL (Lower u16, Payload Size)
            0x00, // pH (Upper u16, Payload Size)
            0x31, // cn (Use Unknown)
            0x52, // fn (Function: Verify)
            0x30, // Unknown
        ];

        // Print out the QrCode
        let v_print = vec![
            0x1d, // Command Part
            0x28, // Command Part
            0x6b, // Command Part
            0x03, // pL (Lower u16, Payload Size)
            0x00, // pH (Upper u16, Payload Size)
            0x31, // cn (Use Unknown)
            0x51, // fn (Function: Print)
            0x30, // Unknown
        ];

        // println!("v_size               = {v_size:0>2x?}");
        // println!("v_error_correction   = {v_error_correction:0>2x?}");
        // println!("v_data               = {v_data:0>2x?}");
        // println!("v_verify             = {v_verify:0>2x?}");
        // println!("v_print              = {v_print:0>2x?}");

        Ok([
            v_size,
            v_error_correction,
            v_data,
            self.data.to_vec(), // The actual data payload
            v_verify,
            v_print,
        ]
        .concat())
    }
}
impl<'a> Default for QrCodeData<'a> {
    fn default() -> Self {
        Self {
            error_correction: QrCodeErrorCorrection::Medium,
            size: 5,
            data: &[],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum QrCodeErrorCorrection {
    Low = 48,
    Medium = 49,
    High = 50,
    VeryHigh = 51,
}

/// Bitmap printing
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BitmapData {
    /// Width (1-384)
    pub width: u16,
    /// Height (1-n)
    pub height: u16,
    /// The actual data (Encoding: Each byte is a chunk of 8 pixels high)
    pub data: Vec<u8>,
    /// The density of the data (0, 1, 32, 33 (See documentation))
    pub density: BitmapDensity,
}
impl<'a> Encode for BitmapData {
    fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        // 0, 1, 32, 33 (See documentation)
        let density = self.density as u8;

        // Split the width into a low and high value
        let [width_h, width_l]: [u8; 2] = split(self.width.min(384));

        // Init the vec
        let mut collected: Vec<u8> = vec![];

        self.data
            .chunks_exact(self.width.into())
            .take(self.height.max(1) as usize)
            .for_each(|chunk| {
                collected.append(
                    &mut [
                        // Feed the initial row data
                        vec![0x1Bu8, 0x2A, density, width_l, width_h],
                        // Insert the actual data
                        chunk.to_vec(),
                        // Line feed
                        vec![0x0Au8], // TODO: Maybe use feed mm?
                    ]
                    .concat(),
                );
            });

        // Return the collected data
        Ok(collected)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum BitmapDensity {
    Single8Bit = 0,
    Double8Bit = 1,
    Single24Bit = 32,
    Double24Bit = 33,
}
impl BitmapDensity {
    pub fn bytes(&self) -> u8 {
        match *self {
            BitmapDensity::Single8Bit => 1,
            BitmapDensity::Double8Bit => 1,
            BitmapDensity::Single24Bit => 3,
            BitmapDensity::Double24Bit => 3,
        }
    }

    pub fn bits(&self) -> u8 {
        match *self {
            BitmapDensity::Single8Bit => 8,
            BitmapDensity::Double8Bit => 8,
            BitmapDensity::Single24Bit => 24,
            BitmapDensity::Double24Bit => 24,
        }
    }

    pub fn is_double(&self) -> bool {
        match *self {
            BitmapDensity::Single8Bit => false,
            BitmapDensity::Double8Bit => true,
            BitmapDensity::Single24Bit => false,
            BitmapDensity::Double24Bit => true,
        }
    }
}
impl IntoIterator for BitmapDensity {
    type Item = u8;
    type IntoIter = std::ops::Range<u8>;

    fn into_iter(self) -> Self::IntoIter {
        0..self.bits()
    }
}

/// Image data
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ImageData<'a> {
    pub path: &'a str,
    pub density: BitmapDensity,
}
impl<'a> Encode for ImageData<'a> {
    fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        let bitmap_data = super::to_bitmap(self.density, image::open(self.path)?);
        super::commands::Command::Bitmap(bitmap_data).encode()
    }
}
