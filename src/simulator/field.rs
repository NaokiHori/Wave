use super::converter::Converter;
use super::coordinate::get_coordinate;
use super::Config;
use super::NDIMS;

pub struct Field {
    pub pos: Vec<f64>,
    pub vel: Vec<f64>,
    pub acc: Vec<f64>,
    pub buf: Vec<f64>,
}

impl Field {
    pub fn new(config: &Config, converter: &mut Converter) -> Self {
        let nitems_total: usize = config.nitems[0] * config.nitems[1];
        let mut pos = vec![0f64; nitems_total];
        let mut vel = vec![0f64; nitems_total];
        let mut acc = vec![0f64; nitems_total];
        let buf = vec![0f64; nitems_total];
        init_pos(config, converter, &mut pos);
        init_vel(config, converter, &mut vel);
        init_acc(config, converter, &mut acc);
        Self { pos, vel, acc, buf }
    }
}

fn gaussian(lengths: &[f64; NDIMS], sigma2: f64, point: &[f64; NDIMS]) -> f64 {
    let mut dx: f64 = point[0].abs();
    let mut dy: f64 = point[1].abs();
    dx = dx.min((point[0] - lengths[0]).abs());
    dx = dx.min((point[0] + lengths[0]).abs());
    dy = dy.min((point[1] - lengths[1]).abs());
    dy = dy.min((point[1] + lengths[1]).abs());
    let d2: f64 = dx.powi(2) + dy.powi(2);
    1f64 / (2f64 * std::f64::consts::PI * sigma2).sqrt() * (0f64 - d2 / 2f64 / sigma2).exp()
}

fn init_pos(config: &Config, converter: &mut Converter, freq: &mut [f64]) {
    let nitems: &[usize; NDIMS] = &config.nitems;
    let mut phys = Vec::with_capacity(nitems[0] * nitems[1]);
    for _j in 0..nitems[1] {
        for _i in 0..nitems[0] {
            phys.push(0f64);
        }
    }
    converter.phys_to_freq(&phys, freq);
}

fn init_vel(config: &Config, converter: &mut Converter, freq: &mut [f64]) {
    let nitems: &[usize; NDIMS] = &config.nitems;
    let mut phys = Vec::with_capacity(nitems[0] * nitems[1]);
    for _j in 0..nitems[1] {
        for _i in 0..nitems[0] {
            phys.push(0f64);
        }
    }
    converter.phys_to_freq(&phys, freq);
}

fn init_acc(config: &Config, converter: &mut Converter, freq: &mut [f64]) {
    let nitems: &[usize; NDIMS] = &config.nitems;
    let lengths: &[f64; NDIMS] = &config.lengths;
    let mut phys = Vec::with_capacity(nitems[0] * nitems[1]);
    let sigma2 = 1e-5f64;
    for j in 0..nitems[1] {
        for i in 0..nitems[0] {
            let x: f64 = get_coordinate(lengths[0], nitems[0], i);
            let y: f64 = get_coordinate(lengths[1], nitems[1], j);
            let mut value: f64 = 0f64;
            value -= gaussian(lengths, 0.5f64 * sigma2, &[x, y]);
            value += gaussian(lengths, 1.0f64 * sigma2, &[x, y]);
            phys.push(value);
        }
    }
    converter.phys_to_freq(&phys, freq);
    // enforce zero mean
    freq[0] = 0f64;
}
