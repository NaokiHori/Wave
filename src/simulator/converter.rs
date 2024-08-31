mod dst;

use super::NDIMS;

use dst::DstPlan;

pub struct Converter {
    dst_plans: [DstPlan; NDIMS],
    nitems: [usize; NDIMS],
    buffer: Vec<f64>,
}

impl Converter {
    pub fn new(nitems: &[usize; NDIMS]) -> Self {
        let dst_plans: [dst::DstPlan; NDIMS] = [DstPlan::new(nitems[0]), DstPlan::new(nitems[1])];
        let buffer = vec![0f64; nitems[0] * nitems[1]];
        Self {
            dst_plans,
            nitems: *nitems,
            buffer,
        }
    }

    pub fn phys_to_freq(&mut self, phys: &[f64], freq: &mut [f64]) {
        let nitems: &[usize; NDIMS] = &self.nitems;
        let nitems_total: usize = nitems[0] * nitems[1];
        let buffer = &mut self.buffer;
        buffer[..nitems_total].copy_from_slice(&phys[..nitems_total]);
        for j in 0..nitems[1] {
            self.dst_plans[0].exec_f(&mut buffer[j * nitems[0]..]);
        }
        transpose(nitems[0], nitems[1], buffer, freq);
        for i in 0..nitems[0] {
            self.dst_plans[1].exec_f(&mut freq[i * nitems[1]..]);
        }
    }

    pub fn freq_to_phys(&mut self, freq: &[f64], phys: &mut [f64]) {
        let nitems: &[usize; NDIMS] = &self.nitems;
        let nitems_total: usize = nitems[0] * nitems[1];
        let buffer = &mut self.buffer;
        buffer[..nitems_total].copy_from_slice(&freq[..nitems_total]);
        for i in 0..nitems[0] {
            self.dst_plans[1].exec_b(&mut buffer[i * nitems[1]..]);
        }
        transpose(nitems[1], nitems[0], buffer, phys);
        for j in 0..nitems[1] {
            self.dst_plans[0].exec_b(&mut phys[j * nitems[0]..]);
        }
    }
}

fn transpose(nx: usize, ny: usize, bef: &[f64], aft: &mut [f64]) {
    for j in 0..ny {
        for i in 0..nx {
            aft[i * ny + j] = bef[j * nx + i];
        }
    }
}

#[cfg(test)]
mod test_transpose {
    #[test]
    fn check() {
        let nx = 3usize;
        let ny = 2usize;
        let bef = [0f64, 1f64, 2f64, 3f64, 4f64, 5f64];
        let mut aft = vec![0f64; nx * ny];
        super::transpose(nx, ny, &bef, &mut aft);
        assert!(0f64 == aft[0]);
        assert!(3f64 == aft[1]);
        assert!(1f64 == aft[2]);
        assert!(4f64 == aft[3]);
        assert!(2f64 == aft[4]);
        assert!(5f64 == aft[5]);
    }
}
