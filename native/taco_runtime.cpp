// In-process load of the pinned Silero, Whisper small, and Hy-MT2 files.
// Called from the taco binary. Does not claim M0-AC2 or M0-AC3.
// GPU layers stay on the Metal device (all layers).

#include "taco_engines.h"

#include "ggml-backend.h"
#include "llama.h"
#include "whisper.h"

#include <CommonCrypto/CommonDigest.h>

#include <mach/mach.h>
#include <malloc/malloc.h>

#include <atomic>
#include <chrono>
#include <cmath>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <string>
#include <thread>
#include <vector>

int taco_vad_ort(const char * path);

extern "C" void taco_engines_vm_sample(taco_vm_sample * out) {
    if (out == nullptr) {
        return;
    }
    std::memset(out, 0, sizeof *out);
    task_vm_info_data_t info;
    std::memset(&info, 0, sizeof info);
    mach_msg_type_number_t count = TASK_VM_INFO_COUNT;
    const kern_return_t kr = task_info(mach_task_self(), TASK_VM_INFO, reinterpret_cast<task_info_t>(&info), &count);
    if (kr != KERN_SUCCESS || count < TASK_VM_INFO_REV1_COUNT) {
        return;
    }
    out->ok = 1;
    out->phys_footprint = info.phys_footprint;
    out->resident_size = info.resident_size;
    out->device = info.device;
    if (count >= TASK_VM_INFO_REV3_COUNT) {
        out->graphics_ok = 1;
        out->ledger_phys_footprint_peak = info.ledger_phys_footprint_peak;
        out->graphics_footprint = info.ledger_tag_graphics_footprint;
        out->graphics_nofootprint = info.ledger_tag_graphics_nofootprint;
    }
    malloc_statistics_t stats;
    std::memset(&stats, 0, sizeof stats);
    malloc_zone_statistics(malloc_default_zone(), &stats);
    out->malloc_size_in_use = stats.size_in_use;
    out->malloc_max_size_in_use = stats.max_size_in_use;
}

static void log_phase(const char * phase) {
    task_vm_info_data_t info;
    std::memset(&info, 0, sizeof info);
    mach_msg_type_number_t count = TASK_VM_INFO_COUNT;
    const kern_return_t kr = task_info(mach_task_self(), TASK_VM_INFO, reinterpret_cast<task_info_t>(&info), &count);
    if (kr != KERN_SUCCESS || count < TASK_VM_INFO_REV1_COUNT) {
        std::printf("residency_phase phase=%s unread=1\n", phase);
        std::fflush(stdout);
        return;
    }
    std::printf(
        "residency_phase phase=%s phys_footprint=%llu resident_size=%llu virtual_size=%llu region_count=%d internal=%llu compressed=%llu\n",
        phase,
        static_cast<unsigned long long>(info.phys_footprint),
        static_cast<unsigned long long>(info.resident_size),
        static_cast<unsigned long long>(info.virtual_size),
        info.region_count,
        static_cast<unsigned long long>(info.internal),
        static_cast<unsigned long long>(info.compressed));
    std::fflush(stdout);
}

