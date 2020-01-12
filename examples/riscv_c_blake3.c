#include <string.h>

#include "pvm.h"

int main(int argc, char* argv[]) {
  if (argc == 1) {
    return 0;
  }

  if (strcmp(argv[1], "blake3") == 0) {
    if (argc != 3) {
        return 1;
    }

    uint8_t *m = (uint8_t *)argv[2];
    uint8_t h[32];

    pvm_blake3(m, strlen(m), h, 32);
    pvm_ret(h, 32);

    return 0;
  }

  return 0;
}
