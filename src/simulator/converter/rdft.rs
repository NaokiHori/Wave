use std::f32::consts::PI;

use super::complex::{Complex, I};

pub struct RdftPlan {
    /// Size of the input / output signals
    nitems: usize,
    /// Trigonometric table
    table: Vec<Complex>,
    /// Internal buffer
    buf: Vec<f32>,
}

impl RdftPlan {
    pub fn new(nitems: usize) -> Result<Self, String> {
        if 0 != nitems % 2 {
            return Err(format!("nitems ({}) should be even", nitems));
        }
        // prepare a trigonometric table
        let mut table = Vec::<Complex>::with_capacity(nitems / 2);
        for k in 0..nitems / 2 {
            let phase: f32 = 2f32 * PI * k as f32 / nitems as f32;
            table.push(Complex {
                real: phase.cos(),
                imag: phase.sin(),
            });
        }
        let buf = vec![0f32; nitems];
        Ok(Self { nitems, table, buf })
    }

    pub fn exec_f(&mut self, xs: &mut [f32]) {
        let nitems: usize = self.nitems;
        let nitems_sub: usize = nitems / 2;
        let trigs: &[Complex] = &self.table;
        let zs: &mut [f32] = &mut self.buf;
        // compute complex dft to find Z_k
        dft(nitems_sub, 1, trigs, xs, zs);
        // compute FFT of the original real signal
        xs[0] = zs[0] + zs[1];
        xs[nitems_sub] = zs[0] - zs[1];
        for k in 1..nitems_sub {
            // 1 / 2 Z_k
            let z0 = Complex {
                real: 0.5f32 * zs[2 * k],
                imag: 0.5f32 * zs[2 * k + 1],
            };
            // 1 / 2 Z_{N / 2 - k}
            let z1 = Complex {
                real: 0.5f32 * zs[2 * (nitems_sub - k)],
                imag: 0.5f32 * zs[2 * (nitems_sub - k) + 1],
            };
            // X_k^e = + 1/2 Z_k + 1/2 Z_{N / 2 - k}^*
            let xe: Complex = z0 + z1.conj();
            // X_k^o = - I/2 Z_k + I/2 Z_{N / 2 - k}^*
            let xo: Complex = -I * z0 + I * z1.conj();
            // X_k^e + exp(- 2 pi k / nitems) X_k^o
            // X_k^e - exp(- 2 pi k / nitems) X_k^o
            let twiddle = trigs[k].conj();
            let za: Complex = xe + xo * twiddle;
            let zb: Complex = xe - xo * twiddle;
            // real and imaginary parts should be stored separately
            xs[k] = za.real;
            xs[k + nitems_sub] = zb.imag;
        }
    }

    pub fn exec_b(&mut self, xs: &mut [f32]) {
        let nitems: usize = self.nitems;
        let nitems_sub: usize = nitems / 2;
        let trigs: &[Complex] = &self.table;
        let zs: &mut [f32] = &mut self.buf;
        // create a complex signal
        (zs[0], zs[1]) = {
            let x0: f32 = 0.5f32 * xs[0];
            let x1: f32 = 0.5f32 * xs[nitems_sub];
            (x0 + x1, x0 - x1)
        };
        for k in 1..nitems_sub {
            let twiddle: Complex = trigs[k];
            // 1/2 X_k
            let x0 = Complex {
                real: 0f32 + 0.5f32 * xs[k],
                imag: 0f32 - 0.5f32 * xs[nitems - k],
            };
            // 1/2 X_{k + N / 2}
            let x1 = Complex {
                real: 0f32 + 0.5f32 * xs[nitems_sub - k],
                imag: 0f32 + 0.5f32 * xs[nitems_sub + k],
            };
            // X_k^e = (+ 1/2 X_k + 1/2 X_{k + N / 2})
            let xe: Complex = x0 + x1;
            // X_k^o = (+ 1/2 X_k - 1/2 X_{k + N / 2}) * exp(arg)
            let xo: Complex = (x0 - x1) * twiddle;
            // Z_k = X_k^e + X_k^o * I
            let z: Complex = xe + xo * I;
            // real and imaginary parts should be stored alternately
            zs[2 * k] = z.real;
            zs[2 * k + 1] = z.imag;
        }
        // compute complex idft to find z_n
        idft(nitems_sub, 1, trigs, zs, xs);
    }
}

