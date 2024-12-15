#![no_std]
#![no_main]

use embedded_hal::digital::OutputPin;
use rp_pico::{hal::{self, prelude::*, pac, Sio}, entry};
use cortex_m::delay::Delay;

use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let sio = Sio::new(pac.SIO);

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led = pins.led.into_push_pull_output();

    //let message = ".... . .-.. .-.. ---    ";
    let message = "...-";

    // Transmit the message using the onboard LED
    loop {
        for char in message.chars() {
            match char {
                // Just delay
                ' ' => delay.delay_ms(500),
                // Short blink
                '.' => {
                    led.set_high().unwrap();
                    delay.delay_ms(200);
                    led.set_low().unwrap();
                },
                // Long blink
                '-' => {
                    led.set_high().unwrap();
                    delay.delay_ms(700);
                    led.set_low().unwrap();
                },
                _other => {},
            }
            delay.delay_ms(300);
        }
    }
}