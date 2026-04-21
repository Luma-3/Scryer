#include "../../commun/memory_event.h"
#include <dlfcn.h>
#include <stddef.h>
#include <sys/socket.h>
#include <unistd.h>

static void *(*real_malloc)(size_t);

extern int sock_fd;

void *malloc(size_t size) {
    if (!real_malloc) {
        real_malloc = dlsym(RTLD_NEXT, "malloc");
    }

    void *ptr = real_malloc(size);

    memory_event_t msg = {
        .type = MEMORY_EVENT_ALLOC, .size = size, .addr = (uintptr_t)ptr};

    send(sock_fd, &msg, sizeof(msg), 0);

    return ptr;
};
