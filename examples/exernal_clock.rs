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

use rtt_target::{rtt_init_print, rprintln};


#[entry]
fn main() -> ! {
    rtt_init_print!();
    
    let pac = pac::Peripherals::take().unwrap();
    // let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    let rcc = pac.RCC;
    // let mut flash = pac.FLASH.constrain();
    
    // setting up clock source
    rcc.cr.write(|w| unsafe {
        w.bits(0x0001_0000)  // for HSE
        // w.bits(0x0000_0001)  // for HSI
    });

    // rcc.cr.write(|w| w.hsion().clear_bit());
    // rcc.cr.write(|w| w.hseon().set_bit());


    
    while rcc.cr.read().hserdy().bit_is_clear() {} // wait for HSE
    // while rcc.cr.read().hsirdy().bit_is_clear() {} // wait for HSI
    
    // configure switch 
    rcc.cfgr.write(|w| unsafe {
        w.bits(0x00000001)  // 0x00000001 for using HSE
        // w.bits(0x00000000)  // 0x00000000 for using HSI
    });
    

    // rcc.apb2enr.write(|w| unsafe {
    //     w.bits(0b0000000000000000000000000000000)
    // });
    // rcc.apb2enr.write(|w| w.afioen().set_bit());

    
    // let rcc = pac.RCC.constrain();
    // rcc.cfgr.use_hse(8);
    // dp.RCC.cr.write(|w| w.hseon().set_bit());
    // while dp.RCC.cr.read().hserdy().bit() {}

    // rcc.cfgr.write(|w| unsafe {
    //     w.ppre1().bits(0b0)
    // });
    //
    // let mut gpioc = pac.GPIOC.split();
    // let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    // 
    // let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // let mut delay = cortex_peripherals.SYST.delay(&adc_clocks);
    // 
    // rcc.apb2enr.write(|w| unsafe {
    //     w.bits(0b0000000000000000000000000000000)
    // });
    
    // rcc.apb2enr.write(|w| unsafe {
    //     w.bits(0x00000000)
    // });

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
        rprintln!("hseon: {}, hsion: {}, rcc.cfgr.sws.is_hsi: {}, rcc.cfgr.sws.is_hse: {}", 
            rcc.cr.read().hseon().bit_is_set(), 
            rcc.cr.read().hsion().bit_is_set(), 
            rcc.cfgr.read().sws().is_hsi(),
            rcc.cfgr.read().sws().is_hse()
        );

        
        // led.toggle();
    }
}
