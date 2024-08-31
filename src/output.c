#include <errno.h> // errno
#include <stddef.h> // size_t
#include <stdio.h>
#include "convert.h"
#include "coordinate.h"
#include "memory.h"
#include "output.h"
#include "snpyio.h"

#define FILE_NAME_BUFFER_SIZE 128

static int save_npy (
    const config_t * const config,
    fftw_plan fftw_plans[NDIMS],
    const size_t cnt,
    const char var_name[],
    const double * const spec
) {
  const size_t * const nitems = config->nitems;
  char file_name[FILE_NAME_BUFFER_SIZE] = {'\0'};
  snprintf(file_name, FILE_NAME_BUFFER_SIZE, "output/%s%010zu.npy", var_name, cnt);
  errno = 0;
  FILE * const fp = fopen(file_name, "w");
  if (NULL == fp) {
    perror(file_name);
    return 1;
  }
#if NDIMS == 1
  snpyio_w_header(NDIMS, (size_t [NDIMS]){nitems[0]}, "'<f8'", false, fp);
#else
  snpyio_w_header(NDIMS, (size_t [NDIMS]){nitems[1], nitems[0]}, "'<f8'", false, fp);
#endif
  size_t nitems_total = 0;
  get_nitems_total(config, &nitems_total);
  double * const phys = memory_alloc(nitems_total, sizeof(double));
  convert_from_spec_to_phys(config, fftw_plans, spec, phys);
  fwrite(phys, sizeof(double), nitems_total, fp);
  memory_free(phys);
  fclose(fp);
  return 0;
}

int output_coordinate (
    const config_t * const config
) {
  const size_t * const nitems = config->nitems;
  const double * const lengths = config->lengths;
  for (size_t dim = 0; dim < NDIMS; dim++) {
    char file_name[FILE_NAME_BUFFER_SIZE] = {'\0'};
    snprintf(file_name, FILE_NAME_BUFFER_SIZE, "output/x%02zu.npy", dim);
    double * const buf = memory_alloc(nitems[dim], sizeof(double));
    for (size_t n = 0; n < nitems[dim]; n++) {
      buf[n] = get_coordinate(lengths[dim], nitems[dim], n);
    }
    FILE * const fp = fopen(file_name, "w");
    if (NULL == fp) {
      perror(file_name);
    } else {
      snpyio_w_header(1, (size_t [1]){nitems[dim]}, "'<f8'", false, fp);
      fwrite(buf, sizeof(double), nitems[dim], fp);
      memory_free(buf);
      fclose(fp);
    }
  }
  return 0;
}

int output_field (
    const config_t * const config,
    fftw_plan fftw_plans[NDIMS],
    const double * const pos,
    const double * const vel
) {
  static size_t cnt = 0;
  save_npy(config, fftw_plans, cnt, "pos", pos);
  save_npy(config, fftw_plans, cnt, "vel", vel);
  cnt = cnt + 1;
  return 0;
}

