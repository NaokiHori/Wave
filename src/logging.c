#include <stdio.h>
#include <stdbool.h>
#include <errno.h>
#include <math.h>
#include "constant.h"
#include "logging.h"
#include "memory.h"

int logging (
    const config_t * const config,
    const size_t step,
    const double time,
    const double * const pos,
    const double * const vel
) {
  static bool first_time = true;
  const char * const mode = first_time ? "w" : "a";
  first_time = false;
  const char file_name[] = {"output/log.dat"};
  errno = 0;
  FILE * const fp = fopen(file_name, mode);
  if (NULL == fp) {
    perror(file_name);
    return 1;
  }
  const size_t * const nitems = config->nitems;
  const double * const lengths = config->lengths;
  const double c2 = config->param.c2;
  size_t nitems_total = 0;
  get_nitems_total(config, &nitems_total);
  double energy[2] = {0., 0.};
#if NDIMS == 1
  const double factor = 0.5 * lengths[0];
  for (size_t i = 0; i < nitems[0]; i++) {
    energy[0] += factor * (0.5 * pow(vel[i], 2.));
  }
  for (size_t i = 0; i < nitems[0]; i++) {
    energy[1] += factor * (0.5 * c2 * pow(pi * (i + 1) / lengths[0] * pos[i], 2.));
  }
#else
  const double factor = (0.5 * lengths[0]) * (0.5 * lengths[1]);
  for (size_t i = 0; i < nitems[0]; i++) {
    for (size_t j = 0; j < nitems[1]; j++) {
      energy[0] += factor * (0.5 * pow(vel[i * nitems[1] + j], 2.));
    }
  }
  for (size_t i = 0; i < nitems[0]; i++) {
    for (size_t j = 0; j < nitems[1]; j++) {
      energy[1] +=
        + factor * (0.5 * c2 * pow(pi * (i + 1) / lengths[0] * pos[i * nitems[1] + j], 2.))
        + factor * (0.5 * c2 * pow(pi * (j + 1) / lengths[1] * pos[i * nitems[1] + j], 2.));
    }
  }
#endif
  fprintf(fp, "% .15e % .15e % .15e % .15e\n", time, energy[0], energy[1], energy[0] + energy[1]);
  fclose(fp);
  printf("step %10zu time % .1e\n", step, time);
  return 0;
}

