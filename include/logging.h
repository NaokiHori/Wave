#if !defined(LOGGING_H)
#define LOGGING_H

#include <stddef.h> // size_t
#include "config.h" // config_t

extern int logging (
    const config_t * const config,
    const size_t step,
    const double time,
    const double * const pos,
    const double * const vel
);

#endif // LOGGING_H
