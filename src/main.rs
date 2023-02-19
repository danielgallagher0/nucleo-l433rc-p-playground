#![no_std]
#![no_main]

extern crate stm32l4xx_hal as hal;

use crate::hal::prelude::*;
use crate::hal::delay::Delay;

use panic_probe as _;
// use panic_halt as _;
use defmt_rtt as _;

// use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);
    let clocks = rcc.cfgr
        .sysclk(64.MHz())
        .pclk1(32.MHz())
        .freeze(&mut flash.acr, &mut pwr);

    // PB13 - LED
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    // PC13 - User button
    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb2);

    let mut led = gpiob
        .pb13
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let button = gpioc.pc13.into_pull_down_input(&mut gpioc.moder, &mut gpioc.pupdr);

    let mut timer = Delay::new(cp.SYST, clocks);
    let mut delay = 1000_u32;
    loop {
        defmt::info!("Blink {}!", delay);

        timer.delay_ms(delay);
        led.set_high();
        timer.delay_ms(delay);
        led.set_low();

        if button.is_low() {
            delay = 500_u32;
        } else {
            delay = 1000_u32;
        }
    }
}
