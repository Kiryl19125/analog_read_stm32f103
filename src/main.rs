#![no_std]
#![no_main]

// #![allow(unused_imports)]
use cortex_m_rt::entry;
use panic_rtt_target as _;
use stm32f1xx_hal::adc;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::serial::{Config, Serial};
// use nb::block;
use core::fmt::Write;

use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    
    rtt_init_print!();
    rprintln!("Hello world");

    let pac_peripherals = pac::Peripherals::take().unwrap();
    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut flash = pac_peripherals.FLASH.constrain();
    let rcc = pac_peripherals.RCC.constrain();
    
    // let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let adc_clocks = rcc.cfgr.adcclk(8.MHz()).freeze(&mut flash.acr);
    rprintln!("ADC Freq = {}", adc_clocks.adcclk());


    let mut adc1 = adc::Adc::adc1(pac_peripherals.ADC1, adc_clocks);
    let mut afio = pac_peripherals.AFIO.constrain();
    let mut gpiob = pac_peripherals.GPIOB.split();
    let mut gpioa = pac_peripherals.GPIOA.split();
    let mut gpioc = pac_peripherals.GPIOC.split();
    let mut ch0 = gpiob.pb0.into_analog(&mut gpiob.crl);
    let mut delay = cortex_peripherals.SYST.delay(&adc_clocks);

        // USART1
    let rx = gpioa.pa10;
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let mut serial = Serial::new(
        pac_peripherals.USART1,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(115200.bps()),
        &adc_clocks
    );

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    
    loop {
        let data: u16 = adc1.read(&mut ch0).unwrap();
        let voltage = convert_to_float(data);
        let integer_part = voltage as u16;
        let decimal_part = ((voltage - integer_part as f32) * 100.0) as u32;
        // rprintln!("Analog data = {}, voltage = {}.{:02}v", data, integer_part, decimal_part);

        // writeln!(serial, "Hello\r\n").unwrap();
        write!(serial, "Analog data = {}, voltage = {}.{:02}v\r\n", data, integer_part, decimal_part).unwrap();
        led.toggle();
        delay.delay_ms(100_u16);
    }
}


fn convert_to_float(input: u16) -> f32 {
    // Make sure the input is within the valid range
    let clamped_input = input.min(4096) as f32;

    // Define the range of the input and output
    let input_range = 4096.0;
    let output_range = 3.3;

    // Perform the linear mapping
    let output = clamped_input * (output_range / input_range);

    output
}
