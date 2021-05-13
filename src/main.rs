#![no_main]
#![no_std]

mod buttons;
mod distance;
mod hid;

#[cfg(not(feature = "rtt"))]
use panic_halt as _;

pub use rtic::{
    app,
    cyccnt::{Instant, U32Ext},
};

use stm32f4xx_hal::i2c::I2c;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::stm32::{EXTI, I2C1};

#[cfg(feature = "rtt")]
use core::panic::PanicInfo;
#[cfg(feature = "rtt")]
use rtt_target::{rprintln, rtt_init_print};

use crate::buttons::*;
use crate::distance::DistanceMeasurement;
use crate::hid::HidClass;
use arrayvec::ArrayVec;
use stm32f4xx_hal::gpio::gpioa::PA;
use stm32f4xx_hal::gpio::gpiob::{PB, PB6, PB7};
use stm32f4xx_hal::gpio::{AlternateOD, Edge, Input, OpenDrain, Output, PullUp, AF4};
use stm32f4xx_hal::otg_fs::{UsbBusType, USB};
use stm32f4xx_hal::{stm32, timer};
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;

pub type BusType = I2c<I2C1, (PB6<AlternateOD<AF4>>, PB7<AlternateOD<AF4>>)>;
type UsbTouchButtonPanelDevice = UsbDevice<'static, UsbBusType>;
type UsbTouchButtonPanelClass = HidClass<'static, UsbBusType>;
type ButtonsType = Buttons<PA<Input<PullUp>>>;

