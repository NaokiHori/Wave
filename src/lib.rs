mod drawer;
mod simulator;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Entrypoint {
    time: f64,
    simulator: simulator::Simulator,
    drawer: drawer::Drawer,
}

#[wasm_bindgen]
impl Entrypoint {
    pub fn new(lengths: &[f64], nitems: &[usize], param_c2: f64, param_nu: f64) -> Self {
        let lengths: [f64; 2] = [lengths[0], lengths[1]];
        let nitems: [usize; 2] = [nitems[0], nitems[1]];
        let config = simulator::Config {
            lengths,
            nitems,
            param_c2,
            param_nu,
        };
        let time = 0f64;
        let simulator = simulator::Simulator::new(&config);
        let drawer = drawer::Drawer::new(&config.nitems);
        Self {
            time,
            simulator,
            drawer,
        }
    }

    pub fn update(&mut self) {
        let time: &mut f64 = &mut self.time;
        let dt = 4e-3f64;
        let time_max = *time + dt;
        loop {
            self.simulator.integrate(dt, time);
            if time_max < *time {
                break;
            }
        }
        let pos: &[f64] = self.simulator.pos();
        self.drawer.draw(pos);
    }

    pub fn pixels(&mut self) -> *const u8 {
        self.drawer.pixels()
    }
}

#[wasm_bindgen(start)]
pub fn init() {}
