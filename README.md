# thermal-printer

A project to control a generic thermal printer module, made in Rust.

## Supported Printers

- EM7820
- ...and probably other printers that support ESC/POS commands

## Goals

- Directly control the printer (via USB, RS232, TTL, etc.) or save to a binary data file 
- Provide a more machine-friendly way to structure the layout of a print (Json, Yaml, etc., via `serde`)
- Load in images and automatically convert (and apply dithering) to send to the printer
- Make it work on a Raspberry Pi

## How to use it

- Coming soon™