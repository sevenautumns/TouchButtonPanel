#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod hid;
mod touch_button_panel;

use panic_semihosting as _;

pub use rtic::{
    app,
    cyccnt::{Instant, U32Ext},
};

#[macro_use(block)]
use nb::block;

use at42qt1070::Key::*;
use at42qt1070::*;
use stm32f4xx_hal::gpio::{
    gpioa::*, gpiob::*, gpioc::*, AlternateOD, Edge, ExtiPin, Input, OpenDrain, Output, PullUp,
    PushPull, AF4, AF9,
};
use stm32f4xx_hal::i2c::*;
use stm32f4xx_hal::interrupt::*;
use stm32f4xx_hal::otg_fs::{UsbBusType, USB};
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::stm32;
use stm32f4xx_hal::stm32::{I2C1, I2C2, I2C3};
use stm32f4xx_hal::timer;
use usb_device::bus::UsbBusAllocator;
use usb_device::class::UsbClass as _;

type UsbTouchButtonPanelDevice = touch_button_panel::Device<'static, UsbBusType>;
type UsbTouchButtonPanelClass = touch_button_panel::Class<'static, UsbBusType>;

type TouchSensor1 = TouchSensor<
    I2C1,
    PB6<AlternateOD<AF4>>,
    PB7<AlternateOD<AF4>>,
    PC15<Output<OpenDrain>>,
    PA2<Input<PullUp>>,
>;
type TouchSensor2 = TouchSensor<
    I2C2,
    PB10<AlternateOD<AF4>>,
    PB3<AlternateOD<AF9>>,
    PC14<Output<OpenDrain>>,
    PA1<Input<PullUp>>,
>;
type TouchSensor3 = TouchSensor<
    I2C3,
    PA8<AlternateOD<AF4>>,
    PB4<AlternateOD<AF9>>,
    PC13<Output<OpenDrain>>,
    PA0<Input<PullUp>>,
>;

pub struct TouchSensor<I2C, CLK, SDA, RESET, CHANGE> {
    pub sensor: At42qt1070<I2c<I2C, (CLK, SDA)>>,
    pub reset: RESET,
    pub change_interrupt: CHANGE,
}

