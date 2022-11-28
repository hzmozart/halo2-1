#include <stdio.h>

__global__ void hello(char *a, char *b) {
  printf("hehhe");
  a[0] = b[0];
}

extern "C" {
	int helloMain(char *a, char *b) {
		hello<<<1,1>>>(a,b);
		return 0;
	}
}
