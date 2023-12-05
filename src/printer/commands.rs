#![allow(dead_code)]

use super::commands_extend::*;
use super::sanitize;

#[derive(Debug)]
pub enum Command<'a> {
    /// Raw Data
    Raw(&'a [u8]),

    /// Text Data (Unsafe characters omitted)
    Text(&'a str),

    /// Select bit-image mode
    Bitmap(BitmapData<'a>),

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
    FontSizeSet(FontSize),

    /// Turn white/black reverse printing mode
    InvertSet(bool),

    /// Turn 90°clockwise rotation mode on/off
    RotateSet(bool),

    /// Turn on/off double-strike mode
    DoubleStrikeSet(bool),

    /// Turn emphasized mode on/off
    EmphasizeSet(bool),

    /// Set right-side character spacing
    RightSpaceSet(Millimeters),

    /// Turns on/off upside-down printing mode
    UpsideDownSet(bool),

    /// Set the underline dots(0,1,2)
    UnderlineSet(Underline),

    /// Select Kanji/Chinese character mode (Note: Unsure if Kanji OR Chinese)
    CharChineseEnable,

    /// Cancel Kanji/Chinese character mode
    CharChineseDisable,

    /// Set print mode for Kanji/Chinese characters
    CharChinesePrintMode(PrintModeChinese),

    /// Select/Cancel user-defined characters
    CharUserEnable(u8),

    /// Define user-defined characters
    CharUserDefine(u8),

    /// Cancel user-defined characters
    CharUserCancel(u8),

    /// Select international character set
    CharInternationalSet(IntlCharset),

    /// Select character code table
    CharCodePageSet(CodePage),

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

    /// Print a QrCode
    QrCode(QrCodeData<'a>),

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
            Command::Bitmap(data) => data.encode(),
            Command::LineFeed => vec![0x0A],
            Command::CarriageReturn => vec![0x0D],
            Command::HorizontalTab => vec![0x09],
            Command::PrintBuffer => vec![0x0D],
            Command::HTabPositionsSet(arr) => super::compose(&[0x1B, 0x44], arr, &[0x00], 32),
            Command::FeedMillimeters(mm) => [vec![0x1B, 0x4A], mm.encode()].concat(),
            Command::FeedLines(lines) => vec![0x1B, 0x64, *lines],
            Command::OnlineSet(state) => vec![0x1B, 0x3D, *state as u8],
            Command::LineSpacingReset => vec![0x1B, 0x32],
            Command::LineSpacingSet(mm) => [vec![0x1B, 0x33], mm.encode()].concat(),
            Command::JustifySet(justify) => vec![0x1B, 0x61, *justify as u8],
            Command::DoubleWidthSet => vec![0x1B, 0x0E],
            Command::DoubleWidthReset => vec![0x1B, 0x14],
            Command::LeftMarginSet(mm) => [vec![0x1D, 0x4C], mm.encode_double()].concat(),
            Command::PositionSet(mm) => [vec![0x1B, 0x24], mm.encode_double()].concat(),
            Command::LeftSpaceSet(n) => vec![0x1B, 0x42, *n],
            Command::PrintModeSet(mode) => [vec![0x1B, 0x21], mode.encode()].concat(),
            Command::FontSizeSet(size) => [vec![0x1D, 0x21], size.encode()].concat(),
            Command::InvertSet(set) => vec![0x1D, 0x42, *set as u8],
            Command::RotateSet(set) => vec![0x1B, 0x56, *set as u8],
            Command::DoubleStrikeSet(set) => vec![0x1B, 0x47, *set as u8],
            Command::EmphasizeSet(set) => vec![0x1B, 0x69, *set as u8],
            Command::RightSpaceSet(mm) => [vec![0x1B, 0x20], mm.encode()].concat(),
            Command::UpsideDownSet(set) => vec![0x1B, 0x7B, *set as u8],
            Command::UnderlineSet(underline) => vec![0x1B, 0x2D, *underline as u8],
            Command::CharChineseEnable => vec![0x1C, 0x26],
            Command::CharChineseDisable => vec![0x1C, 0x2E],
            Command::CharChinesePrintMode(mode) => [vec![0x1C, 0x21], mode.encode()].concat(),
            Command::CharUserEnable(_) => todo!(), //  `[1B, 25, $n]`
            Command::CharUserDefine(_) => todo!(), //  `[1B, 26, $^]`
            Command::CharUserCancel(_) => todo!(), //  `[1B, 3F, $n]`
            Command::CharInternationalSet(charset) => vec![0x1B, 0x52, *charset as u8],
            Command::CharCodePageSet(cpage) => vec![0x1B, 0x74, *cpage as u8],
            Command::BitmapModeSet(_) => todo!(), //  `[1B, 2A, $^]`
            Command::BitmapDefine(_) => todo!(),  //  `[1D, 2A, $^]`
            Command::BitmapPrintDefined(_) => todo!(), //  `[1D, 2F, $n]`
            Command::BitmapPrintRaster(_) => todo!(), //  `[1D, 76, 30, $^]
            Command::BitmapPrintData(_) => todo!(), //  `[12, 2A, $^]`
            Command::BitmapPrintDataMsb(_) => todo!(), //  `[12, 56, $^]`
            Command::BitmapPrintDataLsb(_) => todo!(), //  `[12, 76, $^]`
            Command::BitmapPrintNv(_, _) => todo!(), //  `[1C, 70, $n, $n]
            Command::BitmapDefineNv(_) => todo!(), //  `[1C, 71, $^]`
            Command::Reset => vec![0x1B, 0x40],   //  `[1B, 40]`
            Command::RequestStatus(_) => todo!(), //  `[1D, 72, $n]`
            Command::RequestStatusAutoEnable(_) => todo!(), //  `[1D, 61, $n]`
            Command::RequestStatusPaperSensor => todo!(), //  `[1B, 76, $n]`
            Command::RequestStatusDrawer(_) => todo!(), //  `[1B, 75, $n]`
            Command::BarcodeHriPositionSet(_) => todo!(), //  `[1D, 48, $n]`
            Command::BarcodeHeightSet(_) => todo!(), //  `[1D, 68, $n]`
            Command::BarcodeWidthSet(_) => todo!(), //  `[1D, 77, $n]`
            Command::BarcodePrint(_) => todo!(),  //  `[1D, 6B, $^]`
            Command::BarcodePositionSet(_) => todo!(), //  `[1D, 78, $n]`
            Command::QrCode(data) => data.encode(),
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
