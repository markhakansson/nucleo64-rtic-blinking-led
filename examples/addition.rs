//! examples/addition.rs

#![no_main]
#![no_std]

use panic_halt as _;
use rtic::cyccnt::U32Ext;
use stm32f4xx_hal;
use cortex_m::asm;

#[rtic::app(device = stm32f4xx_hal::stm32, monotonic = rtic::cyccnt::CYCCNT, peripherals = true)]
const APP: () = {
    struct Resources {
        #[init(0)]
        sum: u32,
    }

    #[init(schedule = [t1, t2])]
    fn init(mut cx: init::Context) {
        asm::bkpt();
        cx.core.DWT.enable_cycle_counter();
        // Reset CYCCNT just in case
        unsafe {
            cx.core.DWT.cyccnt.modify(|_| 0);
        }
        cx.schedule.t1(cx.start + 100_000.cycles()).unwrap();
        cx.schedule.t2(cx.start + 200_000.cycles()).unwrap();
    }

    #[inline(never)]
    #[task(resources = [sum])]
    fn t1(cx: t1::Context) {
        asm::bkpt();
        *cx.resources.sum += 1;
        asm::bkpt();
    }

    #[inline(never)]
    #[task(resources = [sum])]
    fn t2(cx: t2::Context) {
        asm::bkpt();
        *cx.resources.sum += 1000;
        asm::bkpt();
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        asm::bkpt();
        loop {
            asm::nop();
        }
    }

    extern "C" {
        fn EXTI0();
    }
};
