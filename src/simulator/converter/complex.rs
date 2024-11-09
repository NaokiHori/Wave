#[derive(Copy, Clone)]
pub struct Complex {
    /// Real part of a complex number.
    pub real: f64,
    /// Imaginary part of a complex number.
    pub imag: f64,
}

/// Imaginary unit.
pub const I: Complex = Complex {
    real: 0f64,
    imag: 1f64,
};

impl Complex {
    /// Returns the complex-conjugate.
    pub fn conj(self) -> Complex {
        Complex {
            real: 0f64 + self.real,
            imag: 0f64 - self.imag,
        }
    }
}

impl core::ops::Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Complex {
        Complex {
            real: 0f64 - self.real,
            imag: 0f64 - self.imag,
        }
    }
}

impl core::ops::Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl core::ops::Sub for Complex {
    type Output = Complex;
    fn sub(self, other: Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl core::ops::Mul<Complex> for Complex {
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex {
        let r0: f64 = self.real;
        let i0: f64 = self.imag;
        let r1: f64 = other.real;
        let i1: f64 = other.imag;
        let r0r1: f64 = r0 * r1;
        let i0i1: f64 = i0 * i1;
        let prod: f64 = (r0 + i0) * (r1 + i1);
        Complex {
            real: r0r1 - i0i1,
            imag: prod - r0r1 - i0i1,
        }
    }
}

impl core::ops::Mul<Complex> for f64 {
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex {
        Complex {
            real: self * other.real,
            imag: self * other.imag,
        }
    }
}

impl core::ops::Mul<f64> for Complex {
    type Output = Complex;
    fn mul(self, other: f64) -> Complex {
        Complex {
            real: self.real * other,
            imag: self.imag * other,
        }
    }
}

#[cfg(test)]
extern crate rand;

#[cfg(test)]
mod test {
    use super::Complex;
    use rand::Rng;
    const SMALL: f64 = 1e-14f64;

    #[test]
    fn conj() {
        let mut rng = rand::thread_rng();
        let complex = Complex {
            real: rng.gen::<f64>(),
            imag: rng.gen::<f64>(),
        };
        let result: Complex = complex.conj();
        assert_eq!(result.real, 0f64 + complex.real);
        assert_eq!(result.imag, 0f64 - complex.imag);
    }

    #[test]
    fn neg() {
        let mut rng = rand::thread_rng();
        let complex = Complex {
            real: rng.gen::<f64>(),
            imag: rng.gen::<f64>(),
        };
        let result: Complex = -complex;
        assert_eq!(result.real, 0f64 - complex.real);
        assert_eq!(result.imag, 0f64 - complex.imag);
    }

    #[test]
    fn add() {
        let mut rng = rand::thread_rng();
        let complex_0 = Complex {
            real: rng.gen::<f64>(),
            imag: rng.gen::<f64>(),
        };
        let complex_1 = Complex {
            real: rng.gen::<f64>(),
            imag: rng.gen::<f64>(),
        };
        let result: Complex = complex_0 + complex_1;
        assert_eq!(result.real, complex_0.real + complex_1.real);
        assert_eq!(result.imag, complex_0.imag + complex_1.imag);
    }

    #[test]
    fn sub() {
        let mut rng = rand::thread_rng();
        let complex_0 = Complex {
            real: rng.gen::<f64>(),
            imag: rng.gen::<f64>(),
        };
        let complex_1 = Complex {
            real: rng.gen::<f64>(),
            imag: rng.gen::<f64>(),
        };
        let result: Complex = complex_0 - complex_1;
        assert_eq!(result.real, complex_0.real - complex_1.real);
        assert_eq!(result.imag, complex_0.imag - complex_1.imag);
    }

    #[test]
    fn mul_complex_complex() {
        let mut rng = rand::thread_rng();
        let complex_0 = Complex {
            real: rng.gen::<f64>(),
            imag: rng.gen::<f64>(),
        };
        let complex_1 = Complex {
            real: rng.gen::<f64>(),
            imag: rng.gen::<f64>(),
        };
        let result: Complex = complex_0 * complex_1;
        assert!(
            (complex_0.real * complex_1.real - complex_0.imag * complex_1.imag - result.real).abs()
                < SMALL
        );
        assert!(
            (complex_0.real * complex_1.imag + complex_0.imag * complex_1.real - result.imag).abs()
                < SMALL
        );
    }

    #[test]
    fn mul_complex_real() {
        let mut rng = rand::thread_rng();
        let complex = Complex {
            real: rng.gen::<f64>(),
            imag: rng.gen::<f64>(),
        };
        let real = rng.gen::<f64>();
        let result: Complex = complex * real;
        assert_eq!(result.real, complex.real * real);
        assert_eq!(result.imag, complex.imag * real);
    }

    #[test]
    fn mul_real_complex() {
        let mut rng = rand::thread_rng();
        let real = rng.gen::<f64>();
        let complex = Complex {
            real: rng.gen::<f64>(),
            imag: rng.gen::<f64>(),
        };
        let result: Complex = real * complex;
        assert_eq!(result.real, real * complex.real);
        assert_eq!(result.imag, real * complex.imag);
    }
}
