#pragma once

#ifdef __cplusplus
extern "C" {
#endif

/* 1 when llama.cpp reports GPU offload support in this Mach-O. */
int taco_engines_linked(void);

#ifdef __cplusplus
}
#endif
