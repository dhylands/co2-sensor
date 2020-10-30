#![no_std]
#![no_main]

use co2_sensor as _;
use cortex_m_rt::entry; // memory layout + panic handler

#[entry]
fn main() -> ! {
    assert!(false, "TODO: Write actual tests");

    co2_sensor::exit();
}
