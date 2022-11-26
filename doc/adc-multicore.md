## Analog to digital converter and multicore

Example code [demo-adc](../app/demo-adc/src/main.rs)

![stlink v2 photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/adc/adc.gif)

Note: real display does not suffer any flickering, it is caused by the
combination of camera and exposure time of individual frames.

## Background

I wanted to use a potentiometer as a user input of value between 0 and 100 percent.
A 10 KOhm potentiometer is connected between ground and 3.3V and Rpi Pico Onboard
ADC is used to read the values.

`ADC RAW` value on the display shows the original 12-bit value read from the ADC,
showing a lot of noise. `Norm` value si normalized to 0-10000 range and filtered
using a exponential average filter, visual bar shows the same value.

## Connection

For SSD1306 display connection, look th the [SSD1306 display demo](display-ssd1306.md).
Rpi Pico allows to connect analog ground input on pin 33 and ADC voltage reference
on pin 35 to a different voltage source, but for the sake of simplicity, i just
connected those pins to GND and VCC. Pin 26 is used as ADC input.

| RPI Pico        |     Other          | Potentiometer  |
| --------------- | ------------------ | -------------- |
| AGND (pin 33)   | GND                | pin1           |
| GPIO26 (pin 31) |                    | pin2           |
| VREF (pin 35)   | VCC 3.3            | pin3           |

## Problems

Rpi Pico supports async ADC along with appropriate interrupt, however Rust HAL
for RP2040 does not support this feature yet and only blocking ADC read
is available. This is why I decided to run ADC read loop on Core 1 while
display rendering and communication remained on default Core 0.

Multicore code is taken from the [rp2040-hal repository multicore example](https://github.com/rp-rs/rp-hal/blob/main/rp2040-hal/examples/multicore_fifo_blink.rs).

The code did not seem to work until I realized that second thread on Core 1 never successfully
started after reprogramming the device, but started without problems after reconnecting
power source (which probably reset some internal state causing the problem).
This problem seems to be fixed by a workaround: when `core1.spawn()` returns an
error (usually `Error::Unresponsive`), a software device reset is performed,
which fixes spawning the thread next time just after reset.

```rust
    let spawn_result = core1.spawn(unsafe { &mut CORE1_STACK.mem }, move || {
        adc_loop_on_core1(adc, adc_pin)
    });

    if let Err(_error) = spawn_result {
        // Could not start second core, reset the device
        // This fixed starting up the second core problem after programming with picoprobe
        cortex_m::peripheral::SCB::sys_reset();
    }
```