#include <dlfcn.h>
#include <stddef.h>
#include <unistd.h>

static void *(*real_malloc)(size_t);

void *malloc(size_t size) {
    if (!real_malloc) {
        real_malloc = dlsym(RTLD_NEXT, "malloc");
    }

    write(1, "malloc called\n", 14);

    void *ptr = real_malloc(size);
    return ptr;
};
