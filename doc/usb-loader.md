## Getting Started with USB loader

![stlink v2 photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/blinky/blinky.gif)

## Prerequisities

In order to start with RPi Pico deaturing a Cortex M0+ ARM cores, it is needed to
install the `thumbv6m-none-eabi` toolchain

```
rustup target install thumbv6m-none-eabi
```

Install `elf2uf2-rs` tool used to flash program directly using the USB
mass storage interface

```
cargo install elf2uf2-rs
```

## Flashing the first program

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

In order to flash programs more easily or to use a debugger, it is needed
to setup a [picoprobe](picoprobe.md)

## Notes

To resolve errors with `#![no_std]` and `rust_analyzer` in `vscode`, set
`"rust-analyzer.check.allTargets": false,`