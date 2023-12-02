use super::PrinterCommand;

/// Print and line feed
#[derive(Debug)]
pub struct LineFeed;
impl<'a> PrinterCommand<'a> for LineFeed {
    fn encode(&self) -> Vec<u8> {
        vec![0x0A]
    }
}

/// Print and carriage return
#[derive(Debug)]
pub struct CarriageReturn;
impl<'a> PrinterCommand<'a> for CarriageReturn {
    fn encode(&self) -> Vec<u8> {
        vec![0x0D]
    }
}

/// Horizontal tab
#[derive(Debug)]
pub struct HorizontalTab;
impl<'a> PrinterCommand<'a> for HorizontalTab {
    fn encode(&self) -> Vec<u8> {
        vec![0x09]
    }
}

/// Print the data in buffer
#[derive(Debug)]
pub struct PrintBuffer;
impl<'a> PrinterCommand<'a> for PrintBuffer {
    fn encode(&self) -> Vec<u8> {
        vec![0x0D]
    }
}

/// Set horizontal tab positions
#[derive(Debug)]
pub struct HTabPositionsSet<'a>(pub &'a [u8]);
impl<'a> PrinterCommand<'a> for HTabPositionsSet<'a> {
    fn encode(&self) -> Vec<u8> {
        super::compose(&[0x1B, 0x44], self.0, &[0x00], 32)
    }
}

