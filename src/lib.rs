mod drawer;
mod random;
mod simulator;

use wasm_bindgen::prelude::*;

pub use drawer::Drawer;
pub use simulator::Simulator;

#[wasm_bindgen(start)]
pub fn init() {}
