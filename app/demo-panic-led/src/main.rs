#![no_std]
#![no_main]

use core::panic::PanicInfo;

use embedded_hal::digital::OutputPin;
use rp_pico::{hal::{self, prelude::*, pac, Sio}, entry};
use cortex_m::delay::Delay;

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
    let mut counter = 0;

    loop {
        let _ = led.set_high();
        delay.delay_ms(500);
        let _ = led.set_low();
        delay.delay_ms(500);
        counter += 1;

        // After some time, an error occurs
        if counter > 5 {
            panic!("Something happened");
        }
    }
}

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