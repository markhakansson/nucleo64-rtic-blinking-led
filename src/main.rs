//! main.rs

#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::stm32, monotonic = rtic::cyccnt::CYCCNT, peripherals = true, dispatchers = [EXTI0,EXTI1])]
mod app {
    use cortex_m::asm;
    use rtic::cyccnt::{Instant, U32Ext};
    use stm32f4xx_hal::{gpio::*, prelude::*};

    const PERIOD: u32 = 8_000_000;

    #[resources]
    struct Resources {
        led: gpioa::PA5<Output<PushPull>>,
    }

    #[init()]
    fn init(mut cx: init::Context) -> init::LateResources {
        // Initialize LED output
        let gpioa = cx.device.GPIOA.split();
        let led = gpioa.pa5.into_push_pull_output();

        // Initialize cyccnt
        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();

        // Schedule led to turn on
        //led_on::schedule(cx.start + PERIOD.cycles()).unwrap();
        led_on::spawn().ok();

        init::LateResources { led }
    }

    #[task(resources = [led])]
    fn led_on(mut cx: led_on::Context) {
        cx.resources.led.lock(|led| led.set_high().unwrap());
        led_off::schedule(cx.scheduled + PERIOD.cycles()).unwrap();
    }

    #[task(resources = [led])]
    fn led_off(mut cx: led_off::Context) {
        cx.resources.led.lock(|led| led.set_low().unwrap());
        led_on::schedule(cx.scheduled + PERIOD.cycles()).unwrap();
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            asm::nop();
        }
    }
}
