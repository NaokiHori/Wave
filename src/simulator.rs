mod converter;
mod coordinate;
mod field;
mod integrate;
pub mod metrics;
mod source;

use super::random::Random;
use converter::Converter;
use field::Field;
use metrics::Metrics;
use source::Source;

pub const NDIMS: usize = 2;

pub struct Config {
    pub lengths: [f64; NDIMS],
    pub nitems: [usize; NDIMS],
    pub param_c2: f64,
    pub param_nu: f64,
}

pub struct Simulator {
    config: Config,
    rng: Random,
    converter: Converter,
    field: Field,
    source: Source,
    dt: f64,
    pub iter_max: u32,
}

impl Simulator {
    pub fn new(random_seed: u64, config: Config, dt_max: f64) -> Self {
        let mut rng = Random::new(random_seed);
        let mut converter = Converter::new(&config.nitems);
        let field = Field::new(&config, &mut converter);
        let source = Source::new(&config, &mut rng);
        let mut dt = dt_max;
        decide_dt(&config, &mut dt);
        let iter_max: u32 = (dt_max / dt).ceil() as u32;
        Self {
            config,
            rng,
            converter,
            field,
            source,
            dt,
            iter_max,
        }
    }

    pub fn integrate(&mut self, time: &mut f64) {
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

    pub fn get_pos(&mut self) -> &[f64] {
        let pos_phys: &mut [f64] = &mut self.field.buf;
        self.converter.freq_to_phys(&self.field.pos, pos_phys);
        pos_phys
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
