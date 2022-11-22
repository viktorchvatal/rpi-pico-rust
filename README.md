# Rust Raspberry Pi Pico Learning Demo

My personal walk through learning Rust development on Raspberry Pi Pico

Before switching to Raspberry Pico, I also learned basics of Rust ARM embedded
development using following platforms:
 - Programming Blue Pill development board with STM32F103 microcontroller:
   https://github.com/viktorchvatal/blue-pill-rust

## Userful Resources

 - https://crates.io/crates/rp-pico
 - https://reltech.substack.com/p/getting-started-with-rust-on-a-raspberry

## Prerequisities

Install `elf2uf2-rs` tool used to flash program directly using the USB
mass storage interface

```
cargo install elf2uf2-rs
```

## Getting Started with USB loader

The simplest example just transmits a morse code using the onboard LED:
[demo-blinky](/demo/demo-blinky/src/main.rs)

RP2040 chip contains onboard firmware that allows to flash program in the
USB mass storage mode. In order to do that, it is needed to:

 - connect the Rpi Pico to the USB while BOOTSEL button is active
 - mount the RPI-RP2 device
 - enable the `runner = "elf2uf2-rs -d"` runner in `.cargo/config`
 - run `cargo run` to flash and run the program
 - after run command, device is disconnected and the program is executed,
   to flash another program version, return to the first step and repeat

## Getting started with Picoprobe

USB loader is quite inconvenient because for every firmware update, it is
needed to disconnect the USB, push the BOOTSEL putton, connect the USB and
copy the firmware.

Picoprobe software enables one Rpi Pico to be used as a flasher/debugger for
another Rpi Pico (so buy at least two pieces).

See [Getting started with Picoprobe](doc/picoprobe.md)