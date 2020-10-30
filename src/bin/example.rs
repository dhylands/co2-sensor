#![no_main]
#![no_std]

use co2_sensor as _; // global logger + panicking-behavior + memory layout

use hal::gpio::Pin;
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Level, Output, PushPull},
    Temp, Timer,
};

use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

/// A trait for controlling a signal (i.e. a gpio output like a relay or LED).
///
/// A Signal has the notion of being on or off which is independent of how that
/// signal is manifested in the hardware.
///
/// Some signals will be on when the output is a logic high and some signals
/// will be on when the output is a logic low.
pub trait OutputSignal {
    /// Turns the signal on.
    fn on(&mut self);

    /// Turns the signal off.
    fn off(&mut self);
}

/// Implements a Signal which is on when a GPIO pin is at a logic low.
pub struct GpioOutputSignalActiveLow {
    pin: Pin<Output<PushPull>>,
}

impl GpioOutputSignalActiveLow {
    pub fn new<Mode>(pin: Pin<Mode>) -> Self {
        Self {
            pin: pin.into_push_pull_output(Level::High),
        }
    }
}

impl OutputSignal for GpioOutputSignalActiveLow {
    /// Turns a Signal on by setting a GPIO pin low.
    fn on(&mut self) {
        self.pin.set_low().unwrap();
    }

    /// Turns a Signal off by setting a GPIO pin high.
    fn off(&mut self) {
        self.pin.set_high().unwrap();
    }
}

/// Implements a Signal which is on when a GPIO pin is at a logic high.
pub struct GpioOutputSignalActiveHigh {
    pin: Pin<Output<PushPull>>,
}

impl GpioOutputSignalActiveHigh {
    pub fn new<Mode>(pin: Pin<Mode>) -> Self {
        Self {
            pin: pin.into_push_pull_output(Level::Low),
        }
    }
}

impl OutputSignal for GpioOutputSignalActiveHigh {
    /// Turns a Signal on by setting a GPIO pin low.
    fn on(&mut self) {
        self.pin.set_high().unwrap();
    }

    /// Turns a Signal off by setting a GPIO pin high.
    fn off(&mut self) {
        self.pin.set_low().unwrap();
    }
}

pub struct RGBLed<'a> {
    red: &'a mut dyn OutputSignal,
    green: &'a mut dyn OutputSignal,
    blue: &'a mut dyn OutputSignal,
}

impl<'a> RGBLed<'a> {
    pub fn new(
        red: &'a mut dyn OutputSignal,
        green: &'a mut dyn OutputSignal,
        blue: &'a mut dyn OutputSignal,
    ) -> Self {
        Self { red, green, blue }
    }

    pub fn off(&mut self) {
        self.red.off();
        self.green.off();
        self.blue.off();
    }

    pub fn red(&mut self) {
        self.red.on();
        self.green.off();
        self.blue.off();
    }

    pub fn green(&mut self) {
        self.red.off();
        self.green.on();
        self.blue.off();
    }

    pub fn blue(&mut self) {
        self.red.off();
        self.green.off();
        self.blue.on();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let board = hal::pac::Peripherals::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let pins = P0Parts::new(board.P0);

    let mut led_1 = GpioOutputSignalActiveLow::new(pins.p0_13.degrade());

    let mut red_pin = GpioOutputSignalActiveLow::new(pins.p0_03.degrade());
    let mut green_pin = GpioOutputSignalActiveLow::new(pins.p0_04.degrade());
    let mut blue_pin = GpioOutputSignalActiveLow::new(pins.p0_28.degrade());

    let mut rgb_led = RGBLed::new(&mut red_pin, &mut green_pin, &mut blue_pin);

    let mut temp = Temp::new(board.TEMP);

    defmt::info!("Hello, world!");

    timer.delay_ms(1000u32);
    loop {
        led_1.on();
        timer.delay_ms(100u32);
        led_1.off();
        timer.delay_ms(100u32);
        led_1.on();
        timer.delay_ms(100u32);
        led_1.off();
        timer.delay_ms(700u32);

        rgb_led.red();
        timer.delay_ms(100u32);
        rgb_led.off();
        timer.delay_ms(100u32);

        rgb_led.green();
        timer.delay_ms(100u32);
        rgb_led.off();
        timer.delay_ms(100u32);

        rgb_led.blue();
        timer.delay_ms(100u32);
        rgb_led.off();
        timer.delay_ms(500u32);

        let temperature: f32 = temp.measure().to_num();
        defmt::info!("{:f32} °C", temperature);
    }

    //co2_sensor::exit()
}