/// Print and Feed $n dots paper
#[derive(Debug)]
pub struct FeedMillimeters(pub u8);
impl<'a> PrinterCommand<'a> for FeedMillimeters {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Print and Feed $n lines
#[derive(Debug)]
pub struct FeedLines(pub u8);
impl<'a> PrinterCommand<'a> for FeedLines {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Set peripheral device
#[derive(Debug)]
pub struct OnlineSet(pub u8);
impl<'a> PrinterCommand<'a> for OnlineSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Select default line spacing
#[derive(Debug)]
pub struct LineSpacingReset;
impl<'a> PrinterCommand<'a> for LineSpacingReset {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Set line spacing
#[derive(Debug)]
pub struct LineSpacingSet(pub u8);
impl<'a> PrinterCommand<'a> for LineSpacingSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Select justification
#[derive(Debug)]
pub struct JustifySet(pub u8);
impl<'a> PrinterCommand<'a> for JustifySet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Select Double Width mode
#[derive(Debug)]
pub struct DoubleWidthSet(pub u8);
impl<'a> PrinterCommand<'a> for DoubleWidthSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Disable Double Width mode
#[derive(Debug)]
pub struct DoubleWidthReset(pub u8);
impl<'a> PrinterCommand<'a> for DoubleWidthReset {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Set left margin
#[derive(Debug)]
pub struct LeftMarginSet(u8, u8);
impl<'a> PrinterCommand<'a> for LeftMarginSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Set absolute print position
#[derive(Debug)]
pub struct PositionSet(u8, u8);
impl<'a> PrinterCommand<'a> for PositionSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Set Left Space
#[derive(Debug)]
pub struct LeftSpaceSet(pub u8);
impl<'a> PrinterCommand<'a> for LeftSpaceSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Select print mode(s)
#[derive(Debug)]
pub struct PrintModeSet(pub u8);
impl<'a> PrinterCommand<'a> for PrintModeSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Select character size
#[derive(Debug)]
pub struct FontSizeSet(pub u8);
impl<'a> PrinterCommand<'a> for FontSizeSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Turn white/black reverse printing mode
#[derive(Debug)]
pub struct InvertSet(pub u8);
impl<'a> PrinterCommand<'a> for InvertSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Turn 90°clockwise rotation mode on/off
#[derive(Debug)]
pub struct RotateSet(pub u8);
impl<'a> PrinterCommand<'a> for RotateSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Turn on/off double-strike mode
#[derive(Debug)]
pub struct DoubleStrikeSet(pub u8);
impl<'a> PrinterCommand<'a> for DoubleStrikeSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Turn emphasized mode on/off
#[derive(Debug)]
pub struct EmphasizeSet(pub u8);
impl<'a> PrinterCommand<'a> for EmphasizeSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Set right-side character spacing
#[derive(Debug)]
pub struct RightSpaceSet(pub u8);
impl<'a> PrinterCommand<'a> for RightSpaceSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Turns on/off upside-down printing mode
#[derive(Debug)]
pub struct UpsideDownSet(pub u8);
impl<'a> PrinterCommand<'a> for UpsideDownSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Set the underline dots(0,1,2)
#[derive(Debug)]
pub struct UnderlineSet(pub u8);
impl<'a> PrinterCommand<'a> for UnderlineSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Select Kanji/Chinese character mode (Note: Unsure if Kanji OR Chinese)
#[derive(Debug)]
pub struct CharChineseEnable;
impl<'a> PrinterCommand<'a> for CharChineseEnable {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Cancel Kanji/Chinese character mode
#[derive(Debug)]
pub struct CharChineseDisable;
impl<'a> PrinterCommand<'a> for CharChineseDisable {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Set print mode for Kanji/Chinese characters
#[derive(Debug)]
pub struct CharChinesePrintMode(pub u8);
impl<'a> PrinterCommand<'a> for CharChinesePrintMode {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Select/Cancel user-defined characters
#[derive(Debug)]
pub struct CharUserEnable(pub u8);
impl<'a> PrinterCommand<'a> for CharUserEnable {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Define user-defined characters
#[derive(Debug)]
pub struct CharUserDefine(pub u8);
impl<'a> PrinterCommand<'a> for CharUserDefine {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Cancel user-defined characters
#[derive(Debug)]
pub struct CharUserCancel(pub u8);
impl<'a> PrinterCommand<'a> for CharUserCancel {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Select international character set
#[derive(Debug)]
pub struct CharInternationalSet(pub u8);
impl<'a> PrinterCommand<'a> for CharInternationalSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Select character code table
#[derive(Debug)]
pub struct CharCodePageSet(pub u8);
impl<'a> PrinterCommand<'a> for CharCodePageSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Select bit-image mode
#[derive(Debug)]
pub struct BitmapModeSet(pub u8);
impl<'a> PrinterCommand<'a> for BitmapModeSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Define downloaded bit image
#[derive(Debug)]
pub struct BitmapDefine(pub u8);
impl<'a> PrinterCommand<'a> for BitmapDefine {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Print downloaded bit image
#[derive(Debug)]
pub struct BitmapPrintDefined(pub u8);
impl<'a> PrinterCommand<'a> for BitmapPrintDefined {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Print raster bit image
#[derive(Debug)]
pub struct BitmapPrintRaster(pub u8);
impl<'a> PrinterCommand<'a> for BitmapPrintRaster {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Print the bitmap
#[derive(Debug)]
pub struct BitmapPrintData(pub u8);
impl<'a> PrinterCommand<'a> for BitmapPrintData {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Print MSB bitmap
#[derive(Debug)]
pub struct BitmapPrintDataMsb(pub u8);
impl<'a> PrinterCommand<'a> for BitmapPrintDataMsb {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Print LSB bitmap
#[derive(Debug)]
pub struct BitmapPrintDataLsb(pub u8);
impl<'a> PrinterCommand<'a> for BitmapPrintDataLsb {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Print NV bitmap
#[derive(Debug)]
pub struct BitmapPrintNv(u8, u8);
impl<'a> PrinterCommand<'a> for BitmapPrintNv {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Define NV bitmap
#[derive(Debug)]
pub struct BitmapDefineNv(pub u8);
impl<'a> PrinterCommand<'a> for BitmapDefineNv {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Initialize the printer
#[derive(Debug)]
pub struct Reset;
impl<'a> PrinterCommand<'a> for Reset {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// (Serial Only) Transmit status
#[derive(Debug)]
pub struct RequestStatus(pub u8);
impl<'a> PrinterCommand<'a> for RequestStatus {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// (Serial Only) Enable/Disable Automatic Status Back(ASB)
#[derive(Debug)]
pub struct RequestStatusAutoEnable(pub u8);
impl<'a> PrinterCommand<'a> for RequestStatusAutoEnable {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// (Serial Only) Transmit paper sensor status
#[derive(Debug)]
pub struct RequestStatusPaperSensor;
impl<'a> PrinterCommand<'a> for RequestStatusPaperSensor {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// (Serial Only) Transmit peripheral device status (For drawer)
#[derive(Debug)]
pub struct RequestStatusDrawer(pub u8);
impl<'a> PrinterCommand<'a> for RequestStatusDrawer {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Select printing position for HRI characters
#[derive(Debug)]
pub struct BarcodeHriPositionSet(pub u8);
impl<'a> PrinterCommand<'a> for BarcodeHriPositionSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Set bar code height
#[derive(Debug)]
pub struct BarcodeHeightSet(pub u8);
impl<'a> PrinterCommand<'a> for BarcodeHeightSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Set bar code width
#[derive(Debug)]
pub struct BarcodeWidthSet(pub u8);
impl<'a> PrinterCommand<'a> for BarcodeWidthSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Print bar code
#[derive(Debug)]
pub struct BarcodePrint(pub u8);
impl<'a> PrinterCommand<'a> for BarcodePrint {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Set barcode printing left space
#[derive(Debug)]
pub struct BarcodePositionSet(pub u8);
impl<'a> PrinterCommand<'a> for BarcodePositionSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Specify the mode of QR code by n1
#[derive(Debug)]
pub struct QrCode(pub u8);
impl<'a> PrinterCommand<'a> for QrCode {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Set the type of QR code graphic module
// QRCode

/// Set the error correction level error of QR code
// QRCode

/// The data stored for receiving QR codes is in a 2d barcode area
// QRCode

/// The data information types that transmit QR code graphics are in 2d barcode
// QRCode

/// Setting Control Parameter Command
#[derive(Debug)]
pub struct SettingsSet(pub u8);
impl<'a> PrinterCommand<'a> for SettingsSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Select sleeping parameter
#[derive(Debug)]
pub struct SleepTimeoutSet(pub u8);
impl<'a> PrinterCommand<'a> for SleepTimeoutSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Select Chinese code format
#[derive(Debug)]
pub struct ChineseCodeFormatSet(pub u8);
impl<'a> PrinterCommand<'a> for ChineseCodeFormatSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Set printing density
#[derive(Debug)]
pub struct PrintingDensitySet(pub u8);
impl<'a> PrinterCommand<'a> for PrintingDensitySet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Printing test page
#[derive(Debug)]
pub struct PrintTestPage;
impl<'a> PrinterCommand<'a> for PrintTestPage {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Enable/disable panel buttons (For button)
#[derive(Debug)]
pub struct ButtonsEnable(pub u8);
impl<'a> PrinterCommand<'a> for ButtonsEnable {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Real-time transmission mode
#[derive(Debug)]
pub struct RealTimeEnable(pub u8);
impl<'a> PrinterCommand<'a> for RealTimeEnable {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Feed Paper to Mark
#[derive(Debug)]
pub struct FeedToMark;
impl<'a> PrinterCommand<'a> for FeedToMark {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}

/// Set Mark paper length
#[derive(Debug)]
pub struct MarkPaperLengthSet(pub u8);
impl<'a> PrinterCommand<'a> for MarkPaperLengthSet {
    fn encode(&self) -> Vec<u8> {
        todo!();
    }
}
