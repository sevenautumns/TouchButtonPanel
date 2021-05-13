//use gyuvl53l0x::VL53L0X;
use crate::BusType;
use arrayvec::ArrayVec;
use embedded_hal::digital::v2::OutputPin;

#[cfg(feature = "rtt")]
use rtt_target::rprintln;

use core::cmp::min;
use shared_bus_rtic::SharedBus;
use vl53l0x::Vl53l0x;

const ADDRESS_DEFAULT: u8 = 0x29;
const SIZE: usize = 8;
const OFFSET: [u16; 8] = [10, 13, 6, 3, 6, 13, 16, 16];

pub struct DistanceMeasurement {
    sensors: [vl53l0x::Vl53l0x<SharedBus<BusType>>; SIZE],
    reads: [Option<usize>; SIZE],
}

impl DistanceMeasurement {
    pub fn new(reset: &mut [impl OutputPin], i2c: BusType) -> Result<DistanceMeasurement, ()> {
        let i2c = shared_bus_rtic::new!(i2c, BusType);
        let mut sensors: ArrayVec<Vl53l0x<SharedBus<BusType>>, SIZE> = ArrayVec::new();
        reset.iter_mut().for_each(|r| {
            r.set_low().ok();
        });
        cortex_m::asm::delay(840_000); //10ms
        reset.iter_mut().for_each(|r| {
            r.set_high().ok();
        });
        cortex_m::asm::delay(84_000); //1ms
        reset.iter_mut().for_each(|r| {
            r.set_low().ok();
        });

        for i in (0..SIZE).rev() {
            #[cfg(feature = "rtt")]
            rprintln!("{}", i);
            reset.get_mut(i).ok_or(())?.set_high().ok();
            cortex_m::asm::delay(840_000);
            let mut s = Vl53l0x::new(i2c.acquire()).map_err(|e| (e, i)).unwrap();
            s.set_measurement_timing_budget(32_000).unwrap();
            s.set_device_address(ADDRESS_DEFAULT + (i as u8) + 10)
                .or(Err(()))?;
            s.start_continuous(0).unwrap();
            sensors.push(s);
        }

        Ok(DistanceMeasurement {
            sensors: sensors.into_inner().or(Err(()))?,
            reads: Default::default(),
        })
    }

    pub fn read_milli(&mut self, index: usize) -> Result<u16, ()> {
        let read = self
            .sensors
            .get_mut(index)
            .ok_or(())?
            .read_range_continuous_millimeters_blocking()
            .map_err(|e| (e, index))
            .unwrap();
        let offset = min(OFFSET[index], read);
        Ok(read - offset)
    }

    pub fn read_section(&mut self, index: usize) -> Result<Option<usize>, ()> {
        let x = self.read_milli(index)?;
        Ok(match x {
            0..=120 => Some(0),
            121..=212 => Some(1),
            213..=330 => Some(2),
            _ => None,
        })
    }

    pub fn update_read_all(&mut self) -> Result<(), ()> {
        let mut values: [Option<usize>; SIZE] = Default::default();
        for (i, v) in values.iter_mut().enumerate() {
            *v = self.read_section(i)?;
        }
        for (r, v) in self.reads.iter_mut().zip(values.iter()) {
            *r = *v
        }

        Ok(())
    }

    pub fn get_cached_buttons(&self) -> [bool; 17] {
        let mut buttons = [false; 17];
        let mut mid: usize = 0;

        //Sensor 2 und 6 werden für die Mitte ignoriert, da sie ein wenig zu weit nach oben messen
        const IGNORED: [usize; 2] = [2, 6];

        for (i, value) in self.reads.iter().enumerate() {
            match value {
                Some(0) => buttons[i] = true,
                Some(1) => buttons[i + 8] = true,
                Some(2) => {
                    if !IGNORED.contains(&i) {
                        mid += 1
                    }
                }
                _ => {}
            }
        }

        if mid >= 2 {
            buttons[16] = true;
        }

        buttons
    }
}
