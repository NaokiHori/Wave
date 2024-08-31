use super::converter;
use super::coordinate;
use super::Config;
use super::NDIMS;

fn init_pos(config: &Config, converter: &mut converter::Converter, freq: &mut [f64]) {
    let nitems: &[usize; NDIMS] = &config.nitems;
    let mut phys = Vec::with_capacity(nitems[0] * nitems[1]);
    for _j in 0..nitems[1] {
        for _i in 0..nitems[0] {
            phys.push(0f64);
        }
    }
    converter.phys_to_freq(&phys, freq);
}

fn init_vel(config: &Config, converter: &mut converter::Converter, freq: &mut [f64]) {
    let nitems: &[usize; NDIMS] = &config.nitems;
    let mut phys = Vec::with_capacity(nitems[0] * nitems[1]);
    for _j in 0..nitems[1] {
        for _i in 0..nitems[0] {
            phys.push(0f64);
        }
    }
    converter.phys_to_freq(&phys, freq);
}

fn init_acc(config: &Config, converter: &mut converter::Converter, freq: &mut [f64]) {
    let nitems: &[usize; NDIMS] = &config.nitems;
    let lengths: &[f64; NDIMS] = &config.lengths;
    let center: [f64; NDIMS] = [0.5f64 * lengths[0], 0.5f64 * lengths[1]];
    let mut phys = Vec::with_capacity(nitems[0] * nitems[1]);
    // gaussian, standard deviation^2
    let sigma = 1e-2f64;
    let sigma2 = sigma.powi(2);
    for j in 0..nitems[1] {
        for i in 0..nitems[0] {
            let x: f64 = coordinate::get_coordinate(lengths[0], nitems[0], i);
            let y: f64 = coordinate::get_coordinate(lengths[1], nitems[1], j);
            let dx: f64 = x - center[0];
            let dy: f64 = y - center[1];
            let d2: f64 = dx.powi(2) + dy.powi(2);
            let value: f64 = (0f64 - d2 / 2f64 / sigma2).exp();
            let value: f64 = value * (std::f64::consts::PI * x / lengths[0]).sin();
            let value: f64 = value * (std::f64::consts::PI * y / lengths[1]).sin();
            phys.push(value);
        }
    }
    converter.phys_to_freq(&phys, freq);
}

pub fn init_field(
    config: &Config,
    converter: &mut converter::Converter,
    pos_freq: &mut [f64],
    vel_freq: &mut [f64],
    acc_freq: &mut [f64],
) {
    init_pos(config, converter, pos_freq);
    init_vel(config, converter, vel_freq);
    init_acc(config, converter, acc_freq);
}
