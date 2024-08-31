#include "convert.h"
#include "memory.h"

#if NDIMS == 2
static int transpose (
    const size_t nx,
    const size_t ny,
    const double * const bef,
    double * const aft
) {
  for (size_t j = 0; j < ny; j++) {
    for (size_t i = 0; i < nx; i++) {
      aft[i * ny + j] = bef[j * nx + i];
    }
  }
  return 0;
}
#endif

int convert_from_phys_to_spec (
    const config_t * const config,
    fftw_plan fftw_plans[NDIMS],
    const double * const phys,
    double * const spec
) {
  const size_t * const nitems = config->nitems;
  size_t nitems_total = 0;
  get_nitems_total(config, &nitems_total);
#if NDIMS == 1
  for (size_t n = 0; n < nitems_total; n++) {
    spec[n] = phys[n] / (2. * (nitems[0] - 1));
  }
  fftw_execute_r2r(fftw_plans[0], spec, spec);
#else
  double * const spec_x = memory_alloc(nitems_total, sizeof(double));
  for (size_t n = 0; n < nitems_total; n++) {
    spec_x[n] = phys[n] / (2. * (nitems[0] - 1) * 2. * (nitems[1] - 1));
  }
  for (size_t j = 0; j < nitems[1]; j++) {
    double * const p = spec_x + j * nitems[0];
    fftw_execute_r2r(fftw_plans[0], p, p);
  }
  transpose(nitems[0], nitems[1], spec_x, spec);
  for (size_t i = 0; i < nitems[0]; i++) {
    double * const p = spec + i * nitems[1];
    fftw_execute_r2r(fftw_plans[1], p, p);
  }
  memory_free(spec_x);
#endif
  return 0;
}

int convert_from_spec_to_phys (
    const config_t * const config,
    fftw_plan fftw_plans[NDIMS],
    const double * const spec,
    double * const phys
) {
  size_t nitems_total = 0;
  get_nitems_total(config, &nitems_total);
#if NDIMS == 1
  for (size_t n = 0; n < nitems_total; n++) {
    phys[n] = spec[n];
  }
  fftw_execute_r2r(fftw_plans[0], phys, phys);
#else
  const size_t * const nitems = config->nitems;
  double * const phys_y = memory_alloc(nitems_total, sizeof(double));
  for (size_t n = 0; n < nitems_total; n++) {
    phys_y[n] = spec[n];
  }
  for (size_t i = 0; i < nitems[0]; i++) {
    double * const p = phys_y + i * nitems[1];
    fftw_execute_r2r(fftw_plans[1], p, p);
  }
  transpose(nitems[1], nitems[0], phys_y, phys);
  for (size_t j = 0; j < nitems[1]; j++) {
    double * const p = phys + j * nitems[0];
    fftw_execute_r2r(fftw_plans[0], p, p);
  }
  memory_free(phys_y);
#endif
  return 0;
}

