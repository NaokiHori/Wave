#if !defined(CONFIG_H)
#define CONFIG_H

#include <stddef.h> // size_t

typedef struct {
  // domain size
  double lengths[NDIMS];
  // resolution
  size_t nitems[NDIMS];
  // integrate up to
  double time_max;
  // physical properties
  struct {
    double c2;
    double nu;
  } param;
  // output flow field / check statistics per
  struct {
    double out;
    double log;
  } frequency;
} config_t;

extern int get_nitems_total (
    const config_t * const config,
    size_t * const nitems_total
);

#endif // CONFIG_H
