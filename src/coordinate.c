#include "coordinate.h"

double get_coordinate (
    const double length,
    const size_t npoints,
    const size_t index
) {
  return length * (index + 1.) / (npoints + 1.);
}

