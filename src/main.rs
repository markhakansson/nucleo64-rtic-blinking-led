//! main.rs

#![deny(unsafe_code)]
#![no_main]
#![no_std]
 
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{
    prelude::*,
    gpio::*,
};
use rtic::cyccnt::{Instant, U32Ext};

const PERIOD: u32 = 8_000_000;

#[rtic::app(device = stm32f4xx_hal::stm32, monotonic = rtic::cyccnt::CYCCNT, peripherals = true)]
const APP: () = {
    struct Resources {
        led: gpioa::PA5<Output<PushPull>>
    }

    #[init(schedule = [led_on])]
    fn init(mut cx: init::Context) -> init::LateResources {
        rtt_init_print!();
        rprintln!("init");
        // Initialize LED output
        let gpioa = cx.device.GPIOA.split();
        let led = gpioa.pa5.into_push_pull_output();

        // Initialize cyccnt
        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();
        
        // Schedule led to turn on
        cx.schedule.led_on(cx.start + PERIOD.cycles()).unwrap();

        init::LateResources{
            led
        }
    }
    
    #[task(schedule = [led_off], resources = [led])]
    fn led_on(cx: led_on::Context) {
        rprintln!("led_on");
        cx.schedule.led_off(cx.scheduled + PERIOD.cycles()).unwrap();
        cx.resources.led.set_high().unwrap();
    }

    #[task(schedule = [led_on], resources = [led])]
    fn led_off(cx: led_off::Context) {
        rprintln!("led_off");
        cx.schedule.led_on(cx.scheduled + PERIOD.cycles()).unwrap();
        cx.resources.led.set_low().unwrap();
    }

    extern "C" {
        fn EXTI0();
        fn EXTI1();
    }
};
