use core::time::Duration;

use vexide::{core::time::Instant, prelude::Float};

use crate::{control::Settles, logic::State};

struct PID {
    kp: f64,
    ki: f64,
    kd: f64,

    output: f64,
    prev_err: Option<f64>,
    total_err: f64,
    prev_update: Instant,

    settle_elapsed: Duration, // duration in settled state
    settle_thres: f64,        // range of errors to settle at
    settle_time: Duration,    // minimum time to consider settled
}

impl PID {
    pub fn new(kp: f64, ki: f64, kd: f64, settle_thres: f64, settle_time: Duration) -> Self {
        Self {
            kp,
            ki,
            kd,
            output: 0.0,
            total_err: 0.0,
            prev_err: None,
            settle_elapsed: Duration::new(0, 0),
            settle_thres,
            settle_time,
            prev_update: Instant::now(),
        }
    }

    pub fn update(&mut self, mut err: f64) -> f64 {
        if err.is_nan() {
            err = 0.0;
        }

        let now = Instant::now();
        let duration = now
            .checked_duration_since(self.prev_update)
            .unwrap_or_default();
        let dt = duration.as_micros() as f64 / 1000.0;

        self.total_err += err * dt;

        let p = self.kp * err;
        let i = self.ki * self.total_err;
        let mut d = self.kd * (err - self.prev_err.unwrap_or(err)) / dt;
        if dt < 1.0 {
            d = 0.0;
        }
        self.output = p + i + d;

        self.prev_err = Some(err);
        self.prev_update = now;

        if err.abs() < self.settle_thres {
            self.settle_elapsed = self
                .settle_elapsed
                .checked_add(Duration::from_millis(dt as u64))
                .unwrap_or_default();
        } else {
            self.settle_elapsed = Duration::new(0, 0);
        }

        self.output
    }

    pub fn output(&self) -> f64 {
        self.output
    }
}

impl Settles for PID {
    fn settled(&self) -> bool {
        self.settle_elapsed > self.settle_time
    }
}

impl State<f64, f64> for PID {
    fn init(&mut self) {
        self.settle_elapsed = Duration::new(0, 0);
        self.prev_update = Instant::now();
        self.prev_err = None;
    }

    fn get(&mut self, input: &f64) -> f64 {
        self.update(*input)
    }
}
