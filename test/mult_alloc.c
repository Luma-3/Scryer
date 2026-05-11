#include <stdlib.h>

int main(void) {
    for (int i = 0; i < 1000; i++) {
        void *ptr = malloc(100);
        free(ptr);
    }

    return EXIT_SUCCESS;
}
