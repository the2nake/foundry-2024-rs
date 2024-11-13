use core::f64::consts::TAU;

pub fn shorter_rad(h0: f64, hf: f64) -> f64 {
    shorter_turn(h0, hf, TAU)
}

pub fn shorter_deg(h0: f64, hf: f64) -> f64 {
    shorter_turn(h0, hf, 360.0)
}

fn shorter_turn(h0: f64, hf: f64, modulo: f64) -> f64 {
    (hf - h0 + modulo / 2.0) % modulo - modulo / 2.0
}
