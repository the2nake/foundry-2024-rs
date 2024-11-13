use alloc::{sync::Arc, vec::Vec};

use vexide::core::sync::Mutex;

use crate::actuator::MotorGroup;

pub struct TankChassis {
    left: Arc<Mutex<MotorGroup>>,
    right: Arc<Mutex<MotorGroup>>,
}

impl TankChassis {
    pub fn new(left: Arc<Mutex<MotorGroup>>, right: Arc<Mutex<MotorGroup>>) -> Self {
        TankChassis { left, right }
    }

    pub async fn arcade(&mut self, throttle: f64, turn: f64) {
        self.left.lock().await.set_voltage(12.0 * (throttle - turn));
        self.left.lock().await.set_voltage(12.0 * (throttle + turn));
    }

    pub async fn tank(&mut self, left: f64, right: f64) {
        self.left.lock().await.set_voltage(12.0 * left);
        self.right.lock().await.set_voltage(12.0 * right);
    }
}
