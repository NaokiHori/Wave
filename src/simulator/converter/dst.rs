pub struct DstPlan {
    /// size of the input / output signals
    nitems: usize,
    /// trigonometric table
    table: Vec<f64>,
    /// internal buffer
    buf: Vec<f64>,
}

fn get_phase(nitems: usize, n: usize, k: usize) -> f64 {
    2f64 * std::f64::consts::PI * (n as f64 + 0.5f64) * (k + 1) as f64 / (2 * nitems) as f64
}

impl DstPlan {
    pub fn new(nitems: usize) -> Self {
        // create a trigonometric table
        let mut table = Vec::<f64>::with_capacity(nitems);
        for i in 0..nitems {
            let phase: f64 = (std::f64::consts::PI * i as f64) / (2 * nitems) as f64;
            table.push(0.5f64 / phase.cos());
        }
        // prepare an internal buffer
        let buf = vec![0f64; nitems];
        Self { nitems, table, buf }
    }

    pub fn exec_f(&mut self, xs: &mut [f64]) {
        let nitems: usize = self.nitems;
        dst2(nitems, 1, &self.table, xs, &mut self.buf);
    }

    pub fn exec_b(&mut self, xs: &mut [f64]) {
        let nitems: usize = self.nitems;
        // normalize (N-1)-th wave number before executing DST3
        xs[nitems - 1] *= 0.5f64;
        dst3(nitems, 1, &self.table, xs, &mut self.buf);
    }
}

// discrete sine transform of type II
fn dst2(nitems: usize, stride: usize, table: &Vec<f64>, xs: &mut [f64], ys: &mut [f64]) {
    if 1 == nitems {
        xs[0] *= 2f64;
    } else if 0 == nitems % 2 {
        // divide and conquer
        let nh: usize = nitems / 2;
        // create input buffers of DST-II
        for n in 0..nh {
            // c: 1 / [ 2 cos(beta) ]
            let c: f64 = table[(2 * n + 1) * stride];
            let value0 = xs[n];
            let value1 = xs[nitems - 1 - n];
            ys[n] = c * (value0 + value1);
            ys[nh + n] = value0 - value1;
        }
        // solve two sub problems
        dst2(nh, stride * 2, table, &mut ys[0..], xs);
        dst2(nh, stride * 2, table, &mut ys[nh..], xs);
        // distribute even frequencies
        for k in 0..nh {
            // for the first element, "-1"-th element is zero
            let value0: f64 = if 0 == k { 0f64 } else { ys[k - 1] };
            let value1: f64 = ys[k];
            xs[k * 2] = value0 + value1;
        }
        // distribute odd frequencies
        for k in 0..nh {
            xs[k * 2 + 1] = ys[nh + k];
        }
    } else {
        // fallback to N^2 DST2
        for (k, y) in ys.iter_mut().enumerate().take(nitems) {
            *y = 0.;
            for (n, x) in xs.iter().enumerate().take(nitems) {
                let phase: f64 = get_phase(nitems, n, k);
                *y += 2f64 * x * phase.sin();
            }
        }
        xs[..nitems].copy_from_slice(&ys[..nitems]);
    }
}

