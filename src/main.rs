#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;

mod actuator;
mod control;
mod localization;
mod logic;
mod tank_chassis;
mod utils;

use alloc::sync::Arc;
use core::time::Duration;

use vexide::{
    core::{sync::Mutex, time::Instant},
    prelude::*,
};

use crate::{
    actuator::MotorGroup,
    tank_chassis::{SimpleController, TankChassis},
};
struct Robot {
    controller: Controller,
    chassis: TankChassis<SimpleController>,
}

impl Compete for Robot {
    async fn autonomous(&mut self) {
        println!("Autonomous!");
    }

    async fn driver(&mut self) {
        println!("Driver!");
        loop {
            let start = Instant::now();

            let state = self.controller.state().unwrap_or_default();
            let left = state.left_stick.y();
            let right = state.right_stick.y();
            self.chassis.tank(left, right).await;

            sleep_until(
                start
                    .checked_add(Duration::from_millis(10))
                    .unwrap_or(Instant::now()),
            )
            .await;
        }
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let l1 = Motor::new(peripherals.port_1, Gearset::Blue, Direction::Forward);
    let l2 = Motor::new(peripherals.port_2, Gearset::Blue, Direction::Forward);
    let l3 = Motor::new(peripherals.port_3, Gearset::Blue, Direction::Reverse);
    let r1 = Motor::new(peripherals.port_6, Gearset::Blue, Direction::Forward);
    let r2 = Motor::new(peripherals.port_7, Gearset::Blue, Direction::Forward);
    let r3 = Motor::new(peripherals.port_8, Gearset::Blue, Direction::Reverse);
    let chassis = TankChassis::new(
        Arc::new(Mutex::new(MotorGroup::new(vec![l1, l2, l3]))),
        Arc::new(Mutex::new(MotorGroup::new(vec![r1, r2, r3]))),
        SimpleController {},
        SimpleController {},
    );

    let robot = Robot {
        controller: peripherals.primary_controller,
        chassis,
    };
    robot.compete().await;
}