fn dft(nitems: usize, stride: usize, trigs: &[Complex], xs: &[f32], ys: &mut [f32]) {
    if 1 == nitems {
        ys[0] = xs[0];
        ys[1] = xs[1];
    } else if 0 == nitems % 2 {
        let nitems_sub: usize = nitems / 2;
        {
            let x_index: usize = 0;
            let y_index: usize = 0;
            dft(
                nitems_sub,
                stride * 2,
                trigs,
                &xs[x_index..],
                &mut ys[y_index..],
            );
        }
        {
            let x_index: usize = 2 * stride;
            let y_index: usize = nitems;
            dft(
                nitems_sub,
                stride * 2,
                trigs,
                &xs[x_index..],
                &mut ys[y_index..],
            );
        }
        for k in 0..nitems_sub {
            let twiddle = trigs[2 * stride * k].conj();
            let index_0: usize = 2 * (k);
            let index_1: usize = 2 * (k + nitems_sub);
            let e = Complex {
                real: ys[index_0],
                imag: ys[index_0 + 1],
            };
            let o = Complex {
                real: ys[index_1],
                imag: ys[index_1 + 1],
            };
            let o: Complex = o * twiddle;
            let y0 = e + o;
            let y1 = e - o;
            ys[index_0] = y0.real;
            ys[index_0 + 1] = y0.imag;
            ys[index_1] = y1.real;
            ys[index_1 + 1] = y1.imag;
        }
    } else {
        // naive O(N^2) DFT
        for k in 0..nitems {
            let y_ref: &mut [f32] = &mut ys[2 * k..];
            y_ref[0] = 0f32;
            y_ref[1] = 0f32;
            for n in 0..nitems {
                let arg: f32 = -2f32 * PI * n as f32 * k as f32 / nitems as f32;
                let twiddle = Complex {
                    real: arg.cos(),
                    imag: arg.sin(),
                };
                let x = Complex {
                    real: xs[2 * stride * n],
                    imag: xs[2 * stride * n + 1],
                };
                let y = x * twiddle;
                y_ref[0] += y.real;
                y_ref[1] += y.imag;
            }
        }
    }
}

fn idft(nitems: usize, stride: usize, trigs: &[Complex], xs: &[f32], ys: &mut [f32]) {
    if 1 == nitems {
        ys[0] = xs[0];
        ys[1] = xs[1];
    } else if 0 == nitems % 2 {
        let nitems_sub: usize = nitems / 2;
        {
            let x_index: usize = 0;
            let y_index: usize = 0;
            idft(
                nitems_sub,
                stride * 2,
                trigs,
                &xs[x_index..],
                &mut ys[y_index..],
            );
        }
        {
            let x_index: usize = 2 * stride;
            let y_index: usize = nitems;
            idft(
                nitems_sub,
                stride * 2,
                trigs,
                &xs[x_index..],
                &mut ys[y_index..],
            );
        }
        for k in 0..nitems_sub {
            let twiddle = trigs[2 * stride * k];
            let index_0 = 2 * k;
            let index_1 = 2 * (k + nitems_sub);
            let e: Complex = Complex {
                real: 0.5f32 * ys[index_0],
                imag: 0.5f32 * ys[index_0 + 1],
            };
            let o: Complex = Complex {
                real: 0.5f32 * ys[index_1],
                imag: 0.5f32 * ys[index_1 + 1],
            };
            let o: Complex = o * twiddle;
            let y0: Complex = e + o;
            let y1: Complex = e - o;
            ys[index_0] = y0.real;
            ys[index_0 + 1] = y0.imag;
            ys[index_1] = y1.real;
            ys[index_1 + 1] = y1.imag;
        }
    } else {
        // naive O(N^2) iDFT
        for k in 0..nitems {
            let index: usize = 2 * k;
            ys[index] = 0f32;
            ys[index + 1] = 0f32;
            for n in 0..nitems {
                let arg: f32 = 2f32 * PI * n as f32 * k as f32 / nitems as f32;
                let twiddle = Complex {
                    real: arg.cos(),
                    imag: arg.sin(),
                };
                let x = Complex {
                    real: xs[2 * stride * n],
                    imag: xs[2 * stride * n + 1],
                };
                let y: Complex = x * twiddle;
                ys[index] += y.real;
                ys[index + 1] += y.imag;
            }
            ys[index] /= nitems as f32;
            ys[index + 1] /= nitems as f32;
        }
    }
}

#[cfg(test)]
extern crate rand;

#[cfg(test)]
mod test {
    use crate::simulator::converter::complex::Complex;
    use rand::Rng;
    use std::f32::consts::PI;

    fn dft(nitems: usize, xs: &[Complex], ys: &mut [Complex]) {
        for k in 0..nitems {
            ys[k] = Complex {
                real: 0f32,
                imag: 0f32,
            };
            for n in 0..nitems {
                let arg: f32 = -2f32 * PI * n as f32 * k as f32 / nitems as f32;
                let twiddle = Complex {
                    real: arg.cos(),
                    imag: arg.sin(),
                };
                ys[k] = ys[k] + xs[n] * twiddle;
            }
        }
    }

