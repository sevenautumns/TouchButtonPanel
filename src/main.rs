#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod touch_button_panel;

use panic_semihosting as _;

pub use rtic::app;

use stm32f4xx_hal::gpio::{
    gpiob::{PB0, PB14, PB7},
    Output, PushPull,
};
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::stm32;
use stm32f4xx_hal::timer;
use stm32f4xx_hal::otg_fs::{UsbBusType, USB};
use usb_device::bus::UsbBusAllocator;
use stm32f4xx_hal::rcc::Rcc;
use usb_device::class::UsbClass as _;

type UsbTouchButtonPanelDevice = touch_button_panel::Device<'static, UsbBusType>;
type UsbTouchButtonPanelClass = touch_button_panel::Class<'static, UsbBusType>;

#[app(device = stm32f4xx_hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        led_red: PB14<Output<PushPull>>,
        led_blue: PB7<Output<PushPull>>,
        led_green: PB0<Output<PushPull>>,
        timer: timer::Timer<stm32::TIM3>,
        usb_device: UsbTouchButtonPanelDevice,
        usb_class : UsbTouchButtonPanelClass,
        led_state: u8,
    }

    #[init]
    fn init(c: init::Context) -> init::LateResources {
        static mut EP_MEMORY: [u32; 1024] = [0; 1024];
        static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;

        let rcc = c.device.RCC.constrain();
        let gpioa = c.device.GPIOA.split();
        let gpiob = c.device.GPIOB.split();
        let _gpioc = c.device.GPIOC.split();
        let led_red = gpiob
            .pb14
            .into_push_pull_output();
        let led_blue = gpiob
            .pb7
            .into_push_pull_output();
        let led_green = gpiob
            .pb0
            .into_push_pull_output();

        // let mut flash = c.device.FLASH.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(25.mhz())
            .sysclk(84.mhz())
            .require_pll48clk()
            .freeze();

        let mut timer = timer::Timer::tim3(c.device.TIM3, 1.hz(), clocks);
        timer.listen(timer::Event::TimeOut);

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

        let led_state = 0;

        init::LateResources {
            led_red,
            led_blue,
            led_green,
            led_state,
            timer,
            usb_device,
            usb_class,
        }
    }

    #[task(binds = TIM3, priority = 3, resources = [led_red, led_blue, led_green, led_state, timer])]
    fn led_blink(c: led_blink::Context) {
        c.resources.timer.clear_interrupt(timer::Event::TimeOut);

        match c.resources.led_state {
            0 => {
                *c.resources.led_state += 1;
                c.resources.led_red.toggle().unwrap();
            }
            1 => {
                *c.resources.led_state += 1;
                c.resources.led_blue.toggle().unwrap();
            }
            2 => {
                *c.resources.led_state += 1;
                c.resources.led_green.toggle().unwrap();
            }
            3 => {
                *c.resources.led_state += 1;
                c.resources.led_green.toggle().unwrap();
            }
            4 => {
                *c.resources.led_state += 1;
                c.resources.led_blue.toggle().unwrap();
            }
            5 => {
                *c.resources.led_state = 0;
                c.resources.led_red.toggle().unwrap();
            }
            _ => {
                *c.resources.led_state = 0;
                c.resources.led_red.set_low().unwrap();
                c.resources.led_blue.set_low().unwrap();
                c.resources.led_green.set_low().unwrap();
            }
        }
    }

    #[task(binds = OTG_FS, priority = 2, resources = [usb_device, usb_class])]
    fn usb_tx(mut c: usb_tx::Context) {
        usb_poll(&mut c.resources.usb_device, &mut c.resources.usb_class);
    }

    #[task(binds = OTG_FS_WKUP, priority = 2, resources = [usb_device, usb_class])]
    fn usb_rx(mut c: usb_rx::Context) {
        usb_poll(&mut c.resources.usb_device, &mut c.resources.usb_class);
    }
};

fn usb_poll(usb_device: &mut UsbTouchButtonPanelDevice, touch_panel: &mut UsbTouchButtonPanelClass){
    if usb_device.poll(&mut [touch_panel]){
        touch_panel.poll();
    }
}