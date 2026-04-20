#ifndef _MEMEVENT_H
#define _MEMEVENT_H

#include <stdint.h>
#include <unistd.h>

typedef struct {
    uint8_t type;
    uintptr_t addr;
    size_t size;
} memory_event_t;

#endif // !_MEMEVENT_H
