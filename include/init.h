#if !defined(INIT_H)
#define INIT_H

#include <fftw3.h> // fftw_plan
#include "config.h" // config_t

extern int init_field (
    const config_t * const config,
    fftw_plan fftw_plans[NDIMS],
    double * const spec_pos,
    double * const spec_vel,
    double * const spec_acc
);

#endif // INIT_H