// discrete sine transform of type III
fn dst3(nitems: usize, stride: usize, table: &Vec<f64>, xs: &mut [f64], ys: &mut [f64]) {
    if 1 == nitems {
        xs[0] *= 1f64;
    } else if 0 == nitems % 2 {
        // divide and conquer
        let nh: usize = nitems / 2;
        // create input buffers of DST-III
        for k in 0..nh {
            let value0: f64 = xs[k * 2];
            let value1: f64 = if nh - 1 == k { 0f64 } else { xs[k * 2 + 2] };
            ys[k] = value0 + value1;
        }
        for k in 0..nh {
            ys[nh + k] = xs[k * 2 + 1];
        }
        // solve two sub problems
        dst3(nh, stride * 2, table, &mut ys[0..], xs);
        dst3(nh, stride * 2, table, &mut ys[nh..], xs);
        // combine results of sub problems
        for n in 0..nh {
            // c: 1 / [ 2 cos(beta) ]
            let c: f64 = table[(2 * n + 1) * stride];
            let value0: f64 = 0.5f64 * c * ys[n];
            let value1: f64 = 0.5f64 * ys[nh + n];
            xs[n] = value0 + value1;
            xs[nitems - 1 - n] = value0 - value1;
        }
    } else {
        // fallback to N^2 DST3
        for (n, y) in ys.iter_mut().enumerate().take(nitems) {
            *y = 0f64;
            for (k, x) in xs.iter().enumerate().take(nitems) {
                let phase: f64 = get_phase(nitems, n, k);
                *y += x * phase.sin();
            }
            *y /= nitems as f64;
        }
        xs[..nitems].copy_from_slice(&ys[..nitems]);
    }
}

#[cfg(test)]
mod test_dst {
    /// A naive DST-II implementation.
    fn dst_f(nitems: usize, xs: &[f64], ys: &mut [f64]) -> () {
        for k in 0..nitems {
            let y: &mut f64 = &mut ys[k];
            *y = 0f64;
            for n in 0..nitems {
                let phase: f64 = super::get_phase(nitems, n, k);
                let x: f64 = xs[n];
                *y += 2f64 * x * phase.sin();
            }
        }
    }
    /// A naive DST-III implementation.
    fn dst_b(nitems: usize, xs: &[f64], ys: &mut [f64]) -> () {
        for n in 0..nitems {
            let y: &mut f64 = &mut ys[n];
            *y = 0f64;
            for k in 0..nitems {
                let phase: f64 = super::get_phase(nitems, n, k);
                let x: f64 = if nitems - 1 == k {
                    0.5f64 * xs[k]
                } else {
                    xs[k]
                };
                *y += x * phase.sin();
            }
            *y /= nitems as f64;
        }
    }
    #[test]
    /// Compares results with naive ones.
    fn f() -> () {
        let nitemss = [
            1, 2, 4, 8, 16, 32, 64, 128, 256, 3, 6, 12, 24, 48, 96, 192, 384, 768, 5, 10, 20, 40,
            80, 160, 320, 640, 1280,
        ];
        for nitems in nitemss {
            let input: Vec<f64> = (0..nitems).map(|x| x as f64 / nitems as f64).collect();
            let mut output0: Vec<f64> = vec![0f64; nitems];
            let mut output1: Vec<f64> = vec![0f64; nitems];
            for n in 0..nitems {
                output1[n] = input[n];
            }
            let mut plan = super::DstPlan::new(nitems);
            dst_f(nitems, &input, &mut output0);
            plan.exec_f(&mut output1);
            let mut dif: f64 = 0.;
            for n in 0..nitems {
                dif += (output0[n] - output1[n]).abs() / nitems as f64;
            }
            assert!(dif < 1e-14 * nitems as f64);
        }
    }
    #[test]
    /// Compares results with naive ones.
    fn b() -> () {
        let nitemss = [
            1, 2, 4, 8, 16, 32, 64, 128, 256, 3, 6, 12, 24, 48, 96, 192, 384, 768, 5, 10, 20, 40,
            80, 160, 320, 640, 1280,
        ];
        for nitems in nitemss {
            let input: Vec<f64> = (0..nitems).map(|x| x as f64 as f64).collect();
            let mut output0: Vec<f64> = vec![0.; nitems];
            let mut output1: Vec<f64> = vec![0.; nitems];
            let mut plan = super::DstPlan::new(nitems);
            for n in 0..nitems {
                output1[n] = input[n];
            }
            dst_b(nitems, &input, &mut output0);
            plan.exec_b(&mut output1);
            let mut dif: f64 = 0.;
            for n in 0..nitems {
                dif += (output0[n] - output1[n]).abs() / nitems as f64;
            }
            assert!(dif < 1e-14 * nitems as f64);
        }
    }
}
