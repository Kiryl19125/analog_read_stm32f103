//! CRC calculation

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]


use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln, rprint};
use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let p = pac::Peripherals::take().unwrap();

    let mut crc = p.CRC.new();

    crc.reset();
    crc.write(0x12345678);

    let val = crc.read();
    rprintln!("found={:08x}, expected={:08x}", val, 0xdf8a8a2b_u32);

    loop {}
}
