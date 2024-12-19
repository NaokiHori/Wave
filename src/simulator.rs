mod converter;
mod coordinate;
mod field;
mod integrate;
pub mod metrics;
mod source;

use wasm_bindgen::prelude::*;

use super::random::Random;
use converter::Converter;
use field::Field;
use metrics::Metrics;
use source::Source;

pub const NDIMS: usize = 2;

#[wasm_bindgen]
pub struct Config {
    lengths: [f64; NDIMS],
    nitems: [usize; NDIMS],
    param_c2: f64,
    param_nu: f64,
}

#[wasm_bindgen]
pub struct Simulator {
    time: f64,
    config: Config,
    rng: Random,
    converter: Converter,
    field: Field,
    source: Source,
    dt: f64,
    iter_max: u32,
}

#[wasm_bindgen]
impl Simulator {
    #[wasm_bindgen(constructor)]
    pub fn new(
        random_seed: u64,
        lengths: &[f64],
        nitems: &[u32],
        param_c2: f64,
        param_nu: f64,
        dt_max: f64,
    ) -> Self {
        let lengths: [f64; 2] = [lengths[0], lengths[1]];
        let nitems: [usize; 2] = [nitems[0].try_into().unwrap(), nitems[1].try_into().unwrap()];
        let config = Config {
            lengths,
            nitems,
            param_c2,
            param_nu,
        };
        let time = 0f64;
        let mut rng = Random::new(random_seed);
        let mut converter = Converter::new(&config.nitems);
        let field = Field::new(&config, &mut converter);
        let source = Source::new(&config, &mut rng);
        let mut dt = dt_max;
        decide_dt(&config, &mut dt);
        let iter_max: u32 = (dt_max / dt).ceil() as u32;
        Self {
            time,
            config,
            rng,
            converter,
            field,
            source,
            dt,
            iter_max,
        }
    }

    pub fn integrate(&mut self) {
        let time: &mut f64 = &mut self.time;
        for _ in 0..self.iter_max {
            integrate::integrate(
                &self.config,
                &mut self.rng,
                &mut self.field,
                &mut self.source,
                self.dt,
            );
            *time += self.dt;
        }
    }

    pub fn get_metrics(&mut self) -> Metrics {
        metrics::get(self)
    }

    pub fn get_pos(&mut self) -> *const f64 {
        let pos_phys: &mut [f64] = &mut self.field.buf;
        self.converter.freq_to_phys(&self.field.pos, pos_phys);
        pos_phys.as_ptr()
    }

    pub fn get_dt(&self) -> f64 {
        self.dt
    }
}

fn decide_dt(config: &Config, dt: &mut f64) {
    // NOTE: maximum dt is given by the user
    for dim in 0..NDIMS {
        let length: f64 = config.lengths[dim];
        let nitems: usize = config.nitems[dim];
        *dt = (*dt).min(length / nitems as f64 / config.param_c2.sqrt());
    }
}
