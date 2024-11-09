use super::field::Field;
use super::source::Source;
use super::Config;
use super::NDIMS;
use crate::random::Random;
use std::f64::consts::PI;

fn get_wavenumber(length: f64, nitems: usize, index: usize) -> f64 {
    let k: i32 = if index < nitems / 2 {
        index as i32
    } else {
        index as i32 - nitems as i32
    };
    2f64 * PI * k as f64 / length
}

fn compute_acc(config: &Config, source: &Source, base: &[f64], acc: &mut [f64]) {
    let nitems: &[usize; NDIMS] = &config.nitems;
    let lengths: &[f64; NDIMS] = &config.lengths;
    for kx in 0..nitems[0] {
        for ky in 0..nitems[1] {
            let index: usize = kx * nitems[1] + ky;
            acc[index] = source.amp * base[index];
        }
    }
    let source_position: [f64; NDIMS] = source.get_position();
    for kx in 0..nitems[0] / 2 + 1 {
        for ky in 0..nitems[1] {
            let phase = 0f64 - 2f64 * PI * kx as f64 * source_position[0] / lengths[0];
            let phase_r = phase.cos();
            let phase_i = phase.sin();
            let index0: usize = kx * nitems[1] + ky;
            let index1: usize = (nitems[0] - kx) * nitems[1] + ky;
            let z0_r = acc[index0];
            let z0_i = if 0 == kx || nitems[0] / 2 == kx {
                0f64
            } else {
                -acc[index1]
            };
            acc[index0] = z0_r * phase_r - z0_i * phase_i;
            if 0 != kx {
                acc[index1] = -z0_r * phase_i - z0_i * phase_r;
            }
        }
    }
    for kx in 0..nitems[0] {
        for ky in 0..nitems[1] / 2 + 1 {
            let phase = 0f64 - 2f64 * PI * ky as f64 * source_position[1] / lengths[1];
            let phase_r = phase.cos();
            let phase_i = phase.sin();
            let index0: usize = kx * nitems[1] + ky;
            let index1: usize = kx * nitems[1] + (nitems[1] - ky);
            let z0_r = acc[index0];
            let z0_i = if 0 == ky || nitems[1] / 2 == ky {
                0f64
            } else {
                -acc[index1]
            };
            acc[index0] = z0_r * phase_r - z0_i * phase_i;
            if 0 != ky {
                acc[index1] = -z0_r * phase_i - z0_i * phase_r;
            }
        }
    }
}

fn update_field(config: &Config, field: &mut Field, source: &Source, dt: f64) {
    compute_acc(config, source, &field.acc, &mut field.buf);
    let nitems: &[usize; NDIMS] = &config.nitems;
    let lengths: &[f64; NDIMS] = &config.lengths;
    let tau: f64 = 0.5f64 * dt;
    for kx in 0..nitems[0] {
        let wx: f64 = get_wavenumber(lengths[0], nitems[0], kx);
        for ky in 0..nitems[1] {
            let wy: f64 = get_wavenumber(lengths[1], nitems[1], ky);
            let w2: f64 = wx.powi(2) + wy.powi(2);
            let nk: f64 = config.param_nu * w2;
            let ck: f64 = config.param_c2 * w2;
            // integrating factor
            let eps_dt: f64 = (nk * dt).exp();
            // local position / velocity / acceleration
            let index: usize = kx * nitems[1] + ky;
            let l_pos: &mut f64 = &mut field.pos[index];
            let l_vel: &mut f64 = &mut field.vel[index];
            let l_acc: f64 = field.buf[index] * dt;
            let a: [f64; 4] = {
                let a0 = 1f64;
                let a1 = 0f64 - tau;
                let a2 = ck * tau * eps_dt;
                let a3 = eps_dt;
                [a0, a1, a2, a3]
            };
            let b: [f64; 2] = {
                let mut b0 = 0f64;
                b0 += *l_pos;
                b0 += tau * *l_vel;
                let mut b1 = 0f64;
                b1 -= ck * tau * *l_pos;
                b1 += *l_vel;
                b1 += l_acc;
                [b0, b1]
            };
            // solve linear system
            let det: f64 = a[0] * a[3] - a[1] * a[2];
            *l_pos = 1f64 / det * (a[3] * b[0] - a[1] * b[1]);
            *l_vel = 1f64 / det * (a[0] * b[1] - a[2] * b[0]);
        }
    }
}

pub fn integrate(
    config: &Config,
    rng: &mut Random,
    field: &mut Field,
    source: &mut Source,
    dt: f64,
) {
    source.update(rng, dt);
    update_field(config, field, source, dt);
}
