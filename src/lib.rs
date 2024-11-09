mod drawer;
mod random;
mod simulator;

use wasm_bindgen::prelude::*;

use drawer::Drawer;
use simulator::metrics::Metrics;
use simulator::{Config, Simulator};

#[wasm_bindgen]
pub struct Entrypoint {
    time: f64,
    simulator: Simulator,
    drawer: Drawer,
}

#[wasm_bindgen]
impl Entrypoint {
    pub fn new(
        random_seed: u64,
        lengths: &[f64],
        nitems: &[usize],
        param_c2: f64,
        param_nu: f64,
        dt_max: f64,
    ) -> Self {
        let lengths: [f64; 2] = [lengths[0], lengths[1]];
        let nitems: [usize; 2] = [nitems[0], nitems[1]];
        let config = Config {
            lengths,
            nitems,
            param_c2,
            param_nu,
        };
        let time = 0f64;
        let simulator = Simulator::new(random_seed, config, dt_max);
        let drawer = Drawer::new(&nitems);
        Self {
            time,
            simulator,
            drawer,
        }
    }

    pub fn update(&mut self) {
        let time: &mut f64 = &mut self.time;
        self.simulator.integrate(time);
        let pos: &[f64] = self.simulator.get_pos();
        self.drawer.draw(pos);
    }

    pub fn get_metrics(&mut self) -> Metrics {
        self.simulator.get_metrics()
    }

    pub fn get_dt(&self) -> f64 {
        self.simulator.get_dt()
    }

    pub fn pixels(&mut self) -> *const u8 {
        self.drawer.pixels()
    }
}

#[wasm_bindgen(start)]
pub fn init() {}
