// Read Apple's MLComputePlan for a CoreML MLProgram cached by ORT.
// The preferred device class is the measurement. Requesting CPUAndGPU is not.

#import <CoreML/CoreML.h>
#import <Foundation/Foundation.h>

#include <cstdio>
#include <dispatch/dispatch.h>

namespace {

void tally_device(id device, const char * op, int * cpu, int * gpu, int * ane, int * other) API_AVAILABLE(macos(14.4)) {
    const char * kind = "other";
    if (device == nil) {
        kind = "none";
        *other += 1;
    } else if ([device isKindOfClass:[MLCPUComputeDevice class]]) {
        kind = "MLCPUComputeDevice";
        *cpu += 1;
    } else if ([device isKindOfClass:[MLGPUComputeDevice class]]) {
        kind = "MLGPUComputeDevice";
        *gpu += 1;
    } else if ([device isKindOfClass:[MLNeuralEngineComputeDevice class]]) {
        kind = "MLNeuralEngineComputeDevice";
        *ane += 1;
    } else {
        *other += 1;
    }
    std::printf("vad_ort_plan op=%s device=%s\n", op, kind);
}

void profile_block(MLComputePlan * plan, MLModelStructureProgramBlock * block, int * cpu, int * gpu, int * ane,
                   int * other) API_AVAILABLE(macos(14.4)) {
    for (MLModelStructureProgramOperation * operation in block.operations) {
        for (size_t i = 0; i < operation.blocks.count; i++) {
            profile_block(plan, operation.blocks[i], cpu, gpu, ane, other);
        }
        if ([operation.operatorName isEqualToString:@"const"]) {
            continue;
        }
        MLComputePlanDeviceUsage * usage = [plan computeDeviceUsageForMLProgramOperation:operation];
        id<MLComputeDeviceProtocol> preferred = usage.preferredComputeDevice;
        tally_device(preferred, operation.operatorName.UTF8String, cpu, gpu, ane, other);
    }
}

}  // namespace

extern "C" void taco_vad_profile_cache(const char * cache_dir, int * cpu, int * gpu, int * ane, int * other) {
    *cpu = 0;
    *gpu = 0;
    *ane = 0;
    *other = 0;
    if (cache_dir == nullptr || cache_dir[0] == '\0') {
        std::printf("vad_ort_plan=missing\n");
        std::fflush(stdout);
        return;
    }
    if (@available(macOS 14.4, *)) {
        @autoreleasepool {
            NSString * root = [NSString stringWithUTF8String:cache_dir];
            NSDirectoryEnumerator * walk = [[NSFileManager defaultManager] enumeratorAtPath:root];
            NSMutableArray<NSString *> * compiled = [NSMutableArray array];
            for (NSString * rel in walk) {
                if ([rel.pathExtension isEqualToString:@"mlmodelc"]) {
                    [compiled addObject:[root stringByAppendingPathComponent:rel]];
                }
            }
            if (compiled.count == 0) {
                std::printf("vad_ort_plan=missing\n");
                std::fflush(stdout);
                return;
            }
            MLModelConfiguration * config = [[MLModelConfiguration alloc] init];
            config.computeUnits = MLComputeUnitsCPUAndGPU;
            for (NSString * path in compiled) {
                NSURL * url = [NSURL fileURLWithPath:path];
                dispatch_semaphore_t gate = dispatch_semaphore_create(0);
                __block int loaded = 0;
                [MLComputePlan loadContentsOfURL:url
                                   configuration:config
                               completionHandler:^(MLComputePlan * _Nullable plan, NSError * _Nullable error) {
                                   if (plan.modelStructure.program == nil) {
                                       const char * msg = error.localizedDescription.UTF8String;
                                       std::printf("vad_ort_plan=fail msg=%s\n", msg == nullptr ? "no_program" : msg);
                                   } else {
                                       loaded = 1;
                                       [plan.modelStructure.program.functions
                                           enumerateKeysAndObjectsUsingBlock:^(
                                               NSString * name, MLModelStructureProgramFunction * function,
                                               BOOL * _Nonnull __unused stop) {
                                             std::printf("vad_ort_plan function=%s\n", name.UTF8String);
                                             profile_block(plan, function.block, cpu, gpu, ane, other);
                                           }];
                                   }
                                   dispatch_semaphore_signal(gate);
                               }];
                const long wait =
                    dispatch_semaphore_wait(gate, dispatch_time(DISPATCH_TIME_NOW, (int64_t)60 * NSEC_PER_SEC));
                if (wait != 0) {
                    std::printf("vad_ort_plan=timeout\n");
                } else if (loaded == 0) {
                    std::printf("vad_ort_plan=empty\n");
                }
            }
            std::printf(
                "vad_ort_plan_count cpu=%d gpu=%d ane=%d other=%d\n", *cpu, *gpu, *ane, *other);
            std::fflush(stdout);
        }
    } else {
        std::printf("vad_ort_plan=unavailable\n");
        std::fflush(stdout);
    }
}
