pub struct Timer {
    next: f64,
    freq: f64,
}

impl Timer {
    pub fn new(freq: f64) -> Self {
        let next: f64 = freq;
        Self { next, freq }
    }

    pub fn next(&self) -> f64 {
        self.next
    }

    pub fn update(&mut self) {
        self.next += self.freq;
    }
}
