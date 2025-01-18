mod complex;
mod rdft;

use super::NDIMS;

use rdft::RdftPlan;

pub struct Converter {
    rdft_plans: [RdftPlan; NDIMS],
    nitems: [usize; NDIMS],
    buffer: Vec<f32>,
}

impl Converter {
    pub fn new(nitems: &[usize; NDIMS]) -> Self {
        let rdft_plans: [RdftPlan; NDIMS] = [
            RdftPlan::new(nitems[0]).unwrap(),
            RdftPlan::new(nitems[1]).unwrap(),
        ];
        let buffer = vec![0f32; nitems[0] * nitems[1]];
        Self {
            rdft_plans,
            nitems: *nitems,
            buffer,
        }
    }

    pub fn phys_to_freq(&mut self, phys: &[f32], freq: &mut [f32]) {
        let nitems: &[usize; NDIMS] = &self.nitems;
        let nitems_total: usize = nitems[0] * nitems[1];
        let buffer = &mut self.buffer;
        buffer[..nitems_total].copy_from_slice(&phys[..nitems_total]);
        for j in 0..nitems[1] {
            self.rdft_plans[0].exec_f(&mut buffer[j * nitems[0]..]);
        }
        transpose(nitems[0], nitems[1], buffer, freq);
        for i in 0..nitems[0] {
            self.rdft_plans[1].exec_f(&mut freq[i * nitems[1]..]);
        }
    }

    pub fn freq_to_phys(&mut self, freq: &[f32], phys: &mut [f32]) {
        let nitems: &[usize; NDIMS] = &self.nitems;
        let nitems_total: usize = nitems[0] * nitems[1];
        let buffer = &mut self.buffer;
        buffer[..nitems_total].copy_from_slice(&freq[..nitems_total]);
        for i in 0..nitems[0] {
            self.rdft_plans[1].exec_b(&mut buffer[i * nitems[1]..]);
        }
        transpose(nitems[1], nitems[0], buffer, phys);
        for j in 0..nitems[1] {
            self.rdft_plans[0].exec_b(&mut phys[j * nitems[0]..]);
        }
    }
}

fn transpose(nx: usize, ny: usize, bef: &[f32], aft: &mut [f32]) {
    for j in 0..ny {
        for i in 0..nx {
            aft[i * ny + j] = bef[j * nx + i];
        }
    }
}

#[cfg(test)]
mod test {
    use super::transpose;
    #[test]
    fn check() {
        let nx = 3usize;
        let ny = 2usize;
        let bef = [0f32, 1f32, 2f32, 3f32, 4f32, 5f32];
        let mut aft = vec![0f32; nx * ny];
        transpose(nx, ny, &bef, &mut aft);
        assert!(0f32 == aft[0]);
        assert!(3f32 == aft[1]);
        assert!(1f32 == aft[2]);
        assert!(4f32 == aft[3]);
        assert!(2f32 == aft[4]);
        assert!(5f32 == aft[5]);
    }
}
