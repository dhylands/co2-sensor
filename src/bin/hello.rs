#![no_main]
#![no_std]

use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Level},
    Timer,
};

use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

use co2_sensor as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    let board = hal::pac::Peripherals::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let pins = P0Parts::new(board.P0);

    let mut led_1 = pins.p0_13.into_push_pull_output(Level::Low);

    defmt::info!("Hello, world!");

    loop {
        led_1.set_high().unwrap();
        timer.delay_ms(1000u32);
        led_1.set_low().unwrap();
        timer.delay_ms(1000u32);
    }

    // co2_sensor::exit()
}
