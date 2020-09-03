use at42qt1070::*;
use core::convert::Infallible;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use stm32f4xx_hal::gpio::ExtiPin;
use stm32f4xx_hal::i2c::*;
use stm32f4xx_hal::stm32::EXTI;

pub struct TouchSensor<I2C, SCL: PinScl<I2C>, SDA: PinSda<I2C>, RESET: OutputPin, CHANGE: InputPin>
{
    pub sensor: At42qt1070<I2c<I2C, (SCL, SDA)>>,
    pub reset: RESET,
    pub change_interrupt: CHANGE,
}

pub struct Buttons<
    BTN0: InputPin + ExtiPin,
    BTN1: InputPin + ExtiPin,
    BTN2: InputPin + ExtiPin,
    BTN3: InputPin + ExtiPin,
    BTN4: InputPin + ExtiPin,
    BTN5: InputPin + ExtiPin,
    BTN6: InputPin + ExtiPin,
    BTN7: InputPin + ExtiPin,
> {
    pub button_0: BTN0,
    pub button_1: BTN1,
    pub button_2: BTN2,
    pub button_3: BTN3,
    pub button_4: BTN4,
    pub button_5: BTN5,
    pub button_6: BTN6,
    pub button_7: BTN7,
    pub state: u8,
}

impl<
        BTN0: InputPin<Error = Infallible> + ExtiPin,
        BTN1: InputPin<Error = Infallible> + ExtiPin,
        BTN2: InputPin<Error = Infallible> + ExtiPin,
        BTN3: InputPin<Error = Infallible> + ExtiPin,
        BTN4: InputPin<Error = Infallible> + ExtiPin,
        BTN5: InputPin<Error = Infallible> + ExtiPin,
        BTN6: InputPin<Error = Infallible> + ExtiPin,
        BTN7: InputPin<Error = Infallible> + ExtiPin,
    > Buttons<BTN0, BTN1, BTN2, BTN3, BTN4, BTN5, BTN6, BTN7>
{
    pub fn set_interrupt_enabled(&mut self, button: u8, exti: &mut EXTI) {
        self.set_interrupt_capability(button, true, exti);
    }

    pub fn set_interrupt_disabled(&mut self, button: u8, exti: &mut EXTI) {
        self.set_interrupt_capability(button, false, exti);
    }

    fn set_interrupt_capability(&mut self, button: u8, enable: bool, exti: &mut EXTI) {
        match button {
            0 => {
                if enable {
                    self.button_0.disable_interrupt(exti)
                } else {
                    self.button_0.enable_interrupt(exti)
                }
            }
            1 => {
                if enable {
                    self.button_1.disable_interrupt(exti)
                } else {
                    self.button_1.enable_interrupt(exti)
                }
            }
            2 => {
                if enable {
                    self.button_2.disable_interrupt(exti)
                } else {
                    self.button_2.enable_interrupt(exti)
                }
            }
            3 => {
                if enable {
                    self.button_3.disable_interrupt(exti)
                } else {
                    self.button_3.enable_interrupt(exti)
                }
            }
            4 => {
                if enable {
                    self.button_4.disable_interrupt(exti)
                } else {
                    self.button_4.enable_interrupt(exti)
                }
            }
            5 => {
                if enable {
                    self.button_5.disable_interrupt(exti)
                } else {
                    self.button_5.enable_interrupt(exti)
                }
            }
            6 => {
                if enable {
                    self.button_6.disable_interrupt(exti)
                } else {
                    self.button_6.enable_interrupt(exti)
                }
            }
            7 => {
                if enable {
                    self.button_7.disable_interrupt(exti)
                } else {
                    self.button_7.enable_interrupt(exti)
                }
            }

            _ => {},
        }
    }

    pub fn clear_pending_interrupt_bit(&mut self, button: u8) {
        match button {
            0 => self.button_0.clear_interrupt_pending_bit(),
            1 => self.button_1.clear_interrupt_pending_bit(),
            2 => self.button_2.clear_interrupt_pending_bit(),
            3 => self.button_3.clear_interrupt_pending_bit(),
            4 => self.button_4.clear_interrupt_pending_bit(),
            5 => self.button_5.clear_interrupt_pending_bit(),
            6 => self.button_6.clear_interrupt_pending_bit(),
            7 => self.button_7.clear_interrupt_pending_bit(),
            _ => {},
        }
    }

    pub fn read_to_status(&mut self, button: u8) {
        let set = match button {
            0 => self.button_0.is_low().unwrap(),
            1 => self.button_1.is_low().unwrap(),
            2 => self.button_2.is_low().unwrap(),
            3 => self.button_3.is_low().unwrap(),
            4 => self.button_4.is_low().unwrap(),
            5 => self.button_5.is_low().unwrap(),
            6 => self.button_6.is_low().unwrap(),
            7 => self.button_7.is_low().unwrap(),
            _ => return,
        };

        if set {
            self.state |= 1 << button;
        } else {
            self.state &= !(1 << button);
        }
    }
}
