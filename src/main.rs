#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[cfg(not(feature = "rtt"))]
use panic_halt as _;

pub use rtic::{
    app,
    cyccnt::{Instant, U32Ext},
};

use stm32f4xx_hal::i2c::*;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::stm32::EXTI;

#[cfg(feature = "rtt")]
use core::panic::PanicInfo;
#[cfg(feature = "rtt")]
use rtt_target::{rprintln, rtt_init_print};

#[app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        exti: EXTI,
    }

    #[init]
    fn init(mut c: init::Context) -> init::LateResources {
        #[cfg(feature = "rtt")]
        rtt_init_print!();

        //Enable Time Measurement
        c.core.DWT.enable_cycle_counter();
        c.core.DCB.enable_trace();
        rprintln!("Start");

        let rcc = c.device.RCC.constrain();
        let _gpioa = c.device.GPIOA.split();
        let gpiob = c.device.GPIOB.split();
        let _gpioc = c.device.GPIOC.split();
        let _gpiod = c.device.GPIOD.split();

        let clocks = rcc
            .cfgr
            .use_hse(25.mhz())
            .sysclk(84.mhz())
            .require_pll48clk()
            .freeze();

        //Initialize Interrupt Input
        let _syscfg = c.device.SYSCFG;
        let _exti = c.device.EXTI;

        rprintln!("I2C Start");
        let scl = gpiob.pb8.into_alternate_af4().set_open_drain();
        let sda = gpiob.pb9.into_alternate_af4().set_open_drain();
        let i2c = I2c::i2c1(c.device.I2C1, (scl, sda), 400.khz(), clocks);

        let mut tof = vl53l0x::VL53L0x::with_address(i2c, 0x29).expect("create");
        rprintln!("I2C 2");

        tof.set_measurement_timing_budget(200000).expect("timbudg");
        tof.start_continuous(0).expect("start cont");

        loop {
            cortex_m::asm::delay(480_000);
            match tof.read_range_continuous_millimeters_blocking(){
                Ok(res) => {
                    rprintln!("Distance: {}mm", res)
                }
                Err(_) => {
                    rprintln!("Read Error")
                }
            }
        }

        //init::LateResources {
        //    exti,
        //}
    }

    // Interrupt for Button0
    #[task(binds = EXTI3, resources = [exti])]
    fn button3_interrupt(_c: button3_interrupt::Context) {

    }

    extern "C" {
        //Any free Interrupt which is used for the Debounce Software Task
        fn SDIO();
    }
};


#[cfg(feature = "rtt")]
#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {} // You might need a compiler fence in here.
}
