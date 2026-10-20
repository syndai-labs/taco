#import <Foundation/Foundation.h>
#import <CoreAudio/CoreAudio.h>
#import <CoreAudio/AudioHardwareTapping.h>
#import <CoreAudio/CATapDescription.h>

#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

typedef struct taco_shared_ring taco_shared_ring;
// ponytail: weak no-op so this .o links. The tap IO proc discards samples.
// Push into the ring when a session reads tap audio.
__attribute__((weak)) void taco_ring_push(taco_shared_ring *ring, const uint8_t *bytes,
                                          size_t len) {
    (void)ring;
    (void)bytes;
    (void)len;
}

enum { TACO_TAP_SYSTEM = 1, TACO_TAP_PID = 2, TACO_TAP_BUNDLE = 3 };
enum { TACO_OK = 0, TACO_NOT_FOUND = 1, TACO_AMBIGUOUS = 2, TACO_OS = 3, TACO_REFUSED = 4 };

typedef struct taco_tap_result {
    int32_t status;
    uint32_t tap_id;
    uint32_t aggregate_id;
    int32_t created_tap;
    int32_t created_aggregate;
    char message[160];
} taco_tap_result;

typedef struct taco_input_result {
    int32_t status;
    uint32_t device_id;
    void *proc_id;
    char message[160];
} taco_input_result;

static void set_msg(char *dst, size_t cap, const char *text) {
    if (dst == NULL || cap == 0) {
        return;
    }
    strncpy(dst, text == NULL ? "" : text, cap - 1);
    dst[cap - 1] = '\0';
}

static AudioObjectPropertyAddress prop(AudioObjectPropertySelector selector) {
    AudioObjectPropertyAddress address = {
        .mSelector = selector,
        .mScope = kAudioObjectPropertyScopeGlobal,
        .mElement = kAudioObjectPropertyElementMain,
    };
    return address;
}

static AudioObjectID default_output(void) {
    AudioObjectID device = kAudioObjectUnknown;
    UInt32 size = sizeof(device);
    AudioObjectPropertyAddress address = prop(kAudioHardwarePropertyDefaultOutputDevice);
    if (AudioObjectGetPropertyData(kAudioObjectSystemObject, &address, 0, NULL, &size, &device) !=
        noErr) {
        return kAudioObjectUnknown;
    }
    return device;
}

int taco_spike_linked(void) { return 1; }

// HAL process objects are clients connected to CoreAudio right now
// (kAudioHardwarePropertyProcessObjectList). TranslatePIDToProcessObject
// returns kAudioObjectUnknown and noErr when the pid is not one of them.
// That miss stays not-found. It does not become a system tap.
static int find_pid(int32_t pid, AudioObjectID *out_proc) {
    pid_t value = (pid_t)pid;
    AudioObjectID proc = kAudioObjectUnknown;
    UInt32 size = sizeof(proc);
    AudioObjectPropertyAddress address = prop(kAudioHardwarePropertyTranslatePIDToProcessObject);
    OSStatus status = AudioObjectGetPropertyData(kAudioObjectSystemObject, &address, sizeof(value),
                                                 &value, &size, &proc);
    if (status != noErr || proc == kAudioObjectUnknown) {
        return TACO_NOT_FOUND;
    }
    *out_proc = proc;
    return TACO_OK;
}

