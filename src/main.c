#include <fftw3.h>
#include "config.h"
#include "init.h"
#include "integrate.h"
#include "memory.h"

#if NDIMS != 1 && NDIMS != 2
#error "not implemented"
#endif

int main (
    void
) {
  const config_t config = {
#if NDIMS == 1
    .lengths[0] = 1.,
    .nitems[0] = 127,
#else
    .lengths[0] = 2.,
    .lengths[1] = 1.,
    .nitems[0] = 255,
    .nitems[1] = 127,
#endif
    .time_max = 2.,
    .param = {
      .c2 = 0.25,
      .nu = 1.e-3,
    },
    .frequency = {
      .out = 0.05,
      .log = 0.01,
    },
  };
  fftw_plan fftw_plans[NDIMS] = {
    NULL,
#if NDIMS == 2
    NULL,
#endif
  };
  for (size_t dim = 0; dim < NDIMS; dim++) {
    fftw_plans[dim] = fftw_plan_r2r_1d(config.nitems[dim], NULL, NULL, FFTW_RODFT00, FFTW_ESTIMATE);
  }
  size_t nitems_total = 0;
  get_nitems_total(&config, &nitems_total);
  double * const pos = memory_alloc(nitems_total, sizeof(double));
  double * const vel = memory_alloc(nitems_total, sizeof(double));
  double * const acc = memory_alloc(nitems_total, sizeof(double));
  init_field(&config, fftw_plans, pos, vel, acc);
  integrate(&config, fftw_plans, pos, vel, acc);
  memory_free(pos);
  memory_free(vel);
  memory_free(acc);
  for (size_t dim = 0; dim < NDIMS; dim++) {
    fftw_destroy_plan(fftw_plans[dim]);
  }
  return 0;
}

