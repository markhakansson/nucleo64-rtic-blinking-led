//! examples/addition.rs

#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::stm32, monotonic = rtic::cyccnt::CYCCNT, peripherals = true, dispatchers = [EXTI0])]
mod app {
    use cortex_m::asm;
    use rtic::cyccnt::U32Ext;
    use stm32f4xx_hal;

    #[resources]
    struct Resources {
        #[init(0)]
        sum: u32,
    }

    #[init()]
    fn init(mut cx: init::Context) -> init::LateResources {
        asm::bkpt();
        cx.core.DWT.enable_cycle_counter();
        // Reset CYCCNT just in case
        unsafe {
            cx.core.DWT.cyccnt.modify(|_| 0);
        }
        t1::schedule(cx.start + 100_000.cycles()).unwrap();
        t2::schedule(cx.start + 200_000.cycles()).unwrap();

        init::LateResources {}
    }

    #[inline(never)]
    #[task(resources = [sum])]
    fn t1(mut cx: t1::Context) {
        asm::bkpt();
        cx.resources.sum.lock(|sum| *sum += 1);
        asm::bkpt();
    }

    #[inline(never)]
    #[task(resources = [sum])]
    fn t2(mut cx: t2::Context) {
        asm::bkpt();
        cx.resources.sum.lock(|sum| *sum += 1000);
        asm::bkpt();
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        asm::bkpt();
        loop {
            asm::nop();
        }
    }
}
