#![no_std]
#![no_main]

use core::{fmt::Write, cmp::{min, max}, sync::atomic::{AtomicU32, Ordering}};
use arrayvec::ArrayString;
use cortex_m::{prelude::_embedded_hal_adc_OneShot, delay::Delay};
use embedded_hal::digital::v2::OutputPin;
use fixed_queue::VecDeque;
use hal::{Clock, multicore::{Stack, Multicore}, Adc, gpio::{Pin, bank0::Gpio26, Input, Floating}};
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
    text::Text
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

    display.init().unwrap();

    loop {
        led.set_high().unwrap();
        display.set_brightness(Brightness::BRIGHTEST).unwrap();

        let adc_value = unsafe { ADC_VALUE.load(Ordering::Relaxed) };
        let filtered = unsafe { FILTERED.load(Ordering::Relaxed) };
        let bounded = unsafe { NORMALIZED.load(Ordering::Relaxed) };

        let mut text = ArrayString::<50>::new();
        let _ = writeln!(&mut text, "ADC RAW {}", adc_value);
        let _ = writeln!(&mut text, "Filter  {}", filtered);
        let _ = writeln!(&mut text, "Norm {}", bounded);

        display.clear();
        let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        let position = Point::new(0, 20);
        Text::new(&text, position, style).draw(&mut display).unwrap();

        display.flush().unwrap();
        led.set_low().unwrap();

        delay.delay_ms(50);
   }
}

static mut ADC_VALUE: AtomicU32 = AtomicU32::new(0);
static mut FILTERED: AtomicU32 = AtomicU32::new(0);
static mut NORMALIZED: AtomicU32 = AtomicU32::new(0);

fn adc_loop_on_core1(
    mut adc: Adc,
    mut adc_pin: Pin<Gpio26, Input<Floating>>
) -> ! {
    let mut input_filter = Filter::<100>::new(0);
    let mut filter1 = Filter::<100>::new(0);
    let mut filter2 = Filter::<100>::new(0);

    let mut adc_value = 0;

    loop {
        if let Some(new_value) = adc.read(&mut adc_pin).ok() {
            adc_value = new_value as u32;
            let _ = input_filter.add(new_value);
            let filtered = input_filter.get_average();
            let output1 = filter1.add(filtered as u16);
            let _output2 = filter2.add(output1);
        }

        let filtered1 = filter1.get_average();
        let filtered2 = filter2.get_average();
        let filtered = (filtered1 + filtered2)/2;

        const MIN: u32 = 30;
        const MAX: u32 = 3900;
        const NORM_MAX: u32 = 10000;

        let clamped = min(max(filtered, MIN), MAX);

        let normalized = (clamped - MIN)*NORM_MAX/(MAX - MIN);

        unsafe {
            ADC_VALUE.store(adc_value, Ordering::Relaxed);
            FILTERED.store(filtered, Ordering::Relaxed);
            NORMALIZED.store(normalized, Ordering::Relaxed);
        }
    }
}

struct Filter<const N: usize> {
    queue: VecDeque<u16, N>,
    sum: u32,
    default: u32
}

impl<const N: usize> Filter<N> {
    pub fn new(default: u32) -> Self {
        Self { queue: VecDeque::new(), sum: 0, default }
    }

    pub fn add(&mut self, value: u16) -> u16 {
        let mut output = value;

        if self.queue.len() == N {
            if let Some(front) = self.queue.pop_front() {
                self.sum -= front as u32;
                output = front;
            }
        }

        if let Ok(_) = self.queue.push_back(value) {
            self.sum += value as u32;
        }

        output
    }

    pub fn get_average(&self) -> u32 {
        if self.queue.len() == 0 {
            self.default
        } else {
            self.sum/self.queue.len() as u32
        }
    }
}
