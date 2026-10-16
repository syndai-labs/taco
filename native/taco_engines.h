#pragma once

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* 1 when llama.cpp reports GPU offload support in this Mach-O. */
int taco_engines_linked(void);

/* role: 0 vad, 1 asr, 2 mt. 0 = ok. */
int taco_engines_init(int role, const char * path);
int taco_engines_infer(int role, const void * data, size_t len, int * out_n);
int taco_engines_cancel(int role, int reserve_ms, int * join_ms);
void taco_engines_close(int role);

typedef struct taco_engines_proof {
    int32_t linked;
    int32_t vad_whisper;
    int32_t vad_ort;
    int32_t asr_ok;
    int32_t mt_ok;
    int32_t gpu_offload;
    int32_t off_n;
    int32_t off_d;
    int32_t asr_text_bytes;
    int32_t mt_text_bytes;
    int32_t whisper_init_join_ms;
    int32_t whisper_decode_join_ms;
    int32_t mt_init_join_ms;
    int32_t mt_decode_join_ms;
    int32_t whisper_init_within;
    int32_t whisper_decode_within;
    int32_t mt_init_within;
    int32_t mt_decode_within;
    int32_t link_three;
    int32_t cancel_lifecycle;
    int32_t teardown;
    char gpu_device[64];
} taco_engines_proof;

/* Load, infer, and cancel the three pins in this process. 0 = both predicates. */
int taco_engines_prove(
    const char * vad,
    const char * asr,
    const char * mt,
    int reserve_ms,
    taco_engines_proof * out);

/* Instantaneous task_vm_info sample. phys_footprint is the catalog metric.
   graphics_* is the kernel graphics ledger, not an MTLBuffer inventory.
   ok is 1 when phys_footprint was read. graphics_ok is 1 when rev3 ledgers were read. */
typedef struct taco_vm_sample {
    uint64_t phys_footprint;
    uint64_t resident_size;
    uint64_t device;
    int64_t ledger_phys_footprint_peak;
    int64_t graphics_footprint;
    int64_t graphics_nofootprint;
    uint64_t malloc_size_in_use;
    uint64_t malloc_max_size_in_use;
    int32_t ok;
    int32_t graphics_ok;
} taco_vm_sample;

void taco_engines_vm_sample(taco_vm_sample * out);

#ifdef __cplusplus
}
#endif
