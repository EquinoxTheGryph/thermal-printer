#![allow(dead_code)]

use super::sanitize;

trait Encode {
    fn encode(&self) -> Vec<u8>;
}

#[derive(Debug)]
pub struct Millimeters(pub u16);
impl Millimeters {
    /// Used for encoding the value into two parts
    fn encode_double(&self) -> Vec<u8> {
        let [high, low] = super::split((self.0 as f32 / 0.125f32) as u16);
        vec![low, high]
    }
}
impl Encode for Millimeters {
    fn encode(&self) -> Vec<u8> {
        vec![(self.0 as f32 / 0.125f32) as u8]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Justify {
    Left = 0,
    Center = 1,
    Right = 2,
}

#[derive(Debug, Clone, Copy)]
pub struct PrintMode {
    use_font_b: bool,
    reverse: bool,
    upside_down: bool,
    emphasize: bool,
    double_height: bool,
    double_width: bool,
    delete_line: bool,
    _undefined: bool,
}

#[derive(Debug)]
pub(crate) enum Command<'a> {
    /// Raw Data
    Raw(&'a [u8]),

    /// Text Data (Unsafe characters omitted)
    Text(&'a str),

    /// Print and line feed
    LineFeed,

    /// Print and carriage return
    CarriageReturn,

    /// Horizontal tab
    HorizontalTab,

    /// Print the data in buffer
    PrintBuffer,

    /// Set horizontal tab positions
    HTabPositionsSet(&'a [u8]),

    /// Print and Feed $n dots paper
    FeedMillimeters(Millimeters),

    /// Print and Feed $n lines
    FeedLines(u8),

    /// Set peripheral device
    OnlineSet(bool),

    /// Select default line spacing
    LineSpacingReset,

    /// Set line spacing
    LineSpacingSet(Millimeters),

    /// Select justification
    JustifySet(Justify),

    /// Select Double Width mode
    DoubleWidthSet,

    /// Disable Double Width mode
    DoubleWidthReset,

    /// Set left margin
    LeftMarginSet(Millimeters),

    /// Set absolute print position
    PositionSet(Millimeters),

    /// Set Left Space
    LeftSpaceSet(u8),

    /// Select print mode(s)
    PrintModeSet(PrintMode),

    /// Select character size
    FontSizeSet(u8),

    /// Turn white/black reverse printing mode
    InvertSet(bool),

    /// Turn 90°clockwise rotation mode on/off
    RotateSet(u8),

    /// Turn on/off double-strike mode
    DoubleStrikeSet(u8),

    /// Turn emphasized mode on/off
    EmphasizeSet(u8),

    /// Set right-side character spacing
    RightSpaceSet(u8),

    /// Turns on/off upside-down printing mode
    UpsideDownSet(u8),

    /// Set the underline dots(0,1,2)
    UnderlineSet(u8),

    /// Select Kanji/Chinese character mode (Note: Unsure if Kanji OR Chinese)
    CharChineseEnable,

    /// Cancel Kanji/Chinese character mode
    CharChineseDisable,

    /// Set print mode for Kanji/Chinese characters
    CharChinesePrintMode(u8),

    /// Select/Cancel user-defined characters
    CharUserEnable(u8),

    /// Define user-defined characters
    CharUserDefine(u8),

    /// Cancel user-defined characters
    CharUserCancel(u8),

    /// Select international character set
    CharInternationalSet(u8),

    /// Select character code table
    CharCodePageSet(u8),

    /// Select bit-image mode
    BitmapModeSet(u8),

    /// Define downloaded bit image
    BitmapDefine(u8),

    /// Print downloaded bit image
    BitmapPrintDefined(u8),

    /// Print raster bit image
    BitmapPrintRaster(u8),

    /// Print the bitmap
    BitmapPrintData(u8),

    /// Print MSB bitmap
    BitmapPrintDataMsb(u8),

    /// Print LSB bitmap
    BitmapPrintDataLsb(u8),

    /// Print NV bitmap
    BitmapPrintNv(u8, u8),

    /// Define NV bitmap
    BitmapDefineNv(u8),

    /// Initialize the printer
    Reset,

    /// (Serial Only) Transmit status
    RequestStatus(u8),

    /// (Serial Only) Enable/Disable Automatic Status Back(ASB)
    RequestStatusAutoEnable(u8),

    /// (Serial Only) Transmit paper sensor status
    RequestStatusPaperSensor,

    /// (Serial Only) Transmit peripheral device status (For drawer)
    RequestStatusDrawer(u8),

    /// Select printing position for HRI characters
    BarcodeHriPositionSet(u8),

    /// Set bar code height
    BarcodeHeightSet(u8),

    /// Set bar code width
    BarcodeWidthSet(u8),

    /// Print bar code
    BarcodePrint(u8),

    /// Set barcode printing left space
    BarcodePositionSet(u8),

    /// Specify the mode of QR code by n1
    QrCode(u8),

    /// Setting Control Parameter Command
    SettingsSet(u8),

    /// Select sleeping parameter
    SleepTimeoutSet(u8),

    /// Select Chinese code format
    ChineseCodeFormatSet(u8),

    /// Set printing density
    PrintingDensitySet(u8),

    /// Printing test page
    PrintTestPage,

    /// Enable/disable panel buttons (For button)
    ButtonsEnable(u8),

    /// Real-time transmission mode
    RealTimeEnable(u8),

    /// Feed Paper to Mark
    FeedToMark,

    /// Set Mark paper length
    MarkPaperLengthSet(u8),
}

impl<'a> Command<'a> {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Command::Raw(out) => out.to_vec(),
            Command::Text(out) => sanitize(out),
            Command::LineFeed => vec![0x0A],       //  `[0A]`
            Command::CarriageReturn => vec![0x0D], //  `[0D]`
            Command::HorizontalTab => vec![0x09],  //  `[09]`
            Command::PrintBuffer => vec![0x0D],    //  `[0C]`
            Command::HTabPositionsSet(arr) => super::compose(&[0x1B, 0x44], arr, &[0x00], 32), //  `[1B, 44, $^, 00]
            Command::FeedMillimeters(mm) => [vec![0x1B, 0x4A], mm.encode()].concat(), //  `[1B, 4A, $n]`
            Command::FeedLines(lines) => vec![0x1B, 0x64, *lines], //  `[1B, 64, $n]`
            Command::OnlineSet(state) => vec![0x1B, 0x3D, *state as u8], //  `[1B, 3D, $n]`
            Command::LineSpacingReset => vec![0x1B, 0x32],         //  `[1B, 32]`
            Command::LineSpacingSet(mm) => [vec![0x1B, 0x33], mm.encode()].concat(), //  `[1B, 33, $n]`
            Command::JustifySet(justify) => vec![0x1B, 0x61, *justify as u8], //  `[1B, 61, $n]`
            Command::DoubleWidthSet => vec![0x1B, 0x0E],                      //  `[1B, 0E]`
            Command::DoubleWidthReset => vec![0x1B, 0x14],                    //  `[1B, 14]`
            Command::LeftMarginSet(mm) => [vec![0x1D, 0x4C], mm.encode_double()].concat(), //  `[1D, 4C, $n, $n]
            Command::PositionSet(mm) => [vec![0x1B, 0x24], mm.encode_double()].concat(), //  `[1B, 24, $n, $n]
            Command::LeftSpaceSet(n) => vec![0x1B, 0x42, *n], //  `[1B, 42, $n]`
            Command::PrintModeSet(_mode) => todo!(),          //  `[1B, 21, $n]`
            Command::FontSizeSet(_) => todo!(),               //  `[1D, 21, $n]`
            Command::InvertSet(set) => vec![0x1D, 0x42, *set as u8], //  `[1D, 42, $n]`
            Command::RotateSet(_) => todo!(),                 //  `[1B, 56, $n]`
            Command::DoubleStrikeSet(_) => todo!(),           //  `[1B, 47, $n]`
            Command::EmphasizeSet(_) => todo!(),              //  `[1B, 69, $n]`
            Command::RightSpaceSet(_) => todo!(),             //  `[1B, 20, $n]`
            Command::UpsideDownSet(_) => todo!(),             //  `[1B, 7B, $n]`
            Command::UnderlineSet(_) => todo!(),              //  `[1B, 2D, $n]`
            Command::CharChineseEnable => todo!(),            //  `[1C, 26]`
            Command::CharChineseDisable => todo!(),           //  `[1C, 2E]`
            Command::CharChinesePrintMode(_) => todo!(),      //  `[1C, 21, $n]`
            Command::CharUserEnable(_) => todo!(),            //  `[1B, 25, $n]`
            Command::CharUserDefine(_) => todo!(),            //  `[1B, 26, $^]`
            Command::CharUserCancel(_) => todo!(),            //  `[1B, 3F, $n]`
            Command::CharInternationalSet(_) => todo!(),      //  `[1B, 52, $n]`
            Command::CharCodePageSet(_) => todo!(),           //  `[1B, 74, $n]`
            Command::BitmapModeSet(_) => todo!(),             //  `[1B, 2A, $^]`
            Command::BitmapDefine(_) => todo!(),              //  `[1D, 2A, $^]`
            Command::BitmapPrintDefined(_) => todo!(),        //  `[1D, 2F, $n]`
            Command::BitmapPrintRaster(_) => todo!(),         //  `[1D, 76, 30, $^]
            Command::BitmapPrintData(_) => todo!(),           //  `[12, 2A, $^]`
            Command::BitmapPrintDataMsb(_) => todo!(),        //  `[12, 56, $^]`
            Command::BitmapPrintDataLsb(_) => todo!(),        //  `[12, 76, $^]`
            Command::BitmapPrintNv(_, _) => todo!(),          //  `[1C, 70, $n, $n]
            Command::BitmapDefineNv(_) => todo!(),            //  `[1C, 71, $^]`
            Command::Reset => vec![0x1B, 0x40],               //  `[1B, 40]`
            Command::RequestStatus(_) => todo!(),             //  `[1D, 72, $n]`
            Command::RequestStatusAutoEnable(_) => todo!(),   //  `[1D, 61, $n]`
            Command::RequestStatusPaperSensor => todo!(),     //  `[1B, 76, $n]`
            Command::RequestStatusDrawer(_) => todo!(),       //  `[1B, 75, $n]`
            Command::BarcodeHriPositionSet(_) => todo!(),     //  `[1D, 48, $n]`
            Command::BarcodeHeightSet(_) => todo!(),          //  `[1D, 68, $n]`
            Command::BarcodeWidthSet(_) => todo!(),           //  `[1D, 77, $n]`
            Command::BarcodePrint(_) => todo!(),              //  `[1D, 6B, $^]`
            Command::BarcodePositionSet(_) => todo!(),        //  `[1D, 78, $n]`
            Command::QrCode(_) => todo!(),
            Command::SettingsSet(_) => todo!(), // `[1B, 37, $^]`
            Command::SleepTimeoutSet(_) => todo!(), // `[1B, 38, $^]`
            Command::ChineseCodeFormatSet(_) => todo!(), // `[1B, 39, $n]`
            Command::PrintingDensitySet(_) => todo!(), // `[12, 23, $n]`
            Command::PrintTestPage => vec![0x12, 0x54], // `[12, 54]`
            Command::ButtonsEnable(_) => todo!(), // `[1B, 63, 35, $n]`
            Command::RealTimeEnable(_) => todo!(), // `[10, 04, $n]`
            Command::FeedToMark => vec![0x12, 0x45], // `[12, 45]`
            Command::MarkPaperLengthSet(_) => todo!(), // `[12, 6D, $^]`
        }
    }
}
