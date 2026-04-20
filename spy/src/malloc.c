#include "memory_event.h"
#include <dlfcn.h>
#include <stddef.h>
#include <unistd.h>

static void *(*real_malloc)(size_t);

void *malloc(size_t size) {
    if (!real_malloc) {
        real_malloc = dlsym(RTLD_NEXT, "malloc");
    }

    void *ptr = real_malloc(size);

    memory_event_t msg =
    {

    }

    return ptr;
};