static int find_one_bundle(const char *bundle_id, AudioObjectID *out_proc) {
    if (bundle_id == NULL || bundle_id[0] == '\0') {
        return TACO_NOT_FOUND;
    }
    NSString *wanted = [NSString stringWithUTF8String:bundle_id];
    if (wanted == nil) {
        return TACO_NOT_FOUND;
    }
    AudioObjectPropertyAddress address = prop(kAudioHardwarePropertyProcessObjectList);
    UInt32 size = 0;
    OSStatus status =
        AudioObjectGetPropertyDataSize(kAudioObjectSystemObject, &address, 0, NULL, &size);
    if (status != noErr) {
        return TACO_OS;
    }
    UInt32 count = size / (UInt32)sizeof(AudioObjectID);
    if (count == 0) {
        return TACO_NOT_FOUND;
    }
    if (count > 65536) {
        return TACO_OS;
    }
    AudioObjectID *ids = calloc(count, sizeof(AudioObjectID));
    if (ids == NULL) {
        return TACO_OS;
    }
    status = AudioObjectGetPropertyData(kAudioObjectSystemObject, &address, 0, NULL, &size, ids);
    if (status != noErr) {
        free(ids);
        return TACO_OS;
    }
    UInt32 matches = 0;
    AudioObjectID found = kAudioObjectUnknown;
    AudioObjectPropertyAddress bundle = prop(kAudioProcessPropertyBundleID);
    for (UInt32 i = 0; i < count; i++) {
        CFStringRef name = NULL;
        UInt32 name_size = sizeof(name);
        status = AudioObjectGetPropertyData(ids[i], &bundle, 0, NULL, &name_size, &name);
        if (status != noErr || name == NULL) {
            continue;
        }
        BOOL equal = [(__bridge NSString *)name isEqualToString:wanted];
        CFRelease(name);
        if (!equal) {
            continue;
        }
        matches += 1;
        found = ids[i];
        if (matches > 1) {
            break;
        }
    }
    free(ids);
    if (matches == 0) {
        return TACO_NOT_FOUND;
    }
    if (matches > 1) {
        return TACO_AMBIGUOUS;
    }
    *out_proc = found;
    return TACO_OK;
}

// Operator window while the aggregate is started. AudioDeviceStart returns
// before the AudioCapture dialog is answered, so the process has to stay up.
// 0 starts and stops without waiting. Out of range uses the default.
// Does not press Allow or Deny.
static int spike_open_ms(void) {
    const char *env = getenv("TACO_CAPTURE_SPIKE_OPEN_MS");
    if (env == NULL || env[0] == '\0') {
        return 300000;
    }
    char *end = NULL;
    long value = strtol(env, &end, 10);
    if (end == env || *end != '\0' || value < 0 || value > 600000L) {
        return 300000;
    }
    return (int)value;
}

static void sleep_ms(int ms) {
    if (ms <= 0) {
        return;
    }
    struct timespec ts = {
        .tv_sec = (time_t)(ms / 1000),
        .tv_nsec = (long)(ms % 1000) * 1000000L,
    };
    while (nanosleep(&ts, &ts) != 0 && errno == EINTR) {
    }
}

// No-op. Starting the aggregate is what asks for AudioCapture. Samples are not read.
static OSStatus taco_tap_ioproc(AudioObjectID device, const AudioTimeStamp *now,
                                const AudioBufferList *input, const AudioTimeStamp *input_time,
                                AudioBufferList *output, const AudioTimeStamp *output_time,
                                void *client) {
    (void)device;
    (void)now;
    (void)input;
    (void)input_time;
    (void)output;
    (void)output_time;
    (void)client;
    return noErr;
}

void taco_process_tap_close(uint32_t tap_id, int32_t created_tap, uint32_t aggregate_id,
                            int32_t created_aggregate) {
    if (created_aggregate && aggregate_id != kAudioObjectUnknown) {
        AudioHardwareDestroyAggregateDevice(aggregate_id);
    }
    if (created_tap && tap_id != kAudioObjectUnknown) {
        AudioHardwareDestroyProcessTap(tap_id);
    }
}

