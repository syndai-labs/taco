#include "taco_engines.h"

#include <cstdio>

int main() {
    int gpu = taco_engines_linked();
    std::printf("taco_engines_linked gpu_offload=%d\n", gpu);
    return gpu == 1 ? 0 : 2;
}
