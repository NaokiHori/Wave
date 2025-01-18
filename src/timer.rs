pub struct Timer {
    next: f32,
    freq: f32,
}

impl Timer {
    pub fn new(freq: f32) -> Self {
        let next: f32 = freq;
        Self { next, freq }
    }

    pub fn next(&self) -> f32 {
        self.next
    }

    pub fn update(&mut self) {
        self.next += self.freq;
    }
}
