//! examples/blinky-bkpt.rs

#![no_main]
#![no_std]
#![feature(asm)]

use cortex_m::asm;
use panic_halt as _;
use rtic::cyccnt::{Instant, U32Ext};
use stm32f4xx_hal::{gpio::*, prelude::*};

const PERIOD: u32 = 4_000_000;

#[rtic::app(device = stm32f4xx_hal::stm32, monotonic = rtic::cyccnt::CYCCNT, peripherals = true)]
const APP: () = {
    struct Resources {
        led: gpioa::PA5<Output<PushPull>>,
    }

    #[no_mangle]
    #[inline(never)]
    #[init(schedule = [led_on])]
    fn init(mut cx: init::Context) -> init::LateResources {
        // Initialize LED output
        let gpioa = cx.device.GPIOA.split();
        let led = gpioa.pa5.into_push_pull_output();

        // Initialize cyccnt
        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();

        // Schedule led to turn on
        cx.schedule.led_on(cx.start + PERIOD.cycles()).unwrap();

        init::LateResources { led }
    }

    #[no_mangle]
    #[inline(never)]
    #[task(schedule = [led_off], resources = [led])]
    fn led_on(cx: led_on::Context) {
        unsafe {
            asm!("bkpt 255");
        }
        cx.schedule.led_off(cx.scheduled + PERIOD.cycles()).unwrap();
        cx.resources.led.set_high().unwrap();
    }

    #[no_mangle]
    #[inline(never)]
    #[task(schedule = [led_on], resources = [led])]
    fn led_off(cx: led_off::Context) {
        unsafe {
            asm!("bkpt 1");
        }
        cx.schedule.led_on(cx.scheduled + PERIOD.cycles()).unwrap();
        cx.resources.led.set_low().unwrap();
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            asm::nop();
        }
    }

    extern "C" {
        fn EXTI0();
        fn EXTI1();
    }
};