int taco_process_tap_open(int kind, int32_t pid, const char *bundle_id, taco_tap_result *out) {
    if (out == NULL) {
        return TACO_OS;
    }
    memset(out, 0, sizeof(*out));
    @autoreleasepool {
        AudioObjectID proc = kAudioObjectUnknown;
        if (kind == TACO_TAP_BUNDLE) {
            int rc = find_one_bundle(bundle_id, &proc);
            if (rc != TACO_OK) {
                out->status = rc;
                set_msg(out->message, sizeof(out->message),
                        rc == TACO_AMBIGUOUS ? "app_ambiguous" : "app_not_found");
                return rc;
            }
        } else if (kind == TACO_TAP_PID) {
            if (find_pid(pid, &proc) != TACO_OK) {
                out->status = TACO_NOT_FOUND;
                set_msg(out->message, sizeof(out->message), "selector_unresolved");
                return TACO_NOT_FOUND;
            }
        } else if (kind != TACO_TAP_SYSTEM) {
            out->status = TACO_REFUSED;
            set_msg(out->message, sizeof(out->message), "selector_syntax");
            return TACO_REFUSED;
        }

        AudioObjectID output_before = default_output();
        CATapDescription *desc = (kind == TACO_TAP_SYSTEM)
                                     ? [[CATapDescription alloc] initStereoGlobalTapButExcludeProcesses:@[]]
                                     : [[CATapDescription alloc] initStereoMixdownOfProcesses:@[@(proc)]];
        if (desc == nil) {
            out->status = TACO_OS;
            set_msg(out->message, sizeof(out->message), "tap_description");
            return TACO_OS;
        }
        desc.privateTap = YES;
        desc.muteBehavior = CATapUnmuted;
        desc.name = @"taco";
        if (@available(macOS 26.0, *)) {
            desc.processRestoreEnabled = NO;
        }

        AudioObjectID tap = kAudioObjectUnknown;
        OSStatus status = AudioHardwareCreateProcessTap(desc, &tap);
        if (status != noErr || tap == kAudioObjectUnknown) {
            out->status = TACO_OS;
            set_msg(out->message, sizeof(out->message), "create_process_tap");
            return TACO_OS;
        }
        out->created_tap = 1;
        out->tap_id = (uint32_t)tap;

        NSString *uuid = desc.UUID.UUIDString;
        if (uuid.length == 0) {
            taco_process_tap_close(out->tap_id, 1, 0, 0);
            memset(out, 0, sizeof(*out));
            out->status = TACO_OS;
            set_msg(out->message, sizeof(out->message), "tap_uuid");
            return TACO_OS;
        }
        NSDictionary *aggregate = @{
            @kAudioAggregateDeviceNameKey : @"taco",
            @kAudioAggregateDeviceUIDKey : [NSUUID UUID].UUIDString,
            @kAudioAggregateDeviceTapListKey : @[ @{@kAudioSubTapUIDKey : uuid} ],
            @kAudioAggregateDeviceIsPrivateKey : @1,
            @kAudioAggregateDeviceTapAutoStartKey : @0,
        };
        AudioObjectID aggregate_id = kAudioObjectUnknown;
        status = AudioHardwareCreateAggregateDevice((__bridge CFDictionaryRef)aggregate, &aggregate_id);
        if (status != noErr || aggregate_id == kAudioObjectUnknown) {
            taco_process_tap_close(out->tap_id, 1, 0, 0);
            memset(out, 0, sizeof(*out));
            out->status = TACO_OS;
            set_msg(out->message, sizeof(out->message), "create_aggregate");
            return TACO_OS;
        }
        out->created_aggregate = 1;
        out->aggregate_id = (uint32_t)aggregate_id;
        if (output_before != kAudioObjectUnknown && default_output() != output_before) {
            taco_process_tap_close(out->tap_id, 1, out->aggregate_id, 1);
            memset(out, 0, sizeof(*out));
            out->status = TACO_REFUSED;
            set_msg(out->message, sizeof(out->message), "default_output_changed");
            return TACO_REFUSED;
        }

        // TapAutoStart stays 0. A 1 makes AudioDeviceStart wait for the first
        // buffer from a tapped process. The prompt is this start, not create.
        AudioDeviceIOProcID io_proc = NULL;
        status = AudioDeviceCreateIOProcID(aggregate_id, taco_tap_ioproc, NULL, &io_proc);
        if (status != noErr || io_proc == NULL) {
            taco_process_tap_close(out->tap_id, 1, out->aggregate_id, 1);
            memset(out, 0, sizeof(*out));
            out->status = TACO_OS;
            set_msg(out->message, sizeof(out->message), "create_ioproc");
            return TACO_OS;
        }
        status = AudioDeviceStart(aggregate_id, io_proc);
        if (status != noErr) {
            AudioDeviceDestroyIOProcID(aggregate_id, io_proc);
            taco_process_tap_close(out->tap_id, 1, out->aggregate_id, 1);
            memset(out, 0, sizeof(*out));
            out->status = TACO_OS;
            set_msg(out->message, sizeof(out->message), "start_tap");
            return TACO_OS;
        }
        sleep_ms(spike_open_ms());
        AudioDeviceStop(aggregate_id, io_proc);
        AudioDeviceDestroyIOProcID(aggregate_id, io_proc);

        out->status = TACO_OK;
        set_msg(out->message, sizeof(out->message), "ok");
        return TACO_OK;
    }
}

