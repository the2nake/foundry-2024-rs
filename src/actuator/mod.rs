use alloc::vec::Vec;

use vexide::prelude::Motor;

pub struct MotorGroup {
    motors: Vec<Motor>,
}

impl MotorGroup {
    pub fn new(motors: Vec<Motor>) -> Self {
        MotorGroup { motors }
    }

    pub fn set_voltage(&mut self, volts: f64) {
        for motor in self.motors.iter_mut() {
            motor.set_voltage(volts).ok();
        }
    }

    pub fn set_velocity(&mut self, rpm: f64) {
        for motor in self.motors.iter_mut() {
            motor.set_velocity(rpm as i32).ok();
        }
    }

    pub fn degrees(&self) -> f64 {
        let mut sum = 0.0;
        let mut count = self.motors.len();
        for motor in self.motors.iter() {
            sum += match motor.position() {
                Ok(x) => x.as_degrees(),
                Err(_) => {
                    count -= 1;
                    0.0
                }
            }
        }
        sum / count as f64
    }

    pub fn radians(&self) -> f64 {
        let mut sum = 0.0;
        let mut count = self.motors.len();
        for motor in self.motors.iter() {
            sum += match motor.position() {
                Ok(x) => x.as_radians(),
                Err(_) => {
                    count -= 1;
                    0.0
                }
            }
        }
        sum / count as f64
    }
}
