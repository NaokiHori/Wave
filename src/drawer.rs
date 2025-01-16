use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Drawer {
    nitems: [usize; 2],
    minmax: [f64; 2],
    buf: Vec<u8>,
}

#[wasm_bindgen]
impl Drawer {
    #[wasm_bindgen(constructor)]
    pub fn new(nitems: &[u32]) -> Self {
        let nitems: [usize; 2] = [nitems[0].try_into().unwrap(), nitems[1].try_into().unwrap()];
        let buf = vec![0u8; nitems[0] * nitems[1] * "rgba".len()];
        let minmax: [f64; 2] = [f64::MAX, f64::MIN];
        Self {
            nitems,
            minmax,
            buf,
        }
    }

    pub fn pixelize(&mut self, array_ptr: *const f64) -> *const u8 {
        let nitems: &[usize; 2] = &self.nitems;
        let array: &[f64] = unsafe { std::slice::from_raw_parts(array_ptr, nitems[0] * nitems[1]) };
        let pixels: &mut [u8] = &mut self.buf;
        let minmax: &mut [f64; 2] = &mut self.minmax;
        get_minmax(array, minmax);
        for n in 0..(nitems[0] * nitems[1]) {
            pixels[n] = (255f64 * (array[n] - minmax[0]) / (minmax[1] - minmax[0])) as u8;
        }
        pixels.as_ptr()
    }
}

fn get_minmax(array: &[f64], minmax: &mut [f64; 2]) {
    // NOTE: clip for better visibility
    // c = 1 denotes no-clipping
    let c = 0.25f64;
    for &value in array.iter() {
        minmax[0] = minmax[0].min(c * value);
        minmax[1] = minmax[1].max(c * value);
    }
}
