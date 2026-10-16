#include "taco_engines.h"

#include "llama.h"
#include "whisper.h"

extern "C" int taco_engines_linked(void) {
    whisper_context_params whisper_params = whisper_context_default_params();
    llama_model_params model_params = llama_model_default_params();
    (void)whisper_params;
    (void)model_params;
    return llama_supports_gpu_offload() ? 1 : 0;
}
