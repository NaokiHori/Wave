#include "config.h"

int get_nitems_total (
    const config_t * const config,
    size_t * const nitems_total
) {
  const size_t * const nitems = config->nitems;
  *nitems_total = 1;
  for (size_t dim = 0; dim < NDIMS; dim++) {
    *nitems_total *= nitems[dim];
  }
  return 0;
}

