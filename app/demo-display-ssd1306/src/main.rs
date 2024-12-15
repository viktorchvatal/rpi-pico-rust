#![no_std]
#![no_main]

use embedded_hal::digital::OutputPin;
use rp_pico::{hal::{self, pac, Sio}, entry};

use panic_halt as _;
use ssd1306::{
    I2CDisplayInterface, size::DisplaySize128x64, rotation::DisplayRotation, Ssd1306,
    prelude::{DisplayConfig, Brightness}
};
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Circle},
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    text::Text
};
use fugit::{RateExtU32};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();

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

    let sio = Sio::new(pac.SIO);

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led = pins.led.into_push_pull_output();

    let sda_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, _> = pins.gpio16.reconfigure();
    let scl_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, _> = pins.gpio17.reconfigure();

    let i2c = hal::I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();
    display.set_brightness(Brightness::BRIGHTEST).unwrap();

    let mut size_1 = 80;
    let mut size_2 = 0;

    loop {
        display.clear();

        let origin = Point::new(1, 1);
        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        Circle::new(origin, size_1).into_styled(style).draw(&mut display).unwrap();
        Circle::new(origin, size_2).into_styled(style).draw(&mut display).unwrap();
        let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        let position = Point::new(40, 60);
        Text::new("Hello RPI Pico!", position, style).draw(&mut display).unwrap();

        size_1 = (size_1 + 1) % 160;
        size_2 = (size_2 + 1) % 160;

        led.set_high().unwrap();
        display.flush().unwrap();
        led.set_low().unwrap();
   }
}