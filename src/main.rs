#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use stm32f4xx_hal::{
    pac::{self},
    prelude::*,
};

#[entry]
fn main() -> ! {
    // Get access to the peripherals
    let dp = pac::Peripherals::take().unwrap();
    // Get access to the core peripherals
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock. We want to run at 48MHz for this one.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

    // Create a delay abstraction based on SysTick
    let mut delay = cp.SYST.delay(&clocks);

    let gpioa = dp.GPIOA.split();
    let gpioc = dp.GPIOC.split();

    let mut led = gpioa.pa5.into_push_pull_output();
    let button = gpioc.pc13;

    hprintln!("Starting up Blink with Button!");

    loop {
        if button.is_low() {
            // Si el botón está presionado, encender el LED
            led.set_high();
        } else {
            // Si el botón no está presionado, apagar el LED
            led.set_low();
        }
        delay.delay_ms(1);
    }
}
