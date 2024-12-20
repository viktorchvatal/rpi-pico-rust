#![no_std]
#![no_main]

use core::{fmt::Write, cmp::{min, max}, sync::atomic::{AtomicU32, Ordering}};
use arrayvec::ArrayString;
use cortex_m::{prelude::_embedded_hal_adc_OneShot, delay::Delay};
use embedded_hal::digital::OutputPin;
use hal::{Clock, multicore::{Stack, Multicore}, Adc, gpio::{Pin, bank0::Gpio26}};
use rp2040_hal::{adc::AdcPin, gpio::{FunctionSio, PullNone, SioInput}};
use rp_pico::{hal::{self, pac, Sio}, entry};

use panic_halt as _;
use ssd1306::{
    I2CDisplayInterface, size::DisplaySize128x64, rotation::DisplayRotation, Ssd1306,
    prelude::{DisplayConfig, Brightness}
};
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    text::Text, primitives::{Rectangle, PrimitiveStyle},
};
use fugit::RateExtU32;

static mut CORE1_STACK: Stack<4096> = Stack::new();

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

    let mut sio = Sio::new(pac.SIO);

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let adc = hal::Adc::new(pac.ADC, &mut pac.RESETS);
    let adc_pin = AdcPin::new(pins.gpio26.into_floating_input()).unwrap();

    let mut mc = Multicore::new(&mut pac.PSM, &mut pac.PPB, &mut sio.fifo);
    let cores = mc.cores();
    let core1 = &mut cores[1];

    let spawn_result = core1.spawn(unsafe { &mut CORE1_STACK.mem }, move || {
        adc_loop_on_core1(adc, adc_pin)
    });

    if let Err(_error) = spawn_result {
        // Could not start second core, reset the device
        // This fixed starting up the second core problem after programming with picoprobe
        cortex_m::peripheral::SCB::sys_reset();
    }

    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

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

    delay.delay_ms(50);
    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();

    loop {
        led.set_high().unwrap();
        display.set_brightness(Brightness::BRIGHTEST).unwrap();

        let adc_value = unsafe { ADC_VALUE.load(Ordering::Relaxed) };
        let normalized = unsafe { NORMALIZED.load(Ordering::Relaxed) };

        let mut text = ArrayString::<50>::new();
        let _ = writeln!(&mut text, "ADC RAW {}", adc_value);
        let _ = writeln!(&mut text, "Norm {}", normalized);

        let _ = display.clear(BinaryColor::Off);

        let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        let position = Point::new(0, 20);
        Text::new(&text, position, style).draw(&mut display).unwrap();
        render_bar(&mut display, normalized).unwrap();
        led.set_low().unwrap();

        display.flush().unwrap();
   }
}

fn render_bar<T, E>(
    display: &mut T,
    value: u32
) -> Result<(), ()>
where T: DrawTarget<Color = BinaryColor, Error = E> {
    const POSITION: Point = Point { x: 1, y: 48 };
    const SIZE: Size = Size { width: 126, height: 15 };

    let filled_size = Size {
        height: SIZE.height,
        width: SIZE.width*value/MAX_VALUE
    };

    let outline_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let filled_style = PrimitiveStyle::with_fill(BinaryColor::On);

    Rectangle::new(POSITION, SIZE)
        .into_styled(outline_style)
        .draw(display)
        .map_err(|_| ())?;

    Rectangle::new(POSITION, filled_size)
        .into_styled(filled_style)
        .draw(display)
        .map_err(|_| ())?;

    Ok(())
}

static mut ADC_VALUE: AtomicU32 = AtomicU32::new(0);
static mut NORMALIZED: AtomicU32 = AtomicU32::new(0);

const MAX_VALUE: u32 = 10000;

fn adc_loop_on_core1(
    mut adc: Adc,
    mut adc_pin: AdcPin<Pin<Gpio26, FunctionSio<SioInput>, PullNone>>
) -> ! {
    let mut filter = GeometricFilter::new(0.0001, 0);

    let mut adc_value = 0;

    loop {
        if let Some(new_value) = adc.read(&mut adc_pin).ok() {
            adc_value = new_value as u32;
            filter.add(new_value);
        }

        let filtered = filter.get_average() as u32;

        const MIN: u32 = 40;
        const MAX: u32 = 3900;

        let clamped = min(max(filtered, MIN), MAX);
        let normalized = (clamped - MIN)*MAX_VALUE/(MAX - MIN);

        unsafe {
            ADC_VALUE.store(adc_value, Ordering::Relaxed);
            NORMALIZED.store(normalized, Ordering::Relaxed);
        }
    }
}

struct GeometricFilter {
    value: u32,
    weight_actual: u32,
    weight_new: u32
}

const GEOMETRIC_MULTIPLIER: u32 = u32::MAX;

impl GeometricFilter {
    pub fn new(weight: f32, value: u16) -> Self {
        let value = (value as u32) << 16;
        let weight_new = (GEOMETRIC_MULTIPLIER as f32*weight) as u32;
        let weight_actual = GEOMETRIC_MULTIPLIER - weight_new;

        Self { value, weight_new, weight_actual }
    }

    pub fn add(&mut self, value: u16) {
        let actual_part = self.value as u64*self.weight_actual as u64;
        let new_part = ((value as u64) << 16)*self.weight_new as u64;
        let new_value = (actual_part + new_part)/GEOMETRIC_MULTIPLIER as u64;
        self.value = new_value as u32;
    }

    pub fn get_average(&self) -> u16 {
        (self.value >> 16) as u16
    }
}
