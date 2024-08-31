use super::Config;
use super::NDIMS;

fn external(time: f64, dt: f64, nk: f64, eps_dt: f64, amp: f64) -> f64 {
    let omega = 8f64 * std::f64::consts::PI;
    let t_old = time;
    let t_new = time + dt;
    let omega_t_old = omega * t_old;
    let omega_t_new = omega * t_new;
    let omega_t_old_sin = omega_t_old.sin();
    let omega_t_old_cos = omega_t_old.cos();
    let omega_t_new_sin = omega_t_new.sin();
    let omega_t_new_cos = omega_t_new.cos();
    let omega2 = omega.powi(2);
    amp * (eps_dt * (nk / omega2 * omega_t_new_sin - 1f64 / omega * omega_t_new_cos)
        - (nk / omega2 * omega_t_old_sin - 1f64 / omega * omega_t_old_cos))
        / (1f64 + nk.powi(2) / omega2)
}

pub fn integrate(
    config: &Config,
    time: f64,
    pos: &mut [f64],
    vel: &mut [f64],
    acc: &mut [f64],
    dt: &mut f64,
) {
    let nitems: &[usize; NDIMS] = &config.nitems;
    let lengths: &[f64; NDIMS] = &config.lengths;
    // decide dt
    // NOTE: maximum dt is given by the user
    {
        for dim in 0..NDIMS {
            let length: f64 = config.lengths[dim];
            let nitems: usize = config.nitems[dim];
            *dt = (*dt).min(length / nitems as f64 / config.param_c2.sqrt());
        }
    };
    // update field
    {
        let tau: f64 = 0.5f64 * *dt;
        for kx in 0..nitems[0] {
            let wx: f64 = std::f64::consts::PI * (kx + 1) as f64 / lengths[0];
            for ky in 0..nitems[1] {
                let wy: f64 = std::f64::consts::PI * (ky + 1) as f64 / lengths[1];
                let wave: f64 = wx.powi(2) + wy.powi(2);
                let nk: f64 = config.param_nu * wave;
                let ck: f64 = config.param_c2 * wave;
                // local position / velocity / acceleration
                let index: usize = kx * nitems[1] + ky;
                let l_pos: &mut f64 = &mut pos[index];
                let l_vel: &mut f64 = &mut vel[index];
                let l_acc: f64 = acc[index];
                // integrating factor
                let eps_dt: f64 = (nk * *dt).exp();
                let a: [f64; 4] = [1f64, -tau, ck * tau * eps_dt, eps_dt];
                let b: [f64; 2] = [
                    *l_pos + tau * *l_vel,
                    -ck * tau * *l_pos + *l_vel + external(time, *dt, nk, eps_dt, l_acc),
                ];
                // solve linear system
                let det: f64 = a[0] * a[3] - a[1] * a[2];
                *l_pos = 1f64 / det * (a[3] * b[0] - a[1] * b[1]);
                *l_vel = 1f64 / det * (a[0] * b[1] - a[2] * b[0]);
            }
        }
    }
}
