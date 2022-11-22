#![no_std]
#![no_main]

use core::fmt::Write;
use arrayvec::ArrayString;
use cortex_m::prelude::_embedded_hal_adc_OneShot;
use embedded_hal::digital::v2::OutputPin;
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

    let sda_pin = pins.gpio16.into_mode::<hal::gpio::FunctionI2C>();
    let scl_pin = pins.gpio17.into_mode::<hal::gpio::FunctionI2C>();

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

    let mut adc = hal::Adc::new(pac.ADC, &mut pac.RESETS);
    let mut adc_pin = pins.gpio26.into_floating_input();

    display.init().unwrap();
    display.set_brightness(Brightness::BRIGHTEST).unwrap();

    loop {
        let adc_value: u16 = adc.read(&mut adc_pin).unwrap();
        display.clear();

        let mut text = ArrayString::<50>::new();
        let _ = write!(&mut text, "ADC demo\n    {}", adc_value);

        let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        let position = Point::new(0, 20);
        Text::new(&text, position, style).draw(&mut display).unwrap();

        led.set_high().unwrap();
        display.flush().unwrap();
        led.set_low().unwrap();
   }
}