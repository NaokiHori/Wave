#if !defined(CONVERT_H)
#define CONVERT_H

#include <fftw3.h> // fftw_plan
#include "config.h" // config_t

extern int convert_from_phys_to_spec (
    const config_t * const config,
    fftw_plan fftw_plans[NDIMS],
    const double * const phys,
    double * const spec
);

extern int convert_from_spec_to_phys (
    const config_t * const config,
    fftw_plan fftw_plans[NDIMS],
    const double * const spec,
    double * const phys
);

#endif // CONVERT_H
