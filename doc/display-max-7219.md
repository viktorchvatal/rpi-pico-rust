# Digital LED display with MAX7219 Driver

Example code [demo-display-max-7219](../app/demo-display-max-7219/src/main.rs)

Notes:

 - CS pin seems to be needed as connecting CS to GND with a resistor did not work
 - on other boards, a time delay before initialization was needed (display did not correctly initialize), but on raspberry pi pico, there seems to be no problem