static OSStatus taco_input_ioproc(AudioObjectID device, const AudioTimeStamp *now,
                                  const AudioBufferList *input, const AudioTimeStamp *input_time,
                                  AudioBufferList *output, const AudioTimeStamp *output_time,
                                  void *client) {
    (void)device;
    (void)now;
    (void)input_time;
    (void)output;
    (void)output_time;
    if (client == NULL || input == NULL) {
        return noErr;
    }
    for (UInt32 i = 0; i < input->mNumberBuffers; i++) {
        const AudioBuffer *buffer = &input->mBuffers[i];
        if (buffer->mData == NULL || buffer->mDataByteSize == 0) {
            continue;
        }
        taco_ring_push(client, buffer->mData, (size_t)buffer->mDataByteSize);
    }
    return noErr;
}

static AudioObjectID device_for_uid(const char *uid) {
    if (uid == NULL || uid[0] == '\0') {
        AudioObjectID device = kAudioObjectUnknown;
        UInt32 size = sizeof(device);
        AudioObjectPropertyAddress address = prop(kAudioHardwarePropertyDefaultInputDevice);
        if (AudioObjectGetPropertyData(kAudioObjectSystemObject, &address, 0, NULL, &size,
                                       &device) != noErr) {
            return kAudioObjectUnknown;
        }
        return device;
    }
    CFStringRef cf = CFStringCreateWithCString(NULL, uid, kCFStringEncodingUTF8);
    if (cf == NULL) {
        return kAudioObjectUnknown;
    }
    AudioObjectID device = kAudioObjectUnknown;
    UInt32 size = sizeof(device);
    AudioObjectPropertyAddress address = prop(kAudioHardwarePropertyTranslateUIDToDevice);
    OSStatus status = AudioObjectGetPropertyData(kAudioObjectSystemObject, &address,
                                                 sizeof(CFStringRef), &cf, &size, &device);
    CFRelease(cf);
    return status == noErr ? device : kAudioObjectUnknown;
}

int taco_input_open(const char *uid_or_null, taco_shared_ring *ring, taco_input_result *out) {
    if (out == NULL) {
        return TACO_OS;
    }
    memset(out, 0, sizeof(*out));
    if (ring == NULL) {
        out->status = TACO_REFUSED;
        set_msg(out->message, sizeof(out->message), "ring_required");
        return TACO_REFUSED;
    }
    AudioObjectID device = device_for_uid(uid_or_null);
    if (device == kAudioObjectUnknown) {
        out->status = TACO_NOT_FOUND;
        set_msg(out->message, sizeof(out->message), "selector_unresolved");
        return TACO_NOT_FOUND;
    }
    AudioDeviceIOProcID proc = NULL;
    OSStatus status = AudioDeviceCreateIOProcID(device, taco_input_ioproc, ring, &proc);
    if (status != noErr || proc == NULL) {
        out->status = TACO_OS;
        set_msg(out->message, sizeof(out->message), "create_ioproc");
        return TACO_OS;
    }
    status = AudioDeviceStart(device, proc);
    if (status != noErr) {
        AudioDeviceDestroyIOProcID(device, proc);
        out->status = TACO_OS;
        set_msg(out->message, sizeof(out->message), "start_input");
        return TACO_OS;
    }
    out->status = TACO_OK;
    out->device_id = (uint32_t)device;
    out->proc_id = (void *)proc;
    set_msg(out->message, sizeof(out->message), "ok");
    return TACO_OK;
}

void taco_input_close(uint32_t device_id, void *proc_id) {
    if (device_id == 0 || proc_id == NULL) {
        return;
    }
    AudioDeviceStop((AudioDeviceID)device_id, (AudioDeviceIOProcID)proc_id);
    AudioDeviceDestroyIOProcID((AudioDeviceID)device_id, (AudioDeviceIOProcID)proc_id);
}
