#include <stdio.h>
#include <stdlib.h>
#include "memory.h"

void * memory_alloc (
    const size_t nitems,
    const size_t size
) {
  void * const ptr = malloc(nitems * size);
  if (NULL == ptr) {
    puts("failed to allocate");
    exit(EXIT_FAILURE);
  }
  return ptr;
}

void memory_free (
    void * const ptr
) {
  free(ptr);
}

