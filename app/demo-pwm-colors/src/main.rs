#![no_std]
#![no_main]

use core::{cmp::{min, max}, sync::atomic::{AtomicU32, Ordering}, fmt::Write};
use arrayvec::ArrayString;
use cortex_m::{prelude::_embedded_hal_adc_OneShot, delay::Delay};
use embedded_hal::{digital::v2::OutputPin, PwmPin};
use hal::{Clock, multicore::{Stack, Multicore}, Adc, gpio::{Pin, bank0::Gpio26, Input, Floating}};
use rp_pico::{hal::{self, pac, Sio}, entry};
use micromath::F32Ext;

use panic_halt as _;
use ssd1306::{
    I2CDisplayInterface, size::DisplaySize128x64, rotation::DisplayRotation, Ssd1306,
    prelude::{DisplayConfig, Brightness}
};
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    mono_font::{MonoTextStyle, ascii::FONT_7X13_BOLD},
    text::Text, primitives::{Rectangle, PrimitiveStyle},
};
use fugit::{RateExtU32};

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
    let adc_pin = pins.gpio26.into_floating_input();

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

    delay.delay_ms(50);
    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    let _ = display.init();

    // Initialize PWM, only certain channel and pic combinations are valid,
    // refer to RP2040 datasheet (1.4.3. GPIO Functions) for details
    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    let pwm6 = &mut pwm_slices.pwm6;
    pwm6.set_ph_correct();
    pwm6.enable();

    let pwm7 = &mut pwm_slices.pwm7;
    pwm7.set_ph_correct();
    pwm7.enable();

    let channel_r = &mut pwm6.channel_b;
    let channel_g = &mut pwm7.channel_a;
    let channel_b = &mut pwm7.channel_b;

    channel_r.output_to(pins.gpio13);
    channel_g.output_to(pins.gpio14);
    channel_b.output_to(pins.gpio15);

    loop {
        led.set_high().unwrap();
        let _ = display.set_brightness(Brightness::BRIGHTEST);

        let normalized = unsafe { NORMALIZED.load(Ordering::Relaxed) };

        let input = normalized as f32;
        let max_log_val = (MAX_VALUE as f32).ln();
        let r = max(1500, minf(input/8000.0 + 7.0, max_log_val).exp() as u16);
        let g = minf(input/8000.0 + 5.0, max_log_val).exp() as u16;
        let b = minf(input/8000.0 + 3.0, max_log_val).exp() as u16;

        channel_r.set_duty(r);
        channel_g.set_duty(g);
        channel_b.set_duty(b);

        display.clear();

        render_bar(&mut display, "I", 1, normalized as u16).unwrap();
        render_bar(&mut display, "R", 15, r).unwrap();
        render_bar(&mut display, "G", 30, g).unwrap();
        render_bar(&mut display, "B", 45, b).unwrap();
        led.set_low().unwrap();

        let _ = display.flush();
   }
}

fn minf(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}


fn render_bar<T, E>(
    display: &mut T,
    name: &str,
    top: i32,
    value: u16
) -> Result<(), ()>
where T: DrawTarget<Color = BinaryColor, Error = E> {
    let position = Point { x: 70, y: top };
    const SIZE: Size = Size { width: 56, height: 10 };

    let filled_size = Size {
        height: SIZE.height,
        width: SIZE.width*value as u32/MAX_VALUE
    };

    let outline_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let filled_style = PrimitiveStyle::with_fill(BinaryColor::On);

    Rectangle::new(position, SIZE)
        .into_styled(outline_style)
        .draw(display)
        .map_err(|_| ())?;

    Rectangle::new(position, filled_size)
        .into_styled(filled_style)
        .draw(display)
        .map_err(|_| ())?;

    let style = MonoTextStyle::new(&FONT_7X13_BOLD, BinaryColor::On);
    let mut text = ArrayString::<15>::new();
    let _ = write!(&mut text, "{}: {:5}", name, value);
    Text::new(&text, Point::new(1, top + 8), style).draw(display).map_err(|_| ())?;

    Ok(())
}

static mut ADC_VALUE: AtomicU32 = AtomicU32::new(0);
static mut NORMALIZED: AtomicU32 = AtomicU32::new(0);

const MAX_VALUE: u32 = u16::MAX as u32;

fn adc_loop_on_core1(
    mut adc: Adc,
    mut adc_pin: Pin<Gpio26, Input<Floating>>
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
        const MAX: u32 = 3700;

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
