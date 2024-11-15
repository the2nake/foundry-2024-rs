use alloc::{boxed::Box, sync::Arc};

use vexide::core::sync::Mutex;

use crate::{actuator::MotorGroup, logic::State};
pub struct TankChassis {
    left: Arc<Mutex<MotorGroup>>,
    right: Arc<Mutex<MotorGroup>>,
    left_ctrl: Box<dyn State<f64, f64>>, // takes ang vel, returns a voltage
    right_ctrl: Box<dyn State<f64, f64>>, // takes ang vel, returns a voltage
}

impl TankChassis {
    pub fn new(left: Arc<Mutex<MotorGroup>>, right: Arc<Mutex<MotorGroup>>) -> Self {
        TankChassis {
            left,
            right,
            left_ctrl: Box::new(SimpleController {}),
            right_ctrl: Box::new(SimpleController {}),
        }
    }

    pub async fn arcade(&mut self, throttle: f64, turn: f64) {
        self.set_left(throttle - turn).await;
        self.set_right(throttle + turn).await;
    }

    pub async fn tank(&mut self, left: f64, right: f64) {
        self.set_left(left).await;
        self.set_right(right).await;
    }

    async fn set_left(&mut self, target: f64) {
        self.left
            .lock()
            .await
            .set_voltage(self.left_ctrl.get(&target));
    }

    async fn set_right(&mut self, target: f64) {
        self.right
            .lock()
            .await
            .set_voltage(self.right_ctrl.get(&target));
    }

    fn left_use(&mut self, ctrl: Box<dyn State<f64, f64>>) {
        self.left_ctrl = ctrl;
    }

    fn right_use(&mut self, ctrl: Box<dyn State<f64, f64>>) {
        self.right_ctrl = ctrl;
    }
}

pub struct SimpleController {}

impl State<f64, f64> for SimpleController {
    fn init(&mut self) {}
    fn get(&mut self, input: &f64) -> f64 {
        12.0 * (*input)
    }
}
