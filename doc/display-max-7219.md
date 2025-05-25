# Digital LED display with MAX7219 Driver

Example code [demo-display-max-7219](../app/demo-display-max-7219/src/main.rs)

![photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/display-max-7219/display-max-7219.gif)

This is very cheap and convenient display to use, requiring only 5 wires for operation. Library is very
easy to user, allowing both high level functions to send directly text string to be rendered, ot raw digit data
to be displayed.

Notes:

 - CS pin seems to be needed as connecting CS to GND with a resistor did not work
 - on other boards, a time delay before initialization was needed (display did not correctly initialize), but on raspberry pi pico, there seems to be no problem
 - display can be powered directly from 3.3 voltage stabilizer on Raspberry Pi Pico, but is a little dim in this case
 - display can also be poweres from 5V bus voltage and is very bright, communication from 3.3V logic also seems
   to be working, but is a little out of spec, so might not be reliable, so 3.3V to 5V level shifter should be used