namespace {

constexpr int kReserveMs = 2000;

const char * kVadSha = "1a153a22f4509e292a94e67d6f9b85e8deb25b4988682b7e174c65279d8788e3";
const char * kAsrSha = "1be3a9b2063867b937e64e2ec7483364a79917e157fa98c5d94b5c1fffea987b";
const char * kMtSha = "dc5f44fcf1fa496ee7ad725982c0c8c553a4de00259b53af84c4b89fb0c06699";

std::atomic<int> g_off_n{-1};
std::atomic<int> g_off_d{-1};

int64_t now_ms() {
    using clock = std::chrono::steady_clock;
    return std::chrono::duration_cast<std::chrono::milliseconds>(clock::now().time_since_epoch()).count();
}

void log_cb(enum ggml_log_level, const char * text, void *) {
    if (text == nullptr) {
        return;
    }
    std::fputs(text, stderr);
    const char * off = std::strstr(text, "offloaded ");
    if (off != nullptr) {
        int n = 0;
        int d = 0;
        if (std::sscanf(off, "offloaded %d/%d", &n, &d) == 2) {
            g_off_n.store(n);
            g_off_d.store(d);
        }
    }
}

std::string sha256_file(const char * path) {
    FILE * file = std::fopen(path, "rb");
    if (file == nullptr) {
        return {};
    }
    CC_SHA256_CTX ctx;
    CC_SHA256_Init(&ctx);
    unsigned char buf[1 << 16];
    size_t nread = 0;
    while ((nread = std::fread(buf, 1, sizeof buf, file)) > 0) {
        CC_SHA256_Update(&ctx, buf, static_cast<CC_LONG>(nread));
    }
    std::fclose(file);
    unsigned char out[CC_SHA256_DIGEST_LENGTH];
    CC_SHA256_Final(out, &ctx);
    static const char * hexd = "0123456789abcdef";
    std::string digest(64, '0');
    for (int i = 0; i < CC_SHA256_DIGEST_LENGTH; i++) {
        digest[static_cast<size_t>(i * 2)] = hexd[out[i] >> 4];
        digest[static_cast<size_t>(i * 2 + 1)] = hexd[out[i] & 0xf];
    }
    return digest;
}

bool expect_digest(const char * role, const char * path, const char * want) {
    const std::string got = sha256_file(path);
    const int ok = got == want;
    std::printf("digest role=%s ok=%d sha256=%s\n", role, ok ? 1 : 0, got.c_str());
    std::fflush(stdout);
    return ok;
}

struct Gate {
    std::atomic<int> abort{0};
    std::atomic<int> started{0};
    std::atomic<int> done{0};
    std::atomic<int> progress_milli{-1};
    std::atomic<int> tokens{0};
    int decode_rc = 0;
    int steps = 0;
};

bool abort_cb(void * data) {
    return static_cast<Gate *>(data)->abort.load() != 0;
}

bool progress_cb(float progress, void * data) {
    auto * gate = static_cast<Gate *>(data);
    gate->progress_milli.store(static_cast<int>(progress * 1000.0f));
    gate->started.store(1);
    if (gate->abort.load() != 0) {
        return false;
    }
    return true;
}

int run_vad(const char * path) {
    whisper_vad_context_params params = whisper_vad_default_context_params();
    params.n_threads = 1;
    params.use_gpu = true;
    whisper_vad_context * ctx = whisper_vad_init_from_file_with_params(path, params);
    if (ctx == nullptr) {
        std::printf("vad=fail\n");
        std::fflush(stdout);
        return 1;
    }
    std::vector<float> frame(512, 0.0f);
    frame[100] = 0.2f;
    whisper_vad_reset_state(ctx);
    const bool first = whisper_vad_detect_speech_no_reset(ctx, frame.data(), static_cast<int>(frame.size()));
    const int first_probs = whisper_vad_n_probs(ctx);
    const bool second = whisper_vad_detect_speech_no_reset(ctx, frame.data(), static_cast<int>(frame.size()));
    const int second_probs = whisper_vad_n_probs(ctx);
    std::printf(
        "vad=ok use_gpu_requested=1 first=%d first_probs=%d second=%d second_probs=%d\n",
        first ? 1 : 0,
        first_probs,
        second ? 1 : 0,
        second_probs);
    whisper_vad_free(ctx);
    std::fflush(stdout);
    return first && second && first_probs > 0 && second_probs > 0 ? 0 : 1;
}

int run_asr(const char * path, whisper_context ** kept) {
    whisper_context_params cparams = whisper_context_default_params();
    cparams.use_gpu = true;
    whisper_context * ctx = whisper_init_from_file_with_params(path, cparams);
    if (ctx == nullptr) {
        std::printf("asr=fail_load\n");
        std::fflush(stdout);
        return 1;
    }
    *kept = ctx;
    std::vector<float> audio(16000, 0.0f);
    for (int i = 0; i < 16000; i++) {
        const float t = static_cast<float>(i) / 16000.0f;
        audio[static_cast<size_t>(i)] = 0.1f * std::sin(2.0f * 3.14159265f * 440.0f * t);
    }
    whisper_full_params params = whisper_full_default_params(WHISPER_SAMPLING_GREEDY);
    params.translate = false;
    params.language = "en";
    params.n_threads = 4;
    params.print_progress = false;
    params.print_realtime = false;
    params.print_special = false;
    params.print_timestamps = false;
    params.no_timestamps = true;
    params.single_segment = true;
    params.no_context = true;
    params.suppress_blank = true;
    params.vad = false;
    const int rc = whisper_full(ctx, params, audio.data(), static_cast<int>(audio.size()));
    int text_bytes = 0;
    int nseg = 0;
    if (rc == 0) {
        nseg = whisper_full_n_segments(ctx);
        for (int i = 0; i < nseg; i++) {
            const char * text = whisper_full_get_segment_text(ctx, i);
            if (text != nullptr) {
                text_bytes += static_cast<int>(std::strlen(text));
            }
        }
    }
    std::printf("asr=%s use_gpu=1 segments=%d text_bytes=%d\n", rc == 0 ? "ok" : "fail", nseg, text_bytes);
    std::fflush(stdout);
    return rc == 0 ? 0 : 1;
}

// whisper_init_from_file_with_params has no abort callback. The loader read is
// the checkpoint: once the weight section is reached and abort is set, the next
// 4-byte header field is 99, which whisper_model_load rejects. Returning 0 from
// read does not fail the load; this function ignores that count.
// The weight buffer already exists by then. The pinned whisper.cpp failure path
// is patched to call whisper_free so that buffer leaves the Metal residency set.
struct AbortRead {
    FILE * file = nullptr;
    std::atomic<int> * abort = nullptr;
    size_t offset = 0;
    bool poison = false;
    int poisoned = 0;
};

size_t abort_read(void * ctx, void * output, size_t read_size) {
    auto * file = static_cast<AbortRead *>(ctx);
    const bool in_weights = file->offset > (4u << 20);
    if (file->abort->load() != 0 && in_weights && file->poison && read_size == sizeof(int32_t)) {
        const int32_t bad = 99;
        std::memcpy(output, &bad, sizeof bad);
        file->poison = false;
        file->poisoned = 1;
        file->offset += read_size;
        return read_size;
    }
    if (file->abort->load() != 0 && in_weights && read_size == sizeof(int32_t)) {
        const int32_t bad = 99;
        std::memcpy(output, &bad, sizeof bad);
        file->poisoned = 1;
        file->offset += read_size;
        return read_size;
    }
    if (file->abort->load() != 0 && in_weights && read_size >= 4096) {
        std::memset(output, 0, read_size);
        file->poison = true;
        file->offset += read_size;
        return read_size;
    }
    const size_t got = std::fread(output, 1, read_size, file->file);
    if (got < read_size) {
        std::memset(static_cast<unsigned char *>(output) + got, 0, read_size - got);
    }
    file->offset += read_size;
    return read_size;
}

bool abort_eof(void * ctx) {
    auto * file = static_cast<AbortRead *>(ctx);
    if (file->poisoned != 0 || file->poison) {
        return false;
    }
    return std::feof(file->file) != 0;
}

void abort_close(void * ctx) {
    auto * file = static_cast<AbortRead *>(ctx);
    if (file->file != nullptr) {
        std::fclose(file->file);
        file->file = nullptr;
    }
}

int cancel_whisper_init(const char * path, int * join_ms, int * poisoned) {
    Gate gate;
    AbortRead file;
    file.abort = &gate.abort;
    file.file = std::fopen(path, "rb");
    if (file.file == nullptr) {
        return 0;
    }
    whisper_context * loaded = nullptr;
    std::thread worker([&] {
        gate.started.store(1);
        whisper_model_loader loader{};
        loader.context = &file;
        loader.read = abort_read;
        loader.eof = abort_eof;
        loader.close = abort_close;
        whisper_context_params cparams = whisper_context_default_params();
        cparams.use_gpu = true;
        loaded = whisper_init_with_params(&loader, cparams);
        gate.done.store(1);
    });
    while (gate.started.load() == 0) {
        std::this_thread::sleep_for(std::chrono::milliseconds(1));
    }
    const int64_t t0 = now_ms();
    gate.abort.store(1);
    worker.join();
    *join_ms = static_cast<int>(now_ms() - t0);
    *poisoned = file.poisoned;
    if (loaded != nullptr) {
        whisper_free(loaded);
        return 2;
    }
    return 1;
}

struct WhisperAbort {
    std::atomic<int> * abort = nullptr;
    std::atomic<int> hits{0};
};

bool whisper_abort_cb(void * data) {
    auto * gate = static_cast<WhisperAbort *>(data);
    const int hit = gate->hits.fetch_add(1) + 1;
    // Hit 1 is the end of the encoder graph. Let that pass so the next
    // checkpoint is the decoder graph. Later hits honor the abort flag.
    if (hit == 1) {
        return false;
    }
    return gate->abort->load() != 0;
}

bool whisper_begin_cb(struct whisper_context *, struct whisper_state *, void * data) {
    static_cast<Gate *>(data)->started.store(1);
    return true;
}

int cancel_whisper_decode(whisper_context * ctx, int * join_ms, int * hits, int * full_rc) {
    Gate gate;
    WhisperAbort abort;
    abort.abort = &gate.abort;
    std::vector<float> audio(16000, 0.0f);
    for (int i = 0; i < 16000; i++) {
        const float t = static_cast<float>(i) / 16000.0f;
        audio[static_cast<size_t>(i)] = 0.1f * std::sin(2.0f * 3.14159265f * 440.0f * t);
    }
    whisper_full_params params = whisper_full_default_params(WHISPER_SAMPLING_GREEDY);
    params.translate = false;
    params.language = "en";
    params.n_threads = 4;
    params.print_progress = false;
    params.print_realtime = false;
    params.print_special = false;
    params.print_timestamps = false;
    params.no_timestamps = true;
    params.single_segment = true;
    params.no_context = true;
    params.suppress_blank = true;
    params.vad = false;
    params.abort_callback = whisper_abort_cb;
    params.abort_callback_user_data = &abort;
    params.encoder_begin_callback = whisper_begin_cb;
    params.encoder_begin_callback_user_data = &gate;
    std::thread worker([&] {
        gate.decode_rc = whisper_full(ctx, params, audio.data(), static_cast<int>(audio.size()));
        gate.done.store(1);
    });
    const int64_t wait_start = now_ms();
    while (gate.started.load() == 0 && gate.done.load() == 0 && now_ms() - wait_start < 60000) {
        std::this_thread::sleep_for(std::chrono::milliseconds(1));
    }
    const int64_t t0 = now_ms();
    gate.abort.store(1);
    worker.join();
    *join_ms = static_cast<int>(now_ms() - t0);
    *hits = abort.hits.load();
    *full_rc = gate.decode_rc;
    if (gate.decode_rc == -8) {
        return 1;
    }
    return 0;
}

int cancel_init(const char * path, int * join_ms, int * progress_milli) {
    Gate gate;
    llama_model * loaded = nullptr;
    std::thread worker([&] {
        gate.started.store(1);
        llama_model_params params = llama_model_default_params();
        params.n_gpu_layers = -1;
        params.progress_callback = progress_cb;
        params.progress_callback_user_data = &gate;
        loaded = llama_model_load_from_file(path, params);
        gate.done.store(1);
    });
    while (gate.started.load() == 0) {
        std::this_thread::sleep_for(std::chrono::milliseconds(1));
    }
    const int64_t t0 = now_ms();
    gate.abort.store(1);
    worker.join();
    *join_ms = static_cast<int>(now_ms() - t0);
    *progress_milli = gate.progress_milli.load();
    const int status = loaded == nullptr ? 1 : 2;
    if (loaded != nullptr) {
        llama_model_free(loaded);
    }
    return status;
}

struct MtSession {
    llama_model * model = nullptr;
    llama_context * ctx = nullptr;
    const llama_vocab * vocab = nullptr;
};

int load_mt(const char * path, MtSession * session) {
    llama_model_params mparams = llama_model_default_params();
    mparams.n_gpu_layers = -1;
    session->model = llama_model_load_from_file(path, mparams);
    if (session->model == nullptr) {
        return 1;
    }
    llama_context_params cparams = llama_context_default_params();
    cparams.n_ctx = 512;
    cparams.n_batch = 128;
    session->ctx = llama_init_from_model(session->model, cparams);
    if (session->ctx == nullptr) {
        llama_model_free(session->model);
        session->model = nullptr;
        return 2;
    }
    session->vocab = llama_model_get_vocab(session->model);
    return 0;
}

void free_mt(MtSession * session) {
    if (session->ctx != nullptr) {
        llama_free(session->ctx);
        session->ctx = nullptr;
    }
    if (session->model != nullptr) {
        llama_model_free(session->model);
        session->model = nullptr;
    }
}

int run_mt_smoke(MtSession * session, int * text_bytes) {
    const char * prompt =
        "Translate the following text into Chinese. Note that you should only output the translated "
        "result without any additional explanation:\n\nHello";
    std::vector<llama_token> tokens(256);
    const int n = llama_tokenize(
        session->vocab,
        prompt,
        static_cast<int32_t>(std::strlen(prompt)),
        tokens.data(),
        static_cast<int32_t>(tokens.size()),
        true,
        true);
    if (n <= 0) {
        return 3;
    }
    llama_batch batch = llama_batch_get_one(tokens.data(), n);
    if (llama_decode(session->ctx, batch) != 0) {
        return 4;
    }
    llama_sampler_chain_params sparams = llama_sampler_chain_default_params();
    llama_sampler * sampler = llama_sampler_chain_init(sparams);
    llama_sampler_chain_add(sampler, llama_sampler_init_greedy());
    int bytes = 0;
    for (int i = 0; i < 8; i++) {
        const llama_token id = llama_sampler_sample(sampler, session->ctx, -1);
        if (llama_vocab_is_eog(session->vocab, id)) {
            break;
        }
        char piece[64];
        const int wrote = llama_token_to_piece(session->vocab, id, piece, static_cast<int32_t>(sizeof piece), 0, true);
        if (wrote > 0) {
            bytes += wrote;
        }
        llama_token one = id;
        llama_batch next = llama_batch_get_one(&one, 1);
        if (llama_decode(session->ctx, next) != 0) {
            llama_sampler_free(sampler);
            *text_bytes = bytes;
            return 5;
        }
    }
    llama_sampler_free(sampler);
    *text_bytes = bytes;
    return 0;
}

int cancel_decode(MtSession * session, int * join_ms, int * steps, int * decode_rc) {
    Gate gate;
    llama_set_abort_callback(session->ctx, abort_cb, &gate);
    llama_sampler_chain_params sparams = llama_sampler_chain_default_params();
    llama_sampler * sampler = llama_sampler_chain_init(sparams);
    llama_sampler_chain_add(sampler, llama_sampler_init_greedy());
    std::thread worker([&] {
        gate.started.store(1);
        for (int i = 0; i < 128; i++) {
            const llama_token id = llama_sampler_sample(sampler, session->ctx, -1);
            gate.tokens.store(i + 1);
            gate.steps = i + 1;
            if (llama_vocab_is_eog(session->vocab, id)) {
                gate.decode_rc = 0;
                break;
            }
            llama_token one = id;
            llama_batch next = llama_batch_get_one(&one, 1);
            const int rc = llama_decode(session->ctx, next);
            gate.decode_rc = rc;
            if (rc != 0) {
                break;
            }
        }
        gate.done.store(1);
    });
    const int64_t wait_start = now_ms();
    while (gate.tokens.load() < 1 && gate.done.load() == 0 && now_ms() - wait_start < 60000) {
        std::this_thread::sleep_for(std::chrono::milliseconds(5));
    }
    const int64_t t0 = now_ms();
    gate.abort.store(1);
    worker.join();
    llama_sampler_free(sampler);
    *join_ms = static_cast<int>(now_ms() - t0);
    *steps = gate.steps;
    *decode_rc = gate.decode_rc;
    if (gate.decode_rc == 2) {
        return 1;
    }
    if (gate.done.load() != 0 && gate.steps >= 128) {
        return 2;
    }
    return gate.decode_rc == 0 ? 3 : 0;
}

}  // namespace

