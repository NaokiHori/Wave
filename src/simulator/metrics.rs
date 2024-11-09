use wasm_bindgen::prelude::*;

use super::field::Field;
use super::Simulator;

#[wasm_bindgen]
pub struct Metrics {
    pub min_displacement: f64,
    pub max_displacement: f64,
    pub mean_displacement: f64,
}

pub fn get(simulator: &mut Simulator) -> Metrics {
    let field: &mut Field = &mut simulator.field;
    let pos_freq: &[f64] = &field.pos;
    let (min_displacement, max_displacement, mean_displacement) = {
        let freq: &[f64] = pos_freq;
        let phys: &mut [f64] = &mut field.buf;
        simulator.converter.freq_to_phys(freq, phys);
        let mut min = f64::MAX;
        let mut max = f64::MIN;
        let mut mean = 0f64;
        for &p in phys.iter() {
            min = min.min(p);
            max = max.max(p);
            mean += p;
        }
        (min, max, mean)
    };
    Metrics {
        min_displacement,
        max_displacement,
        mean_displacement,
    }
}
