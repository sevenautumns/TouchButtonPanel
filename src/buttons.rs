use arrayvec::ArrayVec;
use core::convert::Infallible;
use embedded_hal::digital::v2::InputPin;
use stm32f4xx_hal::gpio::ExtiPin;
use stm32f4xx_hal::stm32::EXTI;

pub struct Buttons<BTN: InputPin<Error = Infallible> + ExtiPin> {
    buttons: [BTN; 8],
    state: u8,
}

impl<BTN: InputPin<Error = Infallible> + ExtiPin> Buttons<BTN> {
    pub fn new(buttons: [BTN; 8]) -> Self {
        Buttons { buttons, state: 0 }
    }

    pub fn enable_interrupt(&mut self, button: usize, exti: &mut EXTI) {
        self.buttons.get_mut(button).unwrap().enable_interrupt(exti);
    }

    pub fn disable_interrupt(&mut self, button: usize, exti: &mut EXTI) {
        self.buttons
            .get_mut(button)
            .unwrap()
            .disable_interrupt(exti);
    }

    pub fn clear_pending_interrupt(&mut self, button: usize) {
        self.buttons
            .get_mut(button)
            .unwrap()
            .clear_interrupt_pending_bit();
    }

    pub fn update_button_status(&mut self, button: usize) {
        let set = self.buttons.get_mut(button).unwrap().is_low().unwrap();
        self.set_cached_button_status(button, set);
    }

    pub fn set_cached_button_status(&mut self, button: usize, value: bool) {
        if value {
            bit_set(&mut self.state, button as u8);
        } else {
            bit_clear(&mut self.state, button as u8);
        }
    }

    pub fn get_cached_button_status(&self, button: usize) -> bool {
        bit_check(self.state, button as u8)
    }

    pub fn toggle_cached_button_status(&mut self, button: usize) {
        bit_toggle(&mut self.state, button as u8)
    }

    pub fn get_full_cached_status(&self) -> [bool; 8] {
        let mut status: ArrayVec<bool, 8> = ArrayVec::new();
        for i in 0..8 {
            status.push(bit_check(self.state, i))
        }
        status.into_inner().ok().unwrap()
    }
}

pub fn bit_check(byte: u8, n: u8) -> bool {
    (byte >> n) & 1 == 1
}

pub fn bit_set(byte: &mut u8, n: u8) {
    *byte |= 1 << n;
}

pub fn bit_clear(byte: &mut u8, n: u8) {
    *byte &= !(1 << n);
}

pub fn bit_toggle(byte: &mut u8, n: u8) {
    *byte ^= 1 << n;
}
