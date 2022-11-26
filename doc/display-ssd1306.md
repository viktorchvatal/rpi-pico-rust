# SSD1306 OLED I2C Display

Example code [demo-display-ssd1306](../app/demo-display-ssd1306/src/main.rs)

![stlink v2 photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/display-ssd1306/display-ssd1306.gif)

Driving SSD1306 display is really straightforward as it uses standard i2c bus
that requires connection of only two data pins. `ssd1306` library works perfectly
in tandem with `embedded_graphics`, so it is easy to render any content.

## Connection

| RPI Pico         |     Other          | SSD1306      |
| ---------------- | ------------------ | ------------ |
| GPIO16 (pin 21)  | 5k pull up         | SDA          |
| GPIO17 (pin 22)  | 5k pull up         | SCK          |
| -                | VCC 3.3            | VCC          |
| -                | GND                | GND          |

