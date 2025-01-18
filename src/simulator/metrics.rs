use wasm_bindgen::prelude::*;

use super::field::Field;
use super::Simulator;

#[wasm_bindgen]
pub struct Metrics {
    pub min_displacement: f32,
    pub max_displacement: f32,
    pub mean_displacement: f32,
}

pub fn get(simulator: &mut Simulator) -> Metrics {
    let field: &mut Field = &mut simulator.field;
    let pos_freq: &[f32] = &field.pos;
    let (min_displacement, max_displacement, mean_displacement) = {
        let freq: &[f32] = pos_freq;
        let phys: &mut [f32] = &mut field.pos_phys;
        simulator.converter.freq_to_phys(freq, phys);
        let mut min = f32::MAX;
        let mut max = f32::MIN;
        let mut mean = 0f32;
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