#[app(device = stm32f4xx_hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        led_1: PA5<Output<PushPull>>,
        led_2: PA6<Output<PushPull>>,
        led_3: PA7<Output<PushPull>>,
        timer: timer::Timer<stm32::TIM3>,
        usb_device: UsbTouchButtonPanelDevice,
        usb_class: UsbTouchButtonPanelClass,
        sensor_one: TouchSensor1,
        sensor_two: TouchSensor2,
        sensor_three: TouchSensor3,
    }

    #[init]
    fn init(mut c: init::Context) -> init::LateResources {
        static mut EP_MEMORY: [u32; 1024] = [0; 1024];
        static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;

        //Time Measurement
        c.core.DWT.enable_cycle_counter();
        c.core.DCB.enable_trace();

        let rcc = c.device.RCC.constrain();
        let gpioa = c.device.GPIOA.split();
        let gpiob = c.device.GPIOB.split();
        let gpioc = c.device.GPIOC.split();

        let mut led_1 = gpioa.pa5.into_push_pull_output();
        let mut led_2 = gpioa.pa6.into_push_pull_output();
        let mut led_3 = gpioa.pa7.into_push_pull_output();

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
        };

        *USB_BUS = Some(UsbBusType::new(usb, EP_MEMORY));
        let usb_bus = USB_BUS.as_ref().unwrap();

        let usb_class = touch_button_panel::new_class(usb_bus);
        let usb_device = touch_button_panel::new_device(usb_bus);

        //Initialize Interrupt Input
        let mut syscfg = c.device.SYSCFG;
        let mut exti = c.device.EXTI;

        //// Init IC1
        // Connect Change Line of IC1 to PA2 and activate interrupt on falling edge
        let mut change_interrupt = gpioa.pa2.into_pull_up_input();
        change_interrupt.make_interrupt_source(&mut syscfg);
        change_interrupt.enable_interrupt(&mut exti);
        change_interrupt.trigger_on_edge(&mut exti, Edge::FALLING);
        // Use PC15 for resetting IC1
        let reset = gpioc.pc15.into_open_drain_output();
        // i2c scl is PB6 and sda is PB7
        let scl = gpiob.pb6.into_alternate_af4().set_open_drain();
        let sda = gpiob.pb7.into_alternate_af4().set_open_drain();
        let i2c = I2c::i2c1(c.device.I2C1, (scl, sda), 400.khz(), clocks);
        let sensor = At42qt1070::new(i2c);
        // Combine into TouchSensor1
        let mut sensor_one = TouchSensor1 {
            sensor,
            reset,
            change_interrupt,
        };

        //// Init IC2
        // Connect Change Line of IC2 to PA1 and activate interrupt on falling edge
        let mut change_interrupt = gpioa.pa1.into_pull_up_input();
        change_interrupt.make_interrupt_source(&mut syscfg);
        change_interrupt.enable_interrupt(&mut exti);
        change_interrupt.trigger_on_edge(&mut exti, Edge::FALLING);
        // Use PC14 for resetting IC2
        let reset = gpioc.pc14.into_open_drain_output();
        // i2c scl is PB10 and sda is PB3
        let scl = gpiob.pb10.into_alternate_af4().set_open_drain();
        let sda = gpiob.pb3.into_alternate_af9().set_open_drain();
        let i2c = I2c::i2c2(c.device.I2C2, (scl, sda), 400.khz(), clocks);
        let sensor = At42qt1070::new(i2c);
        // Combine into TouchSensor2
        let mut sensor_two = TouchSensor2 {
            sensor,
            reset,
            change_interrupt,
        };

        //// Init IC3
        // Connect Change Line of IC3 to PA0 and activate interrupt on falling edge
        let mut change_interrupt = gpioa.pa0.into_pull_up_input();
        change_interrupt.make_interrupt_source(&mut syscfg);
        change_interrupt.enable_interrupt(&mut exti);
        change_interrupt.trigger_on_edge(&mut exti, Edge::FALLING);
        // Use PC13 for resetting IC3
        let reset = gpioc.pc13.into_open_drain_output();
        // i2c scl is PA8 and sda is PB4
        let scl = gpioa.pa8.into_alternate_af4().set_open_drain();
        let sda = gpiob.pb4.into_alternate_af9().set_open_drain();
        let i2c = I2c::i2c3(c.device.I2C3, (scl, sda), 400.khz(), clocks);
        let sensor = At42qt1070::new(i2c);
        // Combine into TouchSensor3
        let mut sensor_three = TouchSensor3 {
            sensor,
            reset,
            change_interrupt,
        };

        //// Reset Sensors
        // Reset all sensors
        sensor_one.reset.set_low().unwrap();
        sensor_two.reset.set_low().unwrap();
        sensor_three.reset.set_low().unwrap();
        // Build timer
        let mut timer = timer::Timer::tim3(c.device.TIM3, 1.khz(), clocks);
        block!(timer.wait()).unwrap(); //wait for 1ms to pass
                                       // Pull reset pins back up
        sensor_one.reset.set_high().unwrap();
        sensor_two.reset.set_high().unwrap();
        sensor_three.reset.set_high().unwrap();

        loop {
            //Wait for Sensor being ready
            //Note: The CHANGE line is pulled low 100 ms after power-up or reset. //Chapter 2.7
            if sensor_one.change_interrupt.is_low().unwrap()
                && sensor_two.change_interrupt.is_low().unwrap()
                && sensor_three.change_interrupt.is_low().unwrap()
            {
                break;
            }
        }

        rtic::pend(EXTI0);
        rtic::pend(EXTI1);
        rtic::pend(EXTI2);

        //Initialize sensors
        sensor_one.sensor.sync_all().unwrap();
        sensor_two.sensor.sync_all().unwrap();
        sensor_three.sensor.sync_all().unwrap();

        //sensor_one.sensor.set_negative_threshold(100, Key1).unwrap();
        //sensor_two.sensor.set_negative_threshold(100, Key1).unwrap();
        //sensor_three.sensor.set_negative_threshold(100, Key1).unwrap();
        //Set AKS to 0 for all Keys, so they are not Grouped
        for i in 0..7 {
            sensor_one.sensor.set_aks(0, Key::from(i)).unwrap();
            sensor_two.sensor.set_aks(0, Key::from(i)).unwrap();
            sensor_three.sensor.set_aks(0, Key::from(i)).unwrap();
        }

        //let mut timer = timer::Timer::tim3(c.device.TIM3, 1.khz(), clocks);
        //Use previously created timer to trigger TIM3 Interrupt every milli second
        timer.listen(timer::Event::TimeOut);
        init::LateResources {
            led_1,
            led_2,
            led_3,
            timer,
            usb_device,
            usb_class,
            sensor_one,
            sensor_two,
            sensor_three,
        }
    }

    // #[interrupt(resources = [ITM, EXTI])]
    // fn EXTI15_10(){
    //     let stim = &mut resources.ITM.stim[0];
    //     iprintln!(stim, "EXTI4 {:?}", resources.EXTI.pr.read().pr12().bit());
    //     iprintln!(stim, "EXTI4 {:?}", resources.EXTI.pr.read().pr13().bit());
    //     resources.EXTI.pr.modify(|_, w| w.pr13().set_bit());
    // }

    #[task(binds = EXTI2, resources = [sensor_one])]
    fn interrupt_sensor_one(c: interrupt_sensor_one::Context) {
        c.resources
            .sensor_one
            .change_interrupt
            .clear_interrupt_pending_bit();

        //Chapter 2.7
        c.resources
            .sensor_one
            .sensor
            .read_detection_status()
            .unwrap();
        c.resources
            .sensor_one
            .sensor
            .read_full_key_status()
            .unwrap();
    }

    #[task(binds = EXTI1, resources = [sensor_two])]
    fn interrupt_sensor_two(c: interrupt_sensor_two::Context) {
        c.resources
            .sensor_two
            .change_interrupt
            .clear_interrupt_pending_bit();

        //Chapter 2.7
        c.resources
            .sensor_two
            .sensor
            .read_detection_status()
            .unwrap();
        c.resources
            .sensor_two
            .sensor
            .read_full_key_status()
            .unwrap();
    }

    #[task(binds = EXTI0, resources = [sensor_three])]
    fn interrupt_sensor_three(c: interrupt_sensor_three::Context) {
        c.resources
            .sensor_three
            .change_interrupt
            .clear_interrupt_pending_bit();

        //Chapter 2.7
        c.resources
            .sensor_three
            .sensor
            .read_detection_status()
            .unwrap();
        c.resources
            .sensor_three
            .sensor
            .read_full_key_status()
            .unwrap();
    }

    #[task(binds = TIM3, resources = [usb_class, sensor_one, sensor_two, sensor_three, timer])]
    fn report(c: report::Context) {
        c.resources.timer.clear_interrupt(timer::Event::TimeOut);
        let one = c.resources.sensor_one.sensor.read_cached_full_key_status();
        let two = c.resources.sensor_two.sensor.read_cached_full_key_status();
        let three = c
            .resources
            .sensor_three
            .sensor
            .read_cached_full_key_status();
        c.resources
            .usb_class
            .write(&key_status_to_report(one, two, three));
    }

    #[task(binds = OTG_FS, resources = [usb_device, usb_class])]
    fn usb_tx(mut c: usb_tx::Context) {
        usb_poll(&mut c.resources.usb_device, &mut c.resources.usb_class);
    }

    #[task(binds = OTG_FS_WKUP, resources = [usb_device, usb_class])]
    fn usb_rx(mut c: usb_rx::Context) {
        usb_poll(&mut c.resources.usb_device, &mut c.resources.usb_class);
    }
};

fn usb_poll(
    usb_device: &mut UsbTouchButtonPanelDevice,
    touch_panel: &mut UsbTouchButtonPanelClass,
) {
    if usb_device.poll(&mut [touch_panel]) {
        touch_panel.poll();
    }
}

fn key_status_to_report(one: [bool; 7], two: [bool; 7], three: [bool; 7]) -> [u8; 4] {
    let mut shift = 0;
    let mut index = 1;
    let mut report = [0 as u8; 4];
    for keys in &[one, two, three] {
        for s in keys {
            if *s {
                report[index] += 1 << shift;
            }
            if shift == 7 {
                // If shift is already 7, reset it to 0 and increase the index
                index += 1;
                shift = 0;
            } else {
                // Else just increase shift by 1
                shift += 1;
            }
            if shift == 1 && index == 3 {
                //If last relevant button was processed
                break;
            }
        }
    }

    report
}
