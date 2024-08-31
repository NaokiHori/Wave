#if !defined(INTEGRATE_H)
#define INTEGRATE_H

#include <fftw3.h> // fftw_plan
#include "config.h" // config_t

extern int integrate (
    const config_t * const config,
    fftw_plan fftw_plans[NDIMS],
    double * const pos,
    double * const vel,
    double * const acc
);

#endif // INTEGRATE_H