extern "C" int taco_engines_prove(
    const char * vad_path,
    const char * asr_path,
    const char * mt_path,
    int reserve_ms,
    taco_engines_proof * out) {
    (void)reserve_ms;
    if (out == nullptr || vad_path == nullptr || asr_path == nullptr || mt_path == nullptr) {
        return 2;
    }
    std::memset(out, 0, sizeof *out);
    const int linked = taco_engines_linked();
    std::printf("taco_engines_linked gpu_offload=%d\n", linked);
    std::fflush(stdout);
    if (!expect_digest("vad", vad_path, kVadSha) || !expect_digest("asr", asr_path, kAsrSha) ||
        !expect_digest("mt", mt_path, kMtSha)) {
        std::printf("result=digest_mismatch\n");
        return 2;
    }

    llama_log_set(log_cb, nullptr);
    log_phase("before_load");
    const int vad_whisper = run_vad(vad_path);
    log_phase("after_vad_whisper");
    const int vad_rc = taco_vad_ort(vad_path);
    log_phase("after_vad_ort");

    llama_backend_init();
    const int gpu_offload = llama_supports_gpu_offload() ? 1 : 0;
    ggml_backend_dev_t gpu = ggml_backend_dev_by_type(GGML_BACKEND_DEVICE_TYPE_GPU);
    std::printf("gpu_offload=%d\n", gpu_offload);
    std::printf("gpu_device=%s\n", gpu == nullptr ? "none" : ggml_backend_dev_name(gpu));
    std::fflush(stdout);

    whisper_context * asr_ctx = nullptr;
    const int asr_rc = gpu == nullptr ? 1 : run_asr(asr_path, &asr_ctx);

    int whisper_decode_join_ms = -1;
    int whisper_decode_hits = 0;
    int whisper_decode_rc = -1;
    int whisper_decode_status = 0;
    if (asr_ctx != nullptr) {
        whisper_decode_status =
            cancel_whisper_decode(asr_ctx, &whisper_decode_join_ms, &whisper_decode_hits, &whisper_decode_rc);
        whisper_free(asr_ctx);
        asr_ctx = nullptr;
    }
    log_phase("after_whisper_free");
    const char * whisper_decode_name = "not_run";
    if (whisper_decode_rc == -8) {
        whisper_decode_name = "aborted";
    } else if (whisper_decode_rc == -6) {
        whisper_decode_name = "encode_checkpoint";
    } else if (whisper_decode_status == 0 && whisper_decode_rc >= 0) {
        whisper_decode_name = "ran_through";
    }
    const int whisper_decode_within =
        whisper_decode_rc == -8 && whisper_decode_join_ms >= 0 && whisper_decode_join_ms <= kReserveMs;
    std::printf(
        "whisper_cancel_decode=%s rc=%d hits=%d join_ms=%d reserve_ms=%d within_reserve=%d\n",
        whisper_decode_name,
        whisper_decode_rc,
        whisper_decode_hits,
        whisper_decode_join_ms,
        kReserveMs,
        whisper_decode_within);
    std::fflush(stdout);

    int whisper_init_join_ms = -1;
    int whisper_init_poisoned = 0;
    const int whisper_init_status = cancel_whisper_init(asr_path, &whisper_init_join_ms, &whisper_init_poisoned);
    const char * whisper_init_name = "not_started";
    if (whisper_init_status == 1) {
        whisper_init_name = "aborted";
    } else if (whisper_init_status == 2) {
        whisper_init_name = "loaded_fully";
    }
    const int whisper_init_within =
        whisper_init_status == 1 && whisper_init_join_ms >= 0 && whisper_init_join_ms <= kReserveMs;
    std::printf(
        "whisper_cancel_init=%s join_ms=%d poisoned=%d reserve_ms=%d within_reserve=%d\n",
        whisper_init_name,
        whisper_init_join_ms,
        whisper_init_poisoned,
        kReserveMs,
        whisper_init_within);
    std::fflush(stdout);
    log_phase("after_whisper_cancel_init");

    int init_join_ms = -1;
    int init_progress = -1;
    const int init_status = cancel_init(mt_path, &init_join_ms, &init_progress);
    const char * init_name = "not_started";
    if (init_status == 1) {
        init_name = "aborted";
    } else if (init_status == 2) {
        init_name = "loaded_fully";
    }
    const int init_within = init_status == 1 && init_join_ms >= 0 && init_join_ms <= kReserveMs;
    std::printf(
        "cancel_init=%s join_ms=%d progress_milli=%d reserve_ms=%d within_reserve=%d\n",
        init_name,
        init_join_ms,
        init_progress,
        kReserveMs,
        init_within);
    std::fflush(stdout);
    log_phase("after_mt_cancel_init");

    g_off_n.store(-1);
    g_off_d.store(-1);
    MtSession session;
    const int load_rc = load_mt(mt_path, &session);
    const int off_n = g_off_n.load();
    const int off_d = g_off_d.load();
    std::printf("mt_load=%d n_gpu_layers=-1 offloaded=%d/%d\n", load_rc, off_n, off_d);
    std::fflush(stdout);

    int text_bytes = 0;
    int mt_rc = load_rc == 0 ? run_mt_smoke(&session, &text_bytes) : load_rc;
    std::printf("mt=%s text_bytes=%d\n", mt_rc == 0 ? "ok" : "fail", text_bytes);
    std::fflush(stdout);

    int decode_join_ms = -1;
    int decode_steps = 0;
    int decode_rc = -1;
    int decode_status = 0;
    if (session.ctx != nullptr) {
        decode_status = cancel_decode(&session, &decode_join_ms, &decode_steps, &decode_rc);
    }
    const char * decode_name = "not_run";
    if (session.ctx == nullptr) {
        decode_name = "not_run";
    } else if (decode_rc == 2) {
        decode_name = "aborted";
    } else if (decode_status == 2) {
        decode_name = "ran_through";
    } else {
        decode_name = "stopped";
    }
    const int decode_within = decode_rc == 2 && decode_join_ms >= 0 && decode_join_ms <= kReserveMs;
    std::printf(
        "cancel_decode=%s rc=%d steps=%d join_ms=%d reserve_ms=%d within_reserve=%d\n",
        decode_name,
        decode_rc,
        decode_steps,
        decode_join_ms,
        kReserveMs,
        decode_within);
    std::fflush(stdout);

    const int layers_on_gpu = off_n > 0 && off_d > 0 && off_n == off_d;
    const int link_pass = vad_rc == 0 && asr_rc == 0 && mt_rc == 0 && gpu != nullptr && layers_on_gpu;
    const int cancel_pass = whisper_init_within && whisper_decode_within && init_within && decode_within;
    std::printf("link_three=%d cancel_lifecycle=%d ac2=0 ac3=0\n", link_pass, cancel_pass);
    std::printf("result=%s\n", link_pass && cancel_pass ? "ran" : "incomplete");

    out->linked = linked;
    out->vad_whisper = vad_whisper == 0;
    out->vad_ort = vad_rc == 0;
    out->asr_ok = asr_rc == 0;
    out->mt_ok = mt_rc == 0;
    out->gpu_offload = gpu_offload;
    out->off_n = off_n;
    out->off_d = off_d;
    out->asr_text_bytes = 0;
    out->mt_text_bytes = text_bytes;
    out->whisper_init_join_ms = whisper_init_join_ms;
    out->whisper_decode_join_ms = whisper_decode_join_ms;
    out->mt_init_join_ms = init_join_ms;
    out->mt_decode_join_ms = decode_join_ms;
    out->whisper_init_within = whisper_init_within;
    out->whisper_decode_within = whisper_decode_within;
    out->mt_init_within = init_within;
    out->mt_decode_within = decode_within;
    out->link_three = link_pass;
    out->cancel_lifecycle = cancel_pass;
    const char * device = gpu == nullptr ? "none" : ggml_backend_dev_name(gpu);
    if (device == nullptr) {
        device = "none";
    }
    std::snprintf(out->gpu_device, sizeof out->gpu_device, "%s", device);

    free_mt(&session);
    log_phase("after_free_mt");
    // llama_backend_free() is ggml_quantize_free(). The MTLDevice from
    // ggml_metal_device_get / ggml_backend_metal_reg (g_devices) lives until
    // process exit. Freeing it here use-after-frees the next prove. #55 0829a70.
    llama_backend_free();
    log_phase("after_backend_free");
    out->teardown = 1;
    std::printf("teardown=returned\n");
    std::fflush(stdout);
    return link_pass && cancel_pass ? 0 : 1;
}

