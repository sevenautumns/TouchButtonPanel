use crate::hid::HIDClass;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;

pub type Class<'a, B> = HIDClass<'a, B>;
pub type Device<'a, B> = UsbDevice<'a, B>;

pub fn new_class<B>(bus: &UsbBusAllocator<B>) -> Class<'_, B>
where
    B: usb_device::bus::UsbBus,
{
    //HIDClass::new(bus, TouchButtonPanelReport::desc(), 60)
    HIDClass::new(bus)
}

pub fn new_device<B>(bus: &UsbBusAllocator<B>) -> Device<'_, B>
where
    B: usb_device::bus::UsbBus,
{
    // https://github.com/pidcodes/pidcodes.github.com/pull/533
    // UsbDeviceBuilder::new(bus, UsbVidPid(0x1209, 0xc0d4))
    UsbDeviceBuilder::new(bus, UsbVidPid(0x1209, 0x0001))
        .manufacturer("Autumnal")
        .product("Touch Button Panel")
        .serial_number(env!("CARGO_PKG_VERSION"))
        // .device_class(0xEF)
        .build()
}
