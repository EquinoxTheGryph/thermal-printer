# Command Codes

Most, if not all have been untested.
There is some nuance between the two sourced documentation, this is a collection of all commands available.

## Table

> Variables:
>
> -   `$n` = One variable
> -   `$^` = See documentation (Many Parameters)

| Short Name                 | Ascii Code                          | Hex Sequence       | D1  | D2  | Category | Short Description                                                                | Extra
| -------------------------- | ----------------------------------- | ------------------ | --- | --- | -------- | -------------------------------------------------------------------------------- | ---
| LineFeed                   | `LF`                                | `[0A]`             | ✔️  | ✔️  | Print    | Print and line feed                                                              |
| CarriageReturn             | `CR`                                | `[0D]`             | ✔️  | ✔️  | Print    | Print and carriage return                                                        |
| HorizontalTab              | `HT`                                | `[09]`             | ✔️  | ✔️  | Print    | Horizontal tab                                                                   |
| PrintBuffer                | `FF`                                | `[0C]`             | ❌  | ✔️  | Print    | Print the data in buffer                                                         |
| HTabPositionsSet m         | `ESC D $^`                          | `[1B, 44, $^, 00]` | ✔️  | ✔️  | Print    | Set horizontal tab positions                                                     |
| FeedMillimeters 1          | `ESC J $n`                          | `[1B, 4A, $n]`     | ✔️  | ✔️  | Print    | Print and Feed $n dots paper                                                     |
| FeedLines 1                | `ESC d $n`                          | `[1B, 64, $n]`     | ✔️  | ✔️  | Print    | Print and Feed $n lines                                                          |
| OnlineSet 1                | `ESC = $n`                          | `[1B, 3D, $n]`     | ✔️  | ✔️  | Print    | Set peripheral device                                                            |
| LineSpacingReset           | `ESC 2`                             | `[1B, 32]`         | ✔️  | ✔️  | Line     | Select default line spacing                                                      |
| LineSpacingSet 1           | `ESC 3 $n`                          | `[1B, 33, $n]`     | ✔️  | ✔️  | Line     | Set line spacing                                                                 |
| JustifySet 1               | `ESC a $n`                          | `[1B, 61, $n]`     | ✔️  | ✔️  | Line     | Select justification                                                             |
| DoubleWidthSet             | `ESC SO`                            | `[1B, 0E]`         | ✔️  | ✔️  | Line     | Select Double Width mode                                                         |
| DoubleWidthReset           | `ESC DC4`                           | `[1B, 14]`         | ✔️  | ✔️  | Line     | Disable Double Width mode                                                        |
| LeftMarginSet 2            | `GS L nL nH`                        | `[1D, 4C, $n, $n]` | ✔️  | ✔️  | Line     | Set left margin                                                                  |
| PositionSet 2              | `ESC $ nL nH`                       | `[1B, 24, $n, $n]` | ✔️  | ✔️  | Line     | Set absolute print position                                                      |
| LeftSpaceSet 1             | `ESC B $n`                          | `[1B, 42, $n]`     | ✔️  | ✔️  | Line     | Set Left Space                                                                   |
| PrintModeSet 1             | `ESC ! $n`                          | `[1B, 21, $n]`     | ✔️  | ✔️  | Char     | Select print mode(s)                                                             |
| FontSizeSet 1              | `GS ! $n`                           | `[1D, 21, $n]`     | ✔️  | ✔️  | Char     | Select character size                                                            |
| InvertSet 1                | `GS B $n`                           | `[1D, 42, $n]`     | ✔️  | ✔️  | Char     | Turn white/black reverse printing mode                                           |
| RotateSet 1                | `ESC V $n`                          | `[1B, 56, $n]`     | ✔️  | ✔️  | Char     | Turn 90°clockwise rotation mode on/off                                           |
| DoubleStrikeSet 1          | `ESC G $n`                          | `[1B, 47, $n]`     | ✔️  | ✔️  | Char     | Turn on/off double-strike mode                                                   |
| EmphasizeSet 1             | `ESC E $n`                          | `[1B, 69, $n]`     | ✔️  | ✔️  | Char     | Turn emphasized mode on/off                                                      |
| RightSpaceSet 1            | `ESC SP $n`                         | `[1B, 20, $n]`     | ❌  | ✔️  | Char     | Set right-side character spacing                                                 |
| UpsideDownSet 1            | `ESC { $n`                          | `[1B, 7B, $n]`     | ✔️  | ✔️  | Char     | Turns on/off upside-down printing mode                                           |
| UnderlineSet 1             | `ESC - $n`                          | `[1B, 2D, $n]`     | ✔️  | ✔️  | Char     | Set the underline dots(0,1,2)                                                    |
| CharChineseEnable          | `FS &`                              | `[1C, 26]`         | ✔️  | ✔️  | Char     | Select Kanji/Chinese character mode (Note: Unsure if Kanji OR Chinese)           |
| CharChineseDisable         | `FS .`                              | `[1C, 2E]`         | ✔️  | ✔️  | Char     | Cancel Kanji/Chinese character mode                                              |
| CharChinesePrintMode 1     | `FS ! $n`                           | `[1C, 21, $n]`     | ✔️  | ✔️  | Char     | Set print mode for Kanji/Chinese characters                                      |
| CharUserEnable 1           | `ESC % $n`                          | `[1B, 25, $n]`     | ✔️  | ✔️  | Char     | Select/Cancel user-defined characters                                            |
| CharUserDefine m           | `ESC & $^`                          | `[1B, 26, $^]`     | ✔️  | ✔️  | Char     | Define user-defined characters                                                   |
| CharUserCancel 1           | `ESC ? $n`                          | `[1B, 3F, $n]`     | ✔️  | ✔️  | Char     | Cancel user-defined characters                                                   |
| CharInternationalSet 1     | `ESC R $n`                          | `[1B, 52, $n]`     | ✔️  | ✔️  | Char     | Select international character set                                               |
| CharCodePageSet 1          | `ESC t $n`                          | `[1B, 74, $n]`     | ✔️  | ✔️  | Char     | Select character code table                                                      |
| BitmapModeSet m            | `ESC * $^`                          | `[1B, 2A, $^]`     | ✔️  | ✔️  | BMP      | Select bit-image mode                                                            |
| BitmapDefine m             | `GS * $^`                           | `[1D, 2A, $^]`     | ✔️  | ✔️  | BMP      | Define downloaded bit image                                                      |
| BitmapPrintDefined 1       | `GS / $n`                           | `[1D, 2F, $n]`     | ✔️  | ✔️  | BMP      | Print downloaded bit image                                                       |
| BitmapPrintRaster m        | `GS v 0 $^`                         | `[1D, 76, 30, $^]` | ✔️  | ✔️  | BMP      | Print raster bit image                                                           |
| BitmapPrintData m          | `DC2 * $^`                          | `[12, 2A, $^]`     | ❌  | ✔️  | BMP      | Print the bitmap                                                                 |
| BitmapPrintDataMSB m       | `DC2 V`                             | `[12, 56, $^]`     | ❌  | ✔️  | BMP      | Print MSB bitmap                                                                 |
| BitmapPrintDataLSB m       | `DC2 v`                             | `[12, 76, $^]`     | ❌  | ✔️  | BMP      | Print LSB bitmap                                                                 |
| BitmapPrintNV 2            | `FS p $n $n`                        | `[1C, 70, $n, $n]` | ✔️  | ✔️  | BMP      | Print NV bitmap                                                                  |
| BitmapDefineNV m           | `FS q $^`                           | `[1C, 71, $^]`     | ✔️  | ✔️  | BMP      | Define NV bitmap                                                                 |
| Reset                      | `ESC @`                             | `[1B, 40]`         | ✔️  | ✔️  | Init     | Initialize the printer                                                           |
| RequestStatus 1            | `GS r $n`                           | `[1D, 72, $n]`     | ✔️  | ✔️  | Status   | (Serial Only) Transmit status                                                    |
| RequestStatusAutoEnable 1  | `GS a $n`                           | `[1D, 61, $n]`     | ✔️  | ✔️  | Status   | (Serial Only) Enable/Disable Automatic Status Back(ASB)                          |
| RequestStatusPaperSensor 1 | `ESC v $n`                          | `[1B, 76, $n]`     | ✔️  | ✔️  | Status   | (Serial Only) Transmit paper sensor status                                       |
| RequestStatusDrawer 1      | `ESC u $n`                          | `[1B, 75, $n]`     | ❌  | ✔️  | Status   | (Serial Only) Transmit peripheral device status (For drawer)                     |
| BarcodeHRIPositionSet 1    | `GS H $n`                           | `[1D, 48, $n]`     | ✔️  | ✔️  | Barcode  | Select printing position for HRI characters                                      |
| BarcodeHeightSet 1         | `GS h $n`                           | `[1D, 68, $n]`     | ✔️  | ✔️  | Barcode  | Set bar code height                                                              |
| BarcodeWidthSet 1          | `GS w $n`                           | `[1D, 77, $n]`     | ✔️  | ✔️  | Barcode  | Set bar code width                                                               |
| BarcodePrint m             | `GS k $^`                           | `[1D, 6B, $^]`     | ✔️  | ✔️  | Barcode  | Print bar code                                                                   |
| BarcodePositionSet 1       | `GS x $n`                           | `[1D, 78, $n]`     | ✔️  | ✔️  | Barcode  | Set barcode printing left space                                                  |
| QRCode (TODO) m            | `GS ( k pL pH cn fn n1 n2 (fn=65)`  | `[1D, 28, 6B, $^]` | ✔️  | ❌  | QRCode   | Specify the mode of QR code by n1                                                | (fn=65/0x41)
| QRCode (TODO) m            | `GS ( k pL pH cn fn $n (fn=67)`     | `[1D, 28, 6B, $^]` | ✔️  | ❌  | QRCode   | Set the type of QR code graphic module                                           | (fn=67/0x43)
| QRCode (TODO) m            | `GS ( k pL pH cn fn $n (fn=69)`     | `[1D, 28, 6B, $^]` | ✔️  | ❌  | QRCode   | Set the error correction level error of QR code                                  | (fn=69/0x45)
| QRCode (TODO) m            | `GS ( k pL pH cn fn m d1…dk(fn=80)` | `[1D, 28, 6B, $^]` | ✔️  | ❌  | QRCode   | The data stored for receiving QR codes is in a 2d barcode area                   | (fn=80/0x50)
| QRCode (TODO) m            | `GS ( k pL pH cn fn m (fn=82)`      | `[1D, 28, 6B, $^]` | ✔️  | ❌  | QRCode   | The data information types that transmit QR code graphics are in 2d barcode area | (fn=82/0x52)
| SettingsSet m              | `ESC 7 $n $n $n`                    | `[1B, 37, $^]`     | ✔️  | ✔️  | Misc.    | Setting Control Parameter Command                                                |
| SleepTimeoutSet m          | `ESC 8 $n $n`                       | `[1B, 38, $^]`     | ❌  | ✔️  | Misc.    | Select sleeping parameter                                                        |
| ChineseCodeFormatSet 1     | `ESC 9 $n`                          | `[1B, 39, $n]`     | ✔️  | ✔️  | Misc.    | Select Chinese code format                                                       |
| PrintingDensitySet 1       | `DC2 # $n`                          | `[12, 23, $n]`     | ❌  | ✔️  | Misc.    | Set printing density                                                             |
| PrintTestPage              | `DC2 T`                             | `[12, 54]`         | ✔️  | ✔️  | Misc.    | Printing test page                                                               |
| ButtonsEnable 1            | `ESC c 5 $n`                        | `[1B, 63, 35, $n]` | ✔️  | ✔️  | Misc.    | Enable/disable panel buttons (For button)                                        |
| RealTimeEnable 1           | `DLE EOT $n`                        | `[10, 04, $n]`     | ✔️  | ❌  | Special  | Real-time transmission mode                                                      |
| FeedToMark                 | `DC2 E`                             | `[12, 45]`         | ❌  | ✔️  | Special  | Feed Paper to Mark                                                               |
| MarkPaperLengthSet m       | `DC2 m $n $n $n`                    | `[12, 6D, $^]`     | ❌  | ✔️  | Special  | Set Mark paper length                                                            |
| (Device Specific) 1        | `FS t $n`                           | `[1C, 74, $n]`     | ❌  | ✔️  | Device   | Select Time out (for CB105B)                                                     |
| (Device Specific) 1        | `ESC C $n`                          | `[1B, 43, $n]`     | ❌  | ✔️  | Device   | Set BM Max (For BM)                                                              |
| (Device Specific)          | `GS FF`                             | `[1D, 0C]`         | ❌  | ✔️  | Device   | Feed marked paper to print starting position (For BM)                            |
| (Device Specific)          | `ESC i`                             | `[1B, 69]`         | ❌  | ✔️  | Device   | Cut Paper (For cut)                                                              |
| (Device Specific)          | `ESC m`                             | `[1B, 6D]`         | ❌  | ✔️  | Device   | Partial Cut Paper (For cut)                                                      |
| (Device Specific) m        | `GS V $^`                           | `[1D, 56, $^]`     | ❌  | ✔️  | Device   | Select cut mode and cut paper (For cut)                                          |
| (Device Specific) m        | `ESC p $^`                          | `[1B, 70, $^]`     | ❌  | ✔️  | Device   | Generate pulse (For drawer)                                                      |
| (Device Specific) m        | `GS ( F $^`                         | `[1D, 28, 46, $^]` | ❌  | ✔️  | Device   | Set adjustment values(s) (For 701BM)                                             |
| (Device Specific)          | `FS C`                              | `[1C, 43]`         | ❌  | ✔️  | Device   | Start receive buffer count (For cb65c)                                           |
| (Device Specific)          | `FS S`                              | `[1C, 53]`         | ❌  | ✔️  | Device   | Send receive buffer count(For cb65c)                                             |
| (Device Specific)          | `FS s`                              | `[1C, 73]`         | ❌  | ✔️  | Device   | save print parameter (for XBLY)                                                  |
| (Device Specific)          | `FS d`                              | `[1C, 64]`         | ❌  | ✔️  | Device   | Load default leaving factory set (for XBLY)                                      |