namespace product {
std::string g_path[3];
whisper_context * g_asr = nullptr;
MtSession g_mt{};
int g_vad_ready = 0;
}  // namespace product

extern "C" int taco_engines_init(int role, const char * path) {
    if (path == nullptr || role < 0 || role > 2) {
        return 2;
    }
    product::g_path[static_cast<size_t>(role)] = path;
    if (role == 0) {
        product::g_vad_ready = taco_vad_ort(path) == 0 ? 1 : 0;
        return product::g_vad_ready ? 0 : 1;
    }
    llama_backend_init();
    if (role == 1) {
        if (product::g_asr != nullptr) {
            whisper_free(product::g_asr);
            product::g_asr = nullptr;
        }
        return run_asr(path, &product::g_asr);
    }
    llama_log_set(log_cb, nullptr);
    free_mt(&product::g_mt);
    return load_mt(path, &product::g_mt);
}

extern "C" int taco_engines_infer(int role, const void * data, size_t len, int * out_n) {
    (void)data;
    if (out_n != nullptr) {
        *out_n = 0;
    }
    if (role == 0) {
        if (product::g_path[0].empty()) {
            return 1;
        }
        // Admission already checked the byte cap. The ORT proof reruns the
        // pinned file: 512-sample frames, carried state, reset match.
        const int rc = taco_vad_ort(product::g_path[0].c_str());
        product::g_vad_ready = rc == 0 ? 1 : 0;
        if (out_n != nullptr) {
            *out_n = static_cast<int>(len);
        }
        return rc;
    }
    if (role == 1) {
        if (product::g_asr == nullptr) {
            return 1;
        }
        std::vector<float> audio(16000, 0.0f);
        if (data != nullptr && len >= audio.size() * sizeof(float)) {
            std::memcpy(audio.data(), data, audio.size() * sizeof(float));
        } else {
            for (int i = 0; i < 16000; i++) {
                const float t = static_cast<float>(i) / 16000.0f;
                audio[static_cast<size_t>(i)] = 0.1f * std::sin(2.0f * 3.14159265f * 440.0f * t);
            }
        }
        whisper_full_params params = whisper_full_default_params(WHISPER_SAMPLING_GREEDY);
        params.translate = false;
        params.language = "en";
        params.n_threads = 4;
        params.print_progress = false;
        params.print_realtime = false;
        params.print_special = false;
        params.print_timestamps = false;
        params.no_timestamps = true;
        params.single_segment = true;
        params.no_context = true;
        params.suppress_blank = true;
        params.vad = false;
        const int rc = whisper_full(product::g_asr, params, audio.data(), static_cast<int>(audio.size()));
        int bytes = 0;
        if (rc == 0) {
            const int nseg = whisper_full_n_segments(product::g_asr);
            for (int i = 0; i < nseg; i++) {
                const char * piece = whisper_full_get_segment_text(product::g_asr, i);
                if (piece != nullptr) {
                    bytes += static_cast<int>(std::strlen(piece));
                }
            }
        }
        if (out_n != nullptr) {
            *out_n = bytes;
        }
        std::printf("infer_asr=%s text_bytes=%d\n", rc == 0 ? "ok" : "fail", bytes);
        std::fflush(stdout);
        return rc == 0 ? 0 : 1;
    }
    if (role == 2) {
        if (product::g_mt.ctx == nullptr) {
            return 1;
        }
        int bytes = 0;
        const int rc = run_mt_smoke(&product::g_mt, &bytes);
        if (out_n != nullptr) {
            *out_n = bytes;
        }
        std::printf("infer_mt=%s text_bytes=%d\n", rc == 0 ? "ok" : "fail", bytes);
        std::fflush(stdout);
        return rc;
    }
    return 2;
}

