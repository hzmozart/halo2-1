#include <stdio.h>

__global__ void hello(char *a, char *b) {
  printf("hehhe");
  a[0] = b[0];
}
