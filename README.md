# Rust Raspberry Pi Pico Learning Demo

My personal walk through learning Rust development on Raspberry Pi Pico, featuring
 - dual code ARM Cortex M0+ microcontroller
 - 264K internal RAM and 2MB onboard flash
 - $4 price ($6 for WiFi version)
 - **Pros:** Cheap, good quality, breadboard-friendly, lots of RAM and Flash space, two cores
 - **Cons:** Some features are not supported in Rust (yet)

![photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/boards/rpi-pico-board.jpg)

## Userful Resources

 - https://crates.io/crates/rp-pico

   - examples: https://github.com/rp-rs/rp-hal-boards/tree/main/boards/rp-pico/examples

 - https://crates.io/crates/rp2040-hal

   - examples: https://github.com/rp-rs/rp-hal/tree/main/rp2040-hal-examples/src/bin

 - https://reltech.substack.com/p/getting-started-with-rust-on-a-raspberry
 - [RPi Pico printable pinout](https://drive.google.com/file/d/1v-ktJeAcibXJ5adw5aSTZiJrBMNNmzJk/view)
 - [Everything about the Raspberry Pi Pico](https://picockpit.com/raspberry-pi/everything-about-the-raspberry-pi-pico/)

## Other ARM Boards

Originally, my goal was to find a cheap ARM board that could be programmed in Rust programming
language. First one I got was Blue pill - a small and low cost board with STM32F103C8
microcontroller and 20K RAM and 64K flash.

My experiments with this board can be found in
[blue-pill-rust repository](https://github.com/viktorchvatal/blue-pill-rust).

![photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/boards/blue-pill-board-small.jpg)

Speaking of Blue pill, I had problems to acquire cheap boards that did not carry fake STM microcontrollers.
This is why I found Black pill - a successor with more powerfull STM32F411CEU6 microcontroller
with 128K RAM and 512K flash.

More examples can be found in the [black-pill-rust repository](https://github.com/viktorchvatal/black-pill-rust).

![photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/boards/black-pill-board-small.jpg)

However, Black pill was not so low cost any more. When Raspberry Pi Pico was announced, I was very happy
about it. For very low cost, I got really powerfull ARM microprocessor with lots of RAM and onboard flash,
with no problem about component quality.

## Basic Programming and Debugging

[Getting Started with USB loader](doc/usb-loader.md) - the easiest way to flash a program just by connecting
RPI Pico to a USB port with BOOTSEL button active

![photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/blinky/blinky-small.gif)

[Getting started with Picoprobe](doc/picoprobe.md) - flashing and debugging Pico using another Pico

![photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/picoprobe/rpi-picoprobe-small.jpg)

[Indicating panic with a LED](doc/panic-led.md)

TODO: Photo

## Digital Displays

[LED display with MAX7219 driver over SPI](doc/display-max-7219.md)

![photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/display-max-7219/display-max-7219-small.gif)

[SSD1306 OLED I2C Display](doc/display-ssd1306.md)

![photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/display-ssd1306/display-ssd1306-small.gif)

## Other Inputs and Outputs

[Analog to digital converter and multicore](doc/adc-multicore.md)

![photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/adc/adc-small.gif)

[PWM driven colored LEDs](doc/pwm-colors.md)

![photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/pwm-colors/pwm-colors-small.gif)