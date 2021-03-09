//! main.rs

#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, dispatchers = [EXTI0])]
mod app {
    use cortex_m::asm;
    use dwt_systick_monotonic::{
        consts::{U0, U8},
        DwtSystick,
    };
    use rtic::time::duration::Seconds;
    use stm32f4xx_hal::{gpio::*, prelude::*};

    const PERIOD: u32 = 8_000_000;

    #[resources]
    struct Resources {
        led: gpioa::PA5<Output<PushPull>>,
    }

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<U8, U0, U0>; // 8 MHz

    #[init()]
    fn init(mut cx: init::Context) -> (init::LateResources, init::Monotonics) {
        // Initialize LED output
        let gpioa = cx.device.GPIOA.split();
        let led = gpioa.pa5.into_push_pull_output();

        // Initialize cyccnt
        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();

        // Initliaze monotonic
        let mono = DwtSystick::new(&mut cx.core.DCB, cx.core.DWT, cx.core.SYST, PERIOD);

        // Schedule led to turn on
        led_on::spawn().ok();

        (init::LateResources { led }, init::Monotonics(mono))
    }

    #[task(resources = [led])]
    fn led_on(mut cx: led_on::Context) {
        cx.resources.led.lock(|led| led.set_high().unwrap());
        led_off::spawn_after(Seconds(1u32)).unwrap();
    }

    #[task(resources = [led])]
    fn led_off(mut cx: led_off::Context) {
        cx.resources.led.lock(|led| led.set_low().unwrap());
        led_on::spawn_after(Seconds(1u32)).unwrap();
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            asm::nop();
        }
    }
}