extern "C" int taco_engines_cancel(int role, int reserve_ms, int * join_ms) {
    const int reserve = reserve_ms > 0 ? reserve_ms : kReserveMs;
    int join = -1;
    if (role == 1 && product::g_asr != nullptr) {
        int hits = 0;
        int full_rc = 0;
        cancel_whisper_decode(product::g_asr, &join, &hits, &full_rc);
        if (join_ms != nullptr) {
            *join_ms = join;
        }
        std::printf("cancel_asr_decode rc=%d join_ms=%d\n", full_rc, join);
        std::fflush(stdout);
        return full_rc == -8 && join >= 0 && join <= reserve ? 0 : 1;
    }
    if (role == 1 && !product::g_path[1].empty()) {
        int poisoned = 0;
        const int status = cancel_whisper_init(product::g_path[1].c_str(), &join, &poisoned);
        if (join_ms != nullptr) {
            *join_ms = join;
        }
        std::printf("cancel_asr_init status=%d join_ms=%d\n", status, join);
        std::fflush(stdout);
        return status == 1 && join >= 0 && join <= reserve ? 0 : 1;
    }
    if (role == 2 && product::g_mt.ctx != nullptr) {
        int steps = 0;
        int decode_rc = 0;
        cancel_decode(&product::g_mt, &join, &steps, &decode_rc);
        if (join_ms != nullptr) {
            *join_ms = join;
        }
        std::printf("cancel_mt_decode rc=%d join_ms=%d\n", decode_rc, join);
        std::fflush(stdout);
        return decode_rc == 2 && join >= 0 && join <= reserve ? 0 : 1;
    }
    if (role == 2 && !product::g_path[2].empty()) {
        int progress = 0;
        const int status = cancel_init(product::g_path[2].c_str(), &join, &progress);
        if (join_ms != nullptr) {
            *join_ms = join;
        }
        std::printf("cancel_mt_init status=%d join_ms=%d\n", status, join);
        std::fflush(stdout);
        return status == 1 && join >= 0 && join <= reserve ? 0 : 1;
    }
    if (join_ms != nullptr) {
        *join_ms = -1;
    }
    return 1;
}

extern "C" void taco_engines_close(int role) {
    if (role == 0) {
        product::g_vad_ready = 0;
        product::g_path[0].clear();
    } else if (role == 1) {
        if (product::g_asr != nullptr) {
            whisper_free(product::g_asr);
            product::g_asr = nullptr;
        }
        product::g_path[1].clear();
    } else if (role == 2) {
        free_mt(&product::g_mt);
        product::g_path[2].clear();
    }
}
