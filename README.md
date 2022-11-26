# Rust Raspberry Pi Pico Learning Demo

My personal walk through learning Rust development on Raspberry Pi Pico, featuring
 - dual code ARM Cortex M0+ microcontroller
 - 264K internal RAM and 2MB onboard flash
 - $4 price ($6 for WiFi version)
 - **Pros:** Cheap, good quality, breadboard-friendly, lots of RAM and Flash space, two cores
 - **Cons:** Some features are not supported in Rust (yet)

![stlink v2 photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/boards/rpi-pico-board.jpg)

## Userful Resources

 - https://crates.io/crates/rp-pico
 - https://reltech.substack.com/p/getting-started-with-rust-on-a-raspberry
 - [RPi Pico printable pinout](https://drive.google.com/file/d/1v-ktJeAcibXJ5adw5aSTZiJrBMNNmzJk/view)
 - [Everything about the Raspberry Pi Pico](https://picockpit.com/raspberry-pi/everything-about-the-raspberry-pi-pico/)

## Other Boards

Many experiments with simple hardware, display and sensors can be found in the
[blue-pill-rust repository](https://github.com/viktorchvatal/blue-pill-rust).
Due to the rust `embedded_hal` abstraction, they can be simply modified to run
on RPi Pico as well. Blue pill is a low cost board featuring STM32F103C8
microcontroller with 20K RAM and 64K flash.

![stlink v2 photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/boards/blue-pill-board-small.jpg)

More examples can be found in the [black-pill-rust repository](https://github.com/viktorchvatal/black-pill-rust) with examples for more powerfull STM32F411CEU6 microcontroller
with 128K RAM and 512K flash.

![stlink v2 photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/boards/black-pill-board-small.jpg)

## Getting Started with USB loader

[Programming via USB](doc/usb-loader.md) - the easiest way to flash a program just by connecting
RPI Pico to a USB port with BOOTSEL button active

![stlink v2 photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/blinky/blinky-small.gif)

## Getting started with Picoprobe

[Getting started with Picoprobe](doc/picoprobe.md) - flashing and debugging Pico using another Pico

![stlink v2 photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/picoprobe/rpi-picoprobe-small.jpg)

## SSD1306 OLED I2C Display

[Connecting a small OLED display via i2c bus](doc/display-ssd1306.md)

![stlink v2 photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/display-ssd1306/display-ssd1306-small.gif)

## Analog to digital converter and multicore

[On board ADC and multicore example](doc/adc-multicore.md)

![stlink v2 photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/adc/adc-small.gif)