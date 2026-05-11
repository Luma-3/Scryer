#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main(void) {
    char msg[] = "Allocation...\n";
    write(1, msg, strlen(msg));
    void *ptr = malloc(100);
    free(ptr);

    return EXIT_SUCCESS;
}
