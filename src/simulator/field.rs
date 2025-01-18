use super::converter::Converter;
use super::coordinate::get_coordinate;
use super::Config;
use super::NDIMS;

pub struct Field {
    pub pos: Vec<f32>,
    pub vel: Vec<f32>,
    pub acc: Vec<f32>,
    pub pos_phys: Vec<f32>,
    pub acc_shifted: Vec<f32>,
}

impl Field {
    pub fn new(config: &Config, converter: &mut Converter) -> Self {
        let nitems_total: usize = config.nitems[0] * config.nitems[1];
        let mut pos = vec![0f32; nitems_total];
        let mut vel = vec![0f32; nitems_total];
        let mut acc = vec![0f32; nitems_total];
        let pos_phys = vec![0f32; nitems_total];
        let acc_shifted = vec![0f32; nitems_total];
        init_pos(config, converter, &mut pos);
        init_vel(config, converter, &mut vel);
        init_acc(config, converter, &mut acc);
        Self {
            pos,
            vel,
            acc,
            pos_phys,
            acc_shifted,
        }
    }
}

fn gaussian(lengths: &[f32; NDIMS], sigma2: f32, point: &[f32; NDIMS]) -> f32 {
    let mut dx: f32 = point[0].abs();
    let mut dy: f32 = point[1].abs();
    dx = dx.min((point[0] - lengths[0]).abs());
    dx = dx.min((point[0] + lengths[0]).abs());
    dy = dy.min((point[1] - lengths[1]).abs());
    dy = dy.min((point[1] + lengths[1]).abs());
    let d2: f32 = dx.powi(2) + dy.powi(2);
    1f32 / (2f32 * std::f32::consts::PI * sigma2).sqrt() * (0f32 - d2 / 2f32 / sigma2).exp()
}

fn init_pos(config: &Config, converter: &mut Converter, freq: &mut [f32]) {
    let nitems: &[usize; NDIMS] = &config.nitems;
    let mut phys = Vec::with_capacity(nitems[0] * nitems[1]);
    for _j in 0..nitems[1] {
        for _i in 0..nitems[0] {
            phys.push(0f32);
        }
    }
    converter.phys_to_freq(&phys, freq);
}

fn init_vel(config: &Config, converter: &mut Converter, freq: &mut [f32]) {
    let nitems: &[usize; NDIMS] = &config.nitems;
    let mut phys = Vec::with_capacity(nitems[0] * nitems[1]);
    for _j in 0..nitems[1] {
        for _i in 0..nitems[0] {
            phys.push(0f32);
        }
    }
    converter.phys_to_freq(&phys, freq);
}

fn init_acc(config: &Config, converter: &mut Converter, freq: &mut [f32]) {
    let nitems: &[usize; NDIMS] = &config.nitems;
    let lengths: &[f32; NDIMS] = &config.lengths;
    let mut phys = Vec::with_capacity(nitems[0] * nitems[1]);
    let sigma2 = 1e-5f32;
    for j in 0..nitems[1] {
        for i in 0..nitems[0] {
            let x: f32 = get_coordinate(lengths[0], nitems[0], i);
            let y: f32 = get_coordinate(lengths[1], nitems[1], j);
            let mut value: f32 = 0f32;
            value -= gaussian(lengths, 0.5f32 * sigma2, &[x, y]);
            value += gaussian(lengths, 1.0f32 * sigma2, &[x, y]);
            phys.push(value);
        }
    }
    converter.phys_to_freq(&phys, freq);
    // enforce zero mean
    freq[0] = 0f32;
}
