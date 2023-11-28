#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_rtt_target as _;
// use stm32f1xx_hal::adc;
// use stm32f1xx_hal::pac;

// use stm32f1xx_hal::prelude::*;

use stm32f1xx_hal::{prelude::*, pac};

// use stm32f1xx_hal::serial::{Config, Serial};
// use nb::block;
// use core::fmt::Write;

use rtt_target::{rtt_init_print, rprintln, rprint};


#[entry]
fn main() -> ! {
    rtt_init_print!();
    
    let pac = pac::Peripherals::take().unwrap();

    let rcc = pac.RCC;
     
    // turn on hsi
    rcc.cr.write(|w| unsafe {
        w.bits(0x00000001)
    });

    // until flag will be set
    while rcc.cr.read().hsirdy().bit_is_clear() {
        
    }

    //reset CR
    rcc.cr.write(|w| unsafe {
        w.bits(0x00000083)
    });

    // turn on HSE
    

    let mut counter = 0;
    loop {
       // if rcc.cr.read().hseon().bit() {
       //      rprint!("hsEon: true |");
       //  } else {
       //      rprint!("hsEon: false |");
       //  } 
       // if rcc.cr.read().hsion().bit() {
       //      rprintln!("hsIon: true ");
       //  } else {
       //      rprintln!("hsIon: false ");
       //  } 
        // rprint!("{} ", rcc.cr.read().hseon().bit());
        
        // for _ in 0..5000_000_000_u64 {
        //     
        // }
        //
        //
        while counter < 1000000 {
            counter += 1;
        }
        counter = 0;
        rprintln!("hseon: {}, hsion: {}", rcc.cr.read().hseon().bit_is_set(), rcc.cr.read().hsion().bit_is_set());
        // led.toggle();
    }
}
