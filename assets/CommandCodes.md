# Command Codes

Most, if not all have been untested.
There is some nuance between the two sourced documentation, this is a collection of all commands available.

## Table

> Variables:
>
> -   `$n` = One variable
> -   `$N` = One or more variables (Null terminated?)
> -   `$^` = See documentation (Many Parameters)

| Ascii Code                          | Hex Sequence       | D1  | D2  | Category | Short Description                                                                |
| ----------------------------------- | ------------------ | --- | --- | -------- | -------------------------------------------------------------------------------- |
| `LF`                                | `[0A]`             | ✔️  | ✔️  | Print    | Print and line feed                                                              |
| `CR`                                | `[0D]`             | ✔️  | ✔️  | Print    | Print and carriage return                                                        |
| `HT`                                | `[09]`             | ✔️  | ✔️  | Print    | Horizontal tab                                                                   |
| `FF`                                | `[0C]`             | ❌  | ✔️  | Print    | Print the data in buffer                                                         |
| `ESC D $N`                          | `[1B, 44, $N, 00]` | ✔️  | ✔️  | Print    | Set horizontal tab positions                                                     |
| `ESC J $n`                          | `[1B, 4A, $n]`     | ✔️  | ✔️  | Print    | Print and Feed $n dots paper                                                     |
| `ESC d $n`                          | `[1B, 64, $n]`     | ✔️  | ✔️  | Print    | Print and Feed $n lines                                                          |
| `ESC = $n`                          | `[1B, 3D, $n]`     | ✔️  | ✔️  | Print    | Set peripheral device                                                            |
| `ESC 2`                             | `[1B, 32]`         | ✔️  | ✔️  | Line     | Select default line spacing                                                      |
| `ESC 3 $n`                          | `[1B, 33, $n]`     | ✔️  | ✔️  | Line     | Set line spacing                                                                 |
| `ESC a $n`                          | `[1B, 61, $n]`     | ✔️  | ✔️  | Line     | Select justification                                                             |
| `ESC SO $n`                         | `[1B, 0E, $n]`     | ✔️  | ✔️  | Line     | Select Double Width mode                                                         |
| `ESC DC4 $n`                        | `[1B, 14, $n]`     | ✔️  | ✔️  | Line     | Disable Double Width mode                                                        |
| `GS L nL nH`                        | `[1D, 4C, $n, $n]` | ✔️  | ✔️  | Line     | Set left margin                                                                  |
| `ESC $ nL nH`                       | `[1B, 24, $n, $n]` | ✔️  | ✔️  | Line     | Set absolute print position                                                      |
| `ESC B $n`                          | `[1B, 42, $n]`     | ✔️  | ✔️  | Line     | Set Left Space                                                                   |
| `ESC ! $n`                          | `[1B, 21, $n]`     | ✔️  | ✔️  | Char     | Select print mode(s)                                                             |
| `GS ! $n`                           | `[1D, 21, $n]`     | ✔️  | ✔️  | Char     | Select character size                                                            |
| `GS B $n`                           | `[1D, 42, $n]`     | ✔️  | ✔️  | Char     | Turn white/black reverse printing mode                                           |
| `ESC V $n`                          | `[1B, 56, $n]`     | ✔️  | ✔️  | Char     | Turn 90°clockwise rotation mode on/off                                           |
| `ESC G $n`                          | `[1B, 47, $n]`     | ✔️  | ✔️  | Char     | Turn on/off double-strike mode                                                   |
| `ESC E $n`                          | `[1B, 69, $n]`     | ✔️  | ✔️  | Char     | Turn emphasized mode on/off                                                      |
| `ESC SP $n`                         | `[1B, 20, $n]`     | ❌  | ✔️  | Char     | Set right-side character spacing                                                 |
| `ESC SO $n`                         | `[1B, 0E, $n]`     | ✔️  | ✔️  | Char     | Select Double Width mode                                                         |
| `ESC { $n`                          | `[1B, 7B, $n]`     | ✔️  | ✔️  | Char     | Turns on/off upside-down printing mode                                           |
| `ESC - $n`                          | `[1B, 2D, $n]`     | ✔️  | ✔️  | Char     | Set the underline dots(0,1,2)                                                    |
| `ESC % $n`                          | `[1B, 25, $n]`     | ✔️  | ✔️  | Char     | Select/Cancel user-defined characters                                            |
| `FS &`                              | `[1C, 26]`         | ✔️  | ✔️  | Char     | Select Kanji character mode                                                      |
| `FS .`                              | `[1C, 2E]`         | ✔️  | ✔️  | Char     | Cancel Kanji character mode                                                      |
| `FS ! $n`                           | `[1C, 21, $n]`     | ✔️  | ✔️  | Char     | Set print mode for Kanji characters                                              |
| `ESC & $^`                          | `[1B, 26, $^]`     | ✔️  | ✔️  | Char     | Define user-defined characters                                                   |
| `ESC ? $n`                          | `[1B, 3F, $n]`     | ✔️  | ✔️  | Char     | Cancel user-defined characters                                                   |
| `ESC R $n`                          | `[1B, 52, $n]`     | ✔️  | ✔️  | Char     | Select and international character set                                           |
| `ESC t $n`                          | `[1B, 74, $n]`     | ✔️  | ✔️  | Char     | Select character code table                                                      |
| `ESC * $^`                          | `[1B, 2A, $^]`     | ✔️  | ✔️  | BMP      | Select bit-image mode                                                            |
| `GS * $^`                           | `[1D, 2A, $^]`     | ✔️  | ✔️  | BMP      | Define downloaded bit image                                                      |
| `GS / $n`                           | `[1D, 2F, $n]`     | ✔️  | ✔️  | BMP      | Print downloaded bit image                                                       |
| `GS v 0 $^`                         | `[1D, 76, 30, $^]` | ✔️  | ✔️  | BMP      | Print raster bit image                                                           |
| `DC2 * $^`                          | `[12, 2A, $^]`     | ❌  | ✔️  | BMP      | Print the bitmap                                                                 |
| `DC2 V`                             | `[12, 56, $^]`     | ❌  | ✔️  | BMP      | Print MSB bitmap                                                                 |
| `DC2 v`                             | `[12, 76, $^]`     | ❌  | ✔️  | BMP      | Print LSB bitmap                                                                 |
| `FS p $n $n`                        | `[1C, 70, $n, $n]` | ✔️  | ✔️  | BMP      | Print NV bitmap                                                                  |
| `FS q $^`                           | `[1C, 71, $^]`     | ✔️  | ✔️  | BMP      | Define NV bitmap                                                                 |
| `ESC @`                             | `[1B, 40]`         | ✔️  | ✔️  | Init     | Initialize the printer                                                           |
| `GS r $n`                           | `[1D, 72, $n]`     | ✔️  | ✔️  | Status   | Transmit status                                                                  |
| `GS a $n`                           | `[1D, 61, $n]`     | ✔️  | ✔️  | Status   | Enable/Disable Automatic Status Back(ASB)                                        |
| `ESC v $n`                          | `[1B, 76, $n]`     | ✔️  | ✔️  | Status   | Transmit paper sensor status                                                     |
| `ESC u $n`                          | `[1B, 75, $n]`     | ❌  | ✔️  | Status   | Transmit peripheral device status (For drawer)                                   |
| `GS H $n`                           | `[1D, 48, $n]`     | ✔️  | ✔️  | Barcode  | Select printing position for HRI characters                                      |
| `GS h $n`                           | `[1D, 68, $n]`     | ✔️  | ✔️  | Barcode  | Set bar code height                                                              |
| `GS w $n`                           | `[1D, 77, $n]`     | ✔️  | ✔️  | Barcode  | Set bar code width                                                               |
| `GS k $^`                           | `[1D, 6B, $^]`     | ✔️  | ✔️  | Barcode  | Print bar code                                                                   |
| `GS x $n`                           | `[1D, 78, $n]`     | ✔️  | ✔️  | Barcode  | Set barcode printing left space                                                  |
| `GS ( k pL pH cn fn n1 n2 (fn=65)`  | `[1D, 28, $^]`     | ✔️  | ❌  | QRCode   | Specify the mode of QR code by n1                                                |
| `GS ( k pL pH cn fn $n (fn=67)`     | `[1D, 28, $^]`     | ✔️  | ❌  | QRCode   | Set the type of QR code graphic module                                           |
| `GS ( k pL pH cn fn $n (fn=69)`     | `[1D, 28, $^]`     | ✔️  | ❌  | QRCode   | Set the error correction level error of QR code                                  |
| `GS ( k pL pH cn fn m d1…dk(fn=80)` | `[1D, 28, $^]`     | ✔️  | ❌  | QRCode   | The data stored for receiving QR codes is in a 2d barcode area                   |
| `GS ( k pL pH cn fn m (fn=82)`      | `[1D, 28, $^]`     | ✔️  | ❌  | QRCode   | The data information types that transmit QR code graphics are in 2d barcode area |
| `ESC 7 $n $n $n`                    | `[1B, 37, $^]`     | ✔️  | ✔️  | Misc.    | Setting Control Parameter Command                                                |
| `ESC 8 $n $n`                       | `[1B, 38, $^]`     | ❌  | ✔️  | Misc.    | Select sleeping parameter                                                        |
| `ESC 9 $n`                          | `[1B, 39, $n]`     | ✔️  | ✔️  | Misc.    | Select Chinese code format                                                       |
| `DC2 # $n`                          | `[12, 23, $n]`     | ❌  | ✔️  | Misc.    | Set printing density                                                             |
| `DC2 T`                             | `[12, 54]`         | ✔️  | ✔️  | Misc.    | Printing test page                                                               |
| `ESC c 5 $n`                        | `[1B, 63, 35, $n]` | ✔️  | ✔️  | Misc.    | Enable/disable panel buttons (For button)                                        |
| `DLE EOT $n`                        | `[10, 04, $n]`     | ✔️  | ❌  | Special  | Real-time transmission mode                                                      |
| `FS t $n`                           | `[1C, 74, $n]`     | ❌  | ✔️  | Device   | Select Time out (for CB105B)                                                     |
| `DC2 E`                             | `[12, 45]`         | ❌  | ✔️  | Special  | Feed Paper to Mark                                                               |
| `DC2 m $n $n $n`                    | `[12, 6D, $^]`     | ❌  | ✔️  | Special  | Set Mark paper length                                                            |
| `ESC C $n`                          | `[1B, 43, $n]`     | ❌  | ✔️  | Device   | Set BM Max (For BM)                                                              |
| `GS FF`                             | `[1D, 0C]`         | ❌  | ✔️  | Device   | Feed marked paper to print starting position (For BM)                            |
| `ESC i`                             | `[1B, 69]`         | ❌  | ✔️  | Device   | Cut Paper (For cut)                                                              |
| `ESC m`                             | `[1B, 6D]`         | ❌  | ✔️  | Device   | Partial Cut Paper (For cut)                                                      |
| `GS V $^`                           | `[1D, 56, $^]`     | ❌  | ✔️  | Device   | Select cut mode and cut paper (For cut)                                          |
| `ESC p $^`                          | `[1B, 70, $^]`     | ❌  | ✔️  | Device   | Generate pulse (For drawer)                                                      |
| `GS ( F $^`                         | `[1D, 28, 46, $^]` | ❌  | ✔️  | Device   | Set adjustment values(s) (For 701BM)                                             |
| `FS C`                              | `[1C, 43]`         | ❌  | ✔️  | Device   | Start receive buffer count (For cb65c)                                           |
| `FS S`                              | `[1C, 53]`         | ❌  | ✔️  | Device   | Send receive buffer count(For cb65c)                                             |
| `FS s`                              | `[1C, 73]`         | ❌  | ✔️  | Device   | save print parameter (for XBLY)                                                  |
| `FS d`                              | `[1C, 64]`         | ❌  | ✔️  | Device   | Load default leaving factory set (for XBLY)                                      |
