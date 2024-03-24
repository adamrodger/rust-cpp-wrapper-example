#ifndef WRAPPER_H
#define WRAPPER_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// wrapper definition does here
void* CreateInstance(unsigned int initialValue);
void ReleaseInstance(void* instance);

int Increment(void* instance, unsigned int amount);
int Decrement(void* instance, unsigned int amount);

unsigned int GetCurrentValue(const void* instance);
unsigned int* GetHistory(const void* instance, size_t* length);

#ifdef __cplusplus
} // extern "C"
#endif

#endif
