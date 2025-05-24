# Panic handling

There are several possibilities how to configure panic behavior in case of some
non-recoverable error.

## Halt on panic

The simplest option is just halt the program on any panic. A `panic-halt`
library does exactly this, it ends the program in the infinite loop on the
panic.

To use `panic-halt`, set the dependency in
[Cargo.toml](../app/demo-blinky-standalone/Cargo.toml)

```
[dependencies]
panic-halt = "1.0.0"
```

and import the library in [main.rs](../app/demo-blinky/src/main.rs)

```
use panic_halt as _;
```

## Custom panic handler and panic LED

A panic handler can be set by defining a function with signature

```
#[panic_handler]
fn on_panic(_info: &PanicInfo) -> !
```

An exclamation mark at the end indicates that the function never returns, so
it is the last code that is run after the panic.

Sometimes the program just stops and I even did not notice it for a while.
In order to clearly see that the program panicked, I connected
a bright blue LED to GPIO22 port to indicate a panic state.
An advantage of the LED is that it works even when debugger is not connected.

Panic handler to set GPIO22 to high state is defined as

```rust
#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    // Steal the peripherals even if another code acquired them before
    // No other code is run after this panic handler so there should
    // be no conflict
    let mut pac = unsafe { pac::Peripherals::steal() };
    let sio = Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);
    let mut panic_led = pins.gpio22.into_push_pull_output();
    // Turn on the LED
    let _ = panic_led.set_high();
    // Infinite loop at the end so we never return
    loop { }
}
```