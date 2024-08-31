#include <math.h>
#include <stdio.h>
#include "constant.h"
#include "integrate.h"
#include "logging.h"
#include "memory.h"
#include "output.h"

static double decide_dt (
    const config_t * const config
) {
  double dt = 1.;
  for (size_t dim = 0; dim < NDIMS; dim++) {
    dt = fmin(dt, config->lengths[dim] / config->nitems[dim]);
  }
  dt *= 0.1;
  return dt;
}

static double compute_external_force (
    const double amp,
    const double omega,
    const double time,
    const double dt
) {
  return amp * sin(omega * time) * dt;
}

static int update (
    const config_t * const config,
    double * const pos,
    double * const vel,
    double * const acc,
    const double time,
    const double dt
) {
  const size_t * const nitems = config->nitems;
  const double * const lengths = config->lengths;
  const double c2 = config->param.c2;
  const double nu = config->param.nu;
  const double tau = 0.5 * dt;
  const size_t nitems_total = (
      nitems[0]
#if 1 < NDIMS
      *
      nitems[1]
#endif
  );
  for (size_t n = 0; n < nitems_total; n++) {
#if NDIMS == 1
    const size_t i = n;
#else
    const size_t i = n / nitems[1];
    const size_t j = n % nitems[1];
#endif
    const double wave = (
        + pow(pi * (i + 1) / lengths[0], 2.)
#if 1 < NDIMS
        + pow(pi * (j + 1) / lengths[1], 2.)
#endif
    );
    double * const l_pos = pos + n;
    double * const l_vel = vel + n;
    // integrating factor
    const double eps = exp(nu * wave * dt);
    // left-hand-side matrix
    const double a[2][2] = {
      {
        + 1.,
        - tau,
      },
      {
        + c2 * wave * tau * eps,
        + eps,
      },
    };
    // right-hand-side vector
    const double b[2] = {
      (
        + *l_pos
        + tau * *l_vel
      ),
      (
        - c2 * wave * tau * *l_pos
        + *l_vel
        + compute_external_force(acc[n], 8. * pi, time, dt)
      ),
    };
    // solve 2 by 2 linear system to update position and velocity
    const double det = a[0][0] * a[1][1] - a[0][1] * a[1][0];
    *l_pos = 1. / det * (+ a[1][1] * b[0] - a[0][1] * b[1]);
    *l_vel = 1. / det * (- a[1][0] * b[0] + a[0][0] * b[1]);
  }
  return 0;
}

int integrate (
    const config_t * const config,
    fftw_plan fftw_plans[NDIMS],
    double * const pos,
    double * const vel,
    double * const acc
) {
  const double dt = decide_dt(config);
  double out_next = config->frequency.out;
  double log_next = config->frequency.log;
  size_t step = 0;
  double time = 0.;
  logging(config, step, time, pos, vel);
  output_field(config, fftw_plans, pos, vel);
  output_coordinate(config);
  // main iteration in time
  for (;;) {
    update(config, pos, vel, acc, time, dt);
    time = time + dt;
    step = step + 1;
    if (config->time_max < time) {
      break;
    }
    if (log_next < time) {
      logging(config, step, time, pos, vel);
      log_next = log_next + config->frequency.log;
    }
    if (out_next < time) {
      output_field(config, fftw_plans, pos, vel);
      out_next = out_next + config->frequency.out;
    }
  }
  return 0;
}

