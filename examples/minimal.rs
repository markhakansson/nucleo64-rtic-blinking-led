#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true)]
mod app {
    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let start: u32 = cx.core.DWT.cyccnt.read();
        init::LateResources {}
    }
}