#[app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        buttons: ButtonsType,
        usb_device: UsbTouchButtonPanelDevice,
        usb_class: UsbTouchButtonPanelClass,
        exti: EXTI,
        distance: DistanceMeasurement,
        timer_tof: timer::Timer<stm32::TIM2>,
        timer_send: timer::Timer<stm32::TIM3>,
    }

    #[init]
    fn init(mut c: init::Context) -> init::LateResources {
        static mut EP_MEMORY: [u32; 1024] = [0; 1024];
        static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;

        #[cfg(feature = "rtt")]
        rtt_init_print!();

        //Enable Time Measurement
        c.core.DWT.enable_cycle_counter();
        c.core.DCB.enable_trace();

        #[cfg(feature = "rtt")]
        rprintln!("Start");

        let rcc = c.device.RCC.constrain();
        let gpioa = c.device.GPIOA.split();
        let gpiob = c.device.GPIOB.split();
        let _gpioc = c.device.GPIOC.split();
        let _gpiod = c.device.GPIOD.split();

        let clocks = rcc
            .cfgr
            .use_hse(25.mhz())
            .sysclk(84.mhz())
            .require_pll48clk()
            .freeze();

        //// USB initialization
        let usb = USB {
            usb_global: c.device.OTG_FS_GLOBAL,
            usb_device: c.device.OTG_FS_DEVICE,
            usb_pwrclk: c.device.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into_alternate_af10(),
            pin_dp: gpioa.pa12.into_alternate_af10(),
            hclk: clocks.hclk(),
        };

        *USB_BUS = Some(UsbBusType::new(usb, EP_MEMORY));
        let usb_bus = USB_BUS.as_ref().unwrap();

        let usb_class = HidClass::new(usb_bus);
        // https://github.com/obdev/v-usb/blob/master/usbdrv/USB-IDs-for-free.txt
        // For USB Joystick as there is no USB Game Pad on this free ID list
        let usb_device = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x16c0, 0x27dc))
            .manufacturer("autumnal.de")
            .product("Touch Button Panel")
            .serial_number(env!("CARGO_PKG_VERSION"))
            .build();

        //Initialize Interrupt Input
        let mut syscfg = c.device.SYSCFG.constrain();
        let mut exti = c.device.EXTI;

        //Distance Sensors I2C
        let scl = gpiob.pb6.into_alternate_af4().set_open_drain();
        let sda = gpiob.pb7.into_alternate_af4().set_open_drain();
        let i2c = I2c::new(c.device.I2C1, (scl, sda), 400.khz(), clocks);

        let mut shutdown = gpiob.pb10.into_open_drain_output();
        shutdown.set_high().unwrap();
        cortex_m::asm::delay(84_000_000); //1sec
        shutdown.set_low().unwrap();

        let mut reset: ArrayVec<PB<Output<OpenDrain>>, 8> = ArrayVec::new();
        reset.push(gpiob.pb0.into_open_drain_output().downgrade());
        reset.push(gpiob.pb1.into_open_drain_output().downgrade());
        reset.push(gpiob.pb12.into_open_drain_output().downgrade());
        reset.push(gpiob.pb3.into_open_drain_output().downgrade());
        reset.push(gpiob.pb4.into_open_drain_output().downgrade());
        reset.push(gpiob.pb5.into_open_drain_output().downgrade());
        reset.push(gpiob.pb9.into_open_drain_output().downgrade());
        reset.push(gpiob.pb8.into_open_drain_output().downgrade());
        let distance =
            DistanceMeasurement::new(&mut reset.into_inner().ok().unwrap(), i2c).unwrap();

        //loop {
        //    cortex_m::asm::delay(840_000);
        //    distance.update_read_all().unwrap();
        //    rprint!("Buttons: ");
        //    for (i, r) in distance.get_cached_buttons().iter().enumerate(){
        //        if *r{
        //            rprint!("{}, ", i);
        //        }
        //    }
        //    rprintln!("")
        //}

        let mut buttons: ArrayVec<PA<Input<PullUp>>, 8> = ArrayVec::new();
        let mut button = gpioa.pa8.into_pull_up_input();
        button.make_interrupt_source(&mut syscfg);
        button.enable_interrupt(&mut exti);
        button.trigger_on_edge(&mut exti, Edge::RISING_FALLING);
        buttons.push(button.downgrade());
        let mut button = gpioa.pa9.into_pull_up_input();
        button.make_interrupt_source(&mut syscfg);
        button.enable_interrupt(&mut exti);
        button.trigger_on_edge(&mut exti, Edge::RISING_FALLING);
        buttons.push(button.downgrade());
        let mut button = gpioa.pa15.into_pull_up_input();
        button.make_interrupt_source(&mut syscfg);
        button.enable_interrupt(&mut exti);
        button.trigger_on_edge(&mut exti, Edge::RISING_FALLING);
        buttons.push(button.downgrade());
        let mut button = gpioa.pa7.into_pull_up_input();
        button.make_interrupt_source(&mut syscfg);
        button.enable_interrupt(&mut exti);
        button.trigger_on_edge(&mut exti, Edge::RISING_FALLING);
        buttons.push(button.downgrade());
        let mut button = gpioa.pa10.into_pull_up_input();
        button.make_interrupt_source(&mut syscfg);
        button.enable_interrupt(&mut exti);
        button.trigger_on_edge(&mut exti, Edge::RISING_FALLING);
        buttons.push(button.downgrade());
        let mut button = gpioa.pa6.into_pull_up_input();
        button.make_interrupt_source(&mut syscfg);
        button.enable_interrupt(&mut exti);
        button.trigger_on_edge(&mut exti, Edge::RISING_FALLING);
        buttons.push(button.downgrade());
        let mut button = gpioa.pa4.into_pull_up_input();
        button.make_interrupt_source(&mut syscfg);
        button.enable_interrupt(&mut exti);
        button.trigger_on_edge(&mut exti, Edge::RISING_FALLING);
        buttons.push(button.downgrade());
        let mut button = gpioa.pa5.into_pull_up_input();
        button.make_interrupt_source(&mut syscfg);
        button.enable_interrupt(&mut exti);
        button.trigger_on_edge(&mut exti, Edge::RISING_FALLING);
        buttons.push(button.downgrade());
        let buttons = Buttons::new(buttons.into_inner().ok().unwrap());

        let mut timer_tof = timer::Timer::tim2(c.device.TIM2, 20.hz(), clocks);
        timer_tof.listen(timer::Event::TimeOut);
        let mut timer_send = timer::Timer::tim3(c.device.TIM3, 1.khz(), clocks);
        timer_send.listen(timer::Event::TimeOut);

        init::LateResources {
            buttons,
            usb_device,
            usb_class,
            exti,
            distance,
            timer_send,
            timer_tof,
        }
    }

    // Interrupt for Button0
    #[task(binds = EXTI4, resources = [buttons, exti], schedule=[debounce])]
    fn button4_interrupt(mut c: button4_interrupt::Context) {
        let buttons: &mut ButtonsType = &mut c.resources.buttons;
        let exti = &mut c.resources.exti;
        buttons.disable_interrupt(6, exti);
        buttons.clear_pending_interrupt(6);
        buttons.toggle_cached_button_status(6);

        c.schedule
            .debounce(Instant::now() + 840_000.cycles(), 6)
            .unwrap();
    }

    // Interrupt for Button0 - Button4
    #[task(binds = EXTI9_5, resources = [buttons, exti], schedule=[debounce])]
    fn button9_5_interrupt(mut c: button9_5_interrupt::Context) {
        let buttons: &mut ButtonsType = &mut c.resources.buttons;
        let exti = &mut c.resources.exti;
        let pr = exti.pr.read();
        let mut due: u8 = 0;

        if pr.pr5().bit_is_set() {
            bit_set(&mut due, 7);
        }

        if pr.pr6().bit_is_set() {
            bit_set(&mut due, 5);
        }

        if pr.pr7().bit_is_set() {
            bit_set(&mut due, 3);
        }

        if pr.pr8().bit_is_set() {
            bit_set(&mut due, 0);
        }

        if pr.pr9().bit_is_set() {
            bit_set(&mut due, 1);
        }

        for i in 0..=7 {
            if bit_check(due, i as u8) {
                buttons.disable_interrupt(i, exti);
                buttons.clear_pending_interrupt(i);
                buttons.toggle_cached_button_status(i);

                c.schedule
                    .debounce(Instant::now() + 840_000.cycles(), i)
                    .unwrap();
            }
        }
    }

    // Interrupt for Button5 - Button7
    #[task(binds = EXTI15_10, resources = [buttons, exti], schedule=[debounce])]
    fn button15_10_interrupt(mut c: button15_10_interrupt::Context) {
        let buttons: &mut ButtonsType = &mut c.resources.buttons;
        let exti = &mut c.resources.exti;
        let pr = exti.pr.read();
        let mut due: u8 = 0;

        if pr.pr10().bit_is_set() {
            bit_set(&mut due, 4);
        }

        if pr.pr15().bit_is_set() {
            bit_set(&mut due, 2);
        }

        for i in 0..=7 {
            if bit_check(due, i as u8) {
                buttons.disable_interrupt(i, exti);
                buttons.clear_pending_interrupt(i);
                buttons.toggle_cached_button_status(i);

                c.schedule
                    .debounce(Instant::now() + 840_000.cycles(), i)
                    .unwrap();
            }
        }
    }

    // Debouncer; Reactivates Interrupt and reads current status
    #[task(resources = [buttons, exti], capacity = 8)]
    fn debounce(mut c: debounce::Context, btn: usize) {
        let buttons: &mut ButtonsType = &mut c.resources.buttons;

        buttons.enable_interrupt(btn, c.resources.exti);
        buttons.update_button_status(btn);
    }

    // Periodic status update to Computer (every millisecond)
    #[task(binds = TIM2, resources = [distance, timer_tof])]
    fn tof(c: tof::Context) {
        c.resources
            .timer_tof
            .clear_interrupt(stm32f4xx_hal::timer::Event::TimeOut);
        let distance: &mut DistanceMeasurement = c.resources.distance;
        distance.update_read_all().unwrap();
    }

    // Periodic status update to Computer (every millisecond)
    #[task(binds = TIM3, resources = [usb_class, distance, timer_send, buttons])]
    fn report(mut c: report::Context) {
        c.resources
            .timer_send
            .clear_interrupt(stm32f4xx_hal::timer::Event::TimeOut);
        let buttons: &mut ButtonsType = c.resources.buttons;
        let distance: &mut DistanceMeasurement = c.resources.distance;

        let report = crate::hid::make_report(
            buttons
                .get_full_cached_status()
                .iter()
                .chain(distance.get_cached_buttons().iter()),
        );
        //Lock usb_class object and report
        c.resources.usb_class.lock(|class| class.write(&report));
    }

    // Global USB Interrupt (does not include Wakeup)
    #[task(binds = OTG_FS, resources = [usb_device, usb_class], priority = 2)]
    fn usb_tx(mut c: usb_tx::Context) {
        usb_poll(&mut c.resources.usb_device, &mut c.resources.usb_class);
    }

    // Interrupt for USB Wakeup
    #[task(binds = OTG_FS_WKUP, resources = [usb_device, usb_class], priority = 2)]
    fn usb_rx(mut c: usb_rx::Context) {
        usb_poll(&mut c.resources.usb_device, &mut c.resources.usb_class);
    }

    extern "C" {
        //Any free Interrupt which is used for the Debounce Software Task
        fn SDIO();
    }
};

fn usb_poll(
    usb_device: &mut UsbTouchButtonPanelDevice,
    touch_panel: &mut UsbTouchButtonPanelClass,
) {
    usb_device.poll(&mut [touch_panel]);
}

#[cfg(feature = "rtt")]
#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {} // You might need a compiler fence in here.
}