    fn idft(nitems: usize, xs: &[Complex], ys: &mut [Complex]) {
        for n in 0..nitems {
            ys[n] = Complex {
                real: 0f32,
                imag: 0f32,
            };
            for k in 0..nitems {
                let arg: f32 = 2f32 * PI * k as f32 * n as f32 / nitems as f32;
                let twiddle = Complex {
                    real: arg.cos(),
                    imag: arg.sin(),
                };
                ys[n] = ys[n] + xs[k] * twiddle;
            }
            ys[n] = ys[n] * (1f32 / nitems as f32)
        }
    }

    #[test]
    fn f() {
        let nitemss = [
            2, 4, 8, 16, 32, 64, 128, 256, 6, 12, 24, 48, 96, 192, 384, 768, 10, 20, 40, 80, 160,
            320, 640, 1280,
        ];
        for nitems in nitemss {
            // shared input
            let input: Vec<f32> = {
                let mut rng = rand::thread_rng();
                (0..nitems).map(|_| rng.gen::<f32>() - 0.5f32).collect()
            };
            // naive complex DFT as a reference
            let output0: Vec<Complex> = {
                let mut input_complex = Vec::<Complex>::with_capacity(nitems);
                for &item in input.iter() {
                    input_complex.push(Complex {
                        real: item,
                        imag: 0f32,
                    });
                }
                let mut output: Vec<Complex> = vec![
                    Complex {
                        real: 0f32,
                        imag: 0f32
                    };
                    nitems
                ];
                dft(nitems, &input_complex, &mut output);
                output
            };
            // real-valued DFT to be tested
            let output1: Vec<f32> = {
                let mut output: Vec<f32> = vec![0f32; nitems];
                let mut plan = super::RdftPlan::new(nitems).unwrap();
                for n in 0..nitems {
                    output[n] = input[n];
                }
                plan.exec_f(&mut output);
                output
            };
            // check L^1 norm
            let dif: f32 = {
                let mut dif: f32 = 0f32;
                for n in 0..nitems / 2 + 1 {
                    dif += (output0[n].real - output1[n]).abs() / nitems as f32;
                }
                for n in nitems / 2 + 1..nitems {
                    dif += (output0[n].imag - output1[n]).abs() / nitems as f32;
                }
                dif
            };
            assert!(dif < 1e-6f32 * nitems as f32);
        }
    }

    #[test]
    fn b() {
        let nitemss = [
            2, 4, 8, 16, 32, 64, 128, 256, 6, 12, 24, 48, 96, 192, 384, 768, 10, 20, 40, 80, 160,
            320, 640, 1280,
        ];
        for nitems in nitemss {
            // shared input
            let input: Vec<f32> = {
                // NOTE: should be Hermitian-symmetric
                let mut rng = rand::thread_rng();
                let seq: Vec<f32> = (0..nitems).map(|_| rng.gen::<f32>() - 0.5f32).collect();
                seq
            };
            // naive complex iDFT as a reference
            let output0: Vec<Complex> = {
                let mut input_complex = vec![
                    Complex {
                        real: 0f32,
                        imag: 0f32
                    };
                    nitems
                ];
                for k in 0..nitems / 2 + 1 {
                    let real: f32 = input[k];
                    let imag: f32 = {
                        if 0 == k || nitems / 2 == k {
                            0f32
                        } else {
                            0f32 - input[nitems - k]
                        }
                    };
                    input_complex[k] = Complex { real, imag };
                }
                for k in 1..nitems / 2 {
                    input_complex[nitems - k] = input_complex[k].conj();
                }
                let mut output: Vec<Complex> = vec![
                    Complex {
                        real: 0f32,
                        imag: 0f32
                    };
                    nitems
                ];
                idft(nitems, &input_complex, &mut output);
                output
            };
            // real-valued iDFT to be tested
            let output1: Vec<f32> = {
                let mut output: Vec<f32> = vec![0f32; nitems];
                let mut plan = super::RdftPlan::new(nitems).unwrap();
                for n in 0..nitems {
                    output[n] = input[n];
                }
                plan.exec_b(&mut output);
                output
            };
            // check L^1 norm
            let dif: f32 = {
                let mut dif: f32 = 0f32;
                for n in 0..nitems {
                    dif += (output0[n].real - output1[n]).abs() / nitems as f32;
                    println!("{} {}", n, output0[n].real - output1[n]);
                }
                dif
            };
            assert!(dif < 1e-6f32 * nitems as f32);
        }
    }

    #[test]
    #[should_panic]
    fn odd_nitems() {
        let nitems = 1usize;
        let _plan = super::RdftPlan::new(nitems).unwrap();
    }
}
