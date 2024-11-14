use alloc::{boxed::Box, sync::Arc, vec::Vec};

use vexide::core::sync::Mutex;

use crate::{actuator::MotorGroup, logic::State};
pub struct TankChassis<T: State<f64, f64>> {
    left: Arc<Mutex<MotorGroup>>,
    right: Arc<Mutex<MotorGroup>>,
    left_controller: T,  // takes a percentage, returns a voltage
    right_controller: T, // takes a percentage, returns a voltage
}

impl<T: State<f64, f64>> TankChassis<T> {
    pub fn new(
        left: Arc<Mutex<MotorGroup>>,
        right: Arc<Mutex<MotorGroup>>,
        left_controller: T,
        right_controller: T,
    ) -> Self {
        TankChassis {
            left,
            right,
            left_controller,
            right_controller,
        }
    }

    pub async fn arcade(&mut self, throttle: f64, turn: f64) {
        let left_target = throttle - turn;
        let right_target = throttle + turn;
        self.left
            .lock()
            .await
            .set_voltage(self.left_controller.get(&left_target));
        self.right
            .lock()
            .await
            .set_voltage(self.right_controller.get(&right_target));
    }

    pub async fn tank(&mut self, left: f64, right: f64) {
        self.left
            .lock()
            .await
            .set_voltage(self.left_controller.get(&left));
        self.right
            .lock()
            .await
            .set_voltage(self.right_controller.get(&right));
    }
}

pub struct SimpleController {}

impl State<f64, f64> for SimpleController {
    fn init(&mut self) {}
    fn get(&mut self, input: &f64) -> f64 {
        12.0 * (*input)
    }
}
