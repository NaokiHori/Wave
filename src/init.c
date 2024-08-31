#include <stddef.h> // size_t
#include <math.h>
#include "init.h"
#include "config.h"
#include "constant.h"
#include "convert.h"
#include "coordinate.h"
#include "memory.h"

int init_field (
    const config_t * const config,
    fftw_plan fftw_plans[NDIMS],
    double * const spec_pos,
    double * const spec_vel,
    double * const spec_acc
) {
  const size_t * const nitems = config->nitems;
  const double * const lengths = config->lengths;
  size_t nitems_total = 0;
  get_nitems_total(config, &nitems_total);
  double * const phys_pos = memory_alloc(nitems_total, sizeof(double));
  double * const phys_vel = memory_alloc(nitems_total, sizeof(double));
  double * const phys_acc = memory_alloc(nitems_total, sizeof(double));
  // displacements
#if NDIMS == 1
  for (size_t i = 0; i < nitems[0]; i++) {
    phys_pos[i] = 0.;
  }
#else
  for (size_t j = 0; j < nitems[1]; j++) {
    for (size_t i = 0; i < nitems[0]; i++) {
      phys_pos[j * nitems[0] + i] = 0.;
    }
  }
#endif
  convert_from_phys_to_spec(config, fftw_plans, phys_pos, spec_pos);
  // velocities
  // NOTE: enforce velocity field to be zero-mean
#if NDIMS == 1
  for (size_t i = 0; i < nitems[0]; i++) {
    phys_vel[i] = 0.;
  }
#else
  for (size_t j = 0; j < nitems[1]; j++) {
    for (size_t i = 0; i < nitems[0]; i++) {
      phys_vel[j * nitems[0] + i] = 0.;
    }
  }
#endif
  convert_from_phys_to_spec(config, fftw_plans, phys_vel, spec_vel);
  // external forcing (acceleration)
  const double amp = 10.;
#if NDIMS == 1
  const double center[NDIMS] = {
    0.45 * lengths[0],
  };
  for (size_t i = 0; i < nitems[0]; i++) {
    const double x = get_coordinate(lengths[0], nitems[0], i);
    const double d = fabs(x - center[0]);
    phys_acc[i] = amp * exp(- 2048. * pow(d, 2));
  }
#else
  const double center[NDIMS] = {
    0.45 * lengths[0],
    0.45 * lengths[1],
  };
  for (size_t j = 0; j < nitems[1]; j++) {
    for (size_t i = 0; i < nitems[0]; i++) {
      const double x = get_coordinate(lengths[0], nitems[0], i);
      const double y = get_coordinate(lengths[1], nitems[1], j);
      const double d = sqrt(
          + pow(x - center[0], 2.)
          + pow(y - center[1], 2.)
      );
      phys_acc[j * nitems[0] + i] = amp * exp(- 2048. * pow(d, 2));
    }
  }
#endif
  convert_from_phys_to_spec(config, fftw_plans, phys_acc, spec_acc);
  //
  memory_free(phys_pos);
  memory_free(phys_vel);
  memory_free(phys_acc);
  return 0;
}

