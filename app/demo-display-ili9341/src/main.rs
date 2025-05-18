#![no_std]
#![no_main]

mod spi_wrapper;

use arrayvec::ArrayString;
use cortex_m::delay::Delay;
use display_interface_spi::SPIInterface;
use embedded_graphics::{
    mono_font::{ascii::FONT_7X13_BOLD, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::{Point, Primitive, RgbColor, Size, WebColors},
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::{Alignment, Text},
    Drawable
};
use embedded_hal::{digital::OutputPin, spi::{Mode, Phase, Polarity}};
use spi_wrapper::SpiWrapper;
use rp_pico::{hal::{self, pac, Sio, Clock, Timer}, entry};
use core::fmt::Write;

use panic_halt as _;

use ili9341::{DisplaySize240x320, Ili9341, Orientation};
use fugit::RateExtU32;

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

    let sio = Sio::new(pac.SIO);

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led = pins.led.into_push_pull_output();

    // Set up our SPI pins so they can be used by the SPI driver
    let spi_mosi = pins.gpio7.into_function::<hal::gpio::FunctionSpi>();
    let spi_miso = pins.gpio4.into_function::<hal::gpio::FunctionSpi>();
    let spi_sclk = pins.gpio6.into_function::<hal::gpio::FunctionSpi>();
    let spi = hal::spi::Spi::<_, _, _, 8>::new(pac.SPI0, (spi_mosi, spi_miso, spi_sclk));

    let mode = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    };

    // Exchange the uninitialised SPI driver for an initialised one
    let lcd_spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        32.MHz(),
        mode,
    );

    let mut timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let lcd_dc = pins.gpio2.into_push_pull_output();
    let lcd_reset = pins.gpio1.into_push_pull_output();

    let sint = SPIInterface::new(SpiWrapper{ bus: lcd_spi }, lcd_dc);
    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let mut lcd = Ili9341::new(
        sint,
        lcd_reset,
        &mut timer,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    // Create a new character style
    let font_fg = MonoTextStyle::new(&FONT_7X13_BOLD, Rgb565::YELLOW);
    let font_bg = MonoTextStyle::new(&FONT_7X13_BOLD, Rgb565::CSS_DARK_BLUE);

    let mut counter = 0i32;

    let bg_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::CSS_DARK_RED)
        .build();


    Rectangle::new(Point::zero(), Size::new(240, 320)).into_styled(bg_style)
        .draw(&mut lcd)
        .unwrap();

    let text = "ILI9341\nTFT Display\nExample";

    loop {
        let mut iter_text = ArrayString::<10>::new();
        let _ = writeln!(&mut iter_text, "{}", counter);

        let position = Point::new(counter % 220, counter % 300);

        Text::with_alignment(text, position, font_fg, Alignment::Left)
            .draw(&mut lcd)
            .unwrap();

        Rectangle::new(Point::zero(), Size::new(100, 20)).into_styled(bg_style)
            .draw(&mut lcd)
            .unwrap();

        Text::with_alignment(&iter_text, Point::new(0, 12), font_fg, Alignment::Left)
            .draw(&mut lcd)
            .unwrap();

        led.set_low().unwrap();
        delay.delay_ms(10);
        led.set_high().unwrap();

        Text::with_alignment(text, position, font_bg, Alignment::Left)
            .draw(&mut lcd)
            .unwrap();

        counter += 1;
    }
}