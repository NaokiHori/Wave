#if !defined(OUTPUT_H)
#define OUTPUT_H

#include <fftw3.h> // fftw_plan
#include "config.h" // config_t

extern int output_coordinate (
    const config_t * const config
);

extern int output_field (
    const config_t * const config,
    fftw_plan fftw_plans[NDIMS],
    const double * const pos,
    const double * const vel
);

#endif // OUTPUT_H
