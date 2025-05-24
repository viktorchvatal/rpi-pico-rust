#![no_std]
#![no_main]

mod spi_wrapper;
use embedded_hal::{digital::OutputPin, spi::{Mode, Phase, Polarity}};
use rp_pico::{hal::{self, prelude::*, pac, Sio}, entry};
use cortex_m::delay::Delay;
use max7219::MAX7219;
use fugit::RateExtU32;
use panic_halt as _;
use spi_wrapper::SpiWrapper;

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

    let spi_mosi = pins.gpio3.into_function::<hal::gpio::FunctionSpi>();
    let spi_sclk = pins.gpio2.into_function::<hal::gpio::FunctionSpi>();
    let cs = pins.gpio4.into_push_pull_output();
    let display_spi = hal::spi::Spi::<_, _, _, 8>::new(pac.SPI0, (spi_mosi, spi_sclk));

    let mode = Mode { polarity: Polarity::IdleLow, phase: Phase::CaptureOnFirstTransition };

    let display_spi = display_spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        1.MHz(),
        mode,
    );

    let spi_wrapper = SpiWrapper{ bus: display_spi };
    let mut display = MAX7219::from_spi_cs(1, spi_wrapper, cs).unwrap();

    let buffer = b"        Hello RP-2040        ";
    let mut shift: usize = 0;
    let mut data = [0; 8];

    loop {
        let _  = led.set_high();
        display.power_on().unwrap();
        data.copy_from_slice(&buffer[shift..(shift + 8)]);

        display.write_str(0, &data, 0x00).unwrap();
        display.set_intensity(0, 0x02).unwrap();
        shift = (shift + 1) % 21;
        let _  = led.set_low();

        delay.delay_ms(300);
    }
}