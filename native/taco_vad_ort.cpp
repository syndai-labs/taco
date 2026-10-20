// Incremental Silero on the pinned ONNX, inside this Mach-O.
// The device token is read from MLComputePlan, not from the provider option.
// Silero's compute is inside If subgraphs. ORT 1.31 CoreML skips those unless
// EnableOnSubgraphs=1. There is still no CoreML builder for LSTM or If, so
// device= stays cpu until a Mac plan names GPU or ANE. This is not whisper_full.

#include <array>
#include <cmath>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <filesystem>
#include <string>
#include <unordered_map>
#include <unistd.h>
#include <vector>

#ifdef TACO_HAVE_ORT
#include "onnxruntime_cxx_api.h"
#endif

namespace {

void print_dims(const std::vector<int64_t> & dims) {
    std::printf("rank=%zu dims=", dims.size());
    for (size_t i = 0; i < dims.size(); i++) {
        if (i != 0) {
            std::printf(",");
        }
        std::printf("%lld", static_cast<long long>(dims[i]));
    }
}

#ifdef TACO_HAVE_ORT
std::string make_cache_dir() {
    const char * tmp = std::getenv("TMPDIR");
    std::string dir = tmp != nullptr && tmp[0] != '\0' ? tmp : "/tmp";
    if (dir.back() != '/') {
        dir.push_back('/');
    }
    dir += "taco-vad-coreml-XXXXXX";
    std::vector<char> buf(dir.begin(), dir.end());
    buf.push_back('\0');
    if (mkdtemp(buf.data()) == nullptr) {
        return {};
    }
    return std::string(buf.data());
}

struct CacheDir {
    std::string path;
    ~CacheDir() {
        if (!path.empty()) {
            std::error_code err;
            std::filesystem::remove_all(path, err);
        }
    }
};

const char * hw_kind(OrtHardwareDeviceType type) {
    switch (type) {
    case OrtHardwareDeviceType_CPU:
        return "cpu";
    case OrtHardwareDeviceType_GPU:
        return "gpu";
    case OrtHardwareDeviceType_NPU:
        return "npu";
    }
    return "other";
}

#endif

}  // namespace

#ifdef TACO_HAVE_ORT_COREML
extern "C" void taco_vad_profile_cache(const char * cache_dir, int * cpu, int * gpu, int * ane, int * other);
#endif

#ifdef TACO_HAVE_ORT

int taco_vad_ort(const char * path) {
    try {
        const OrtApiBase * api = OrtGetApiBase();
        Ort::Env env(ORT_LOGGING_LEVEL_WARNING, "taco");
        for (const std::string & name : Ort::GetAvailableProviders()) {
            std::printf("vad_ort_provider=%s\n", name.c_str());
        }
        try {
            for (const Ort::ConstHardwareDevice & hw : env.GetHardwareDevices()) {
                const char * vendor = hw.Vendor();
                std::printf("vad_ort_hw type=%s vendor=%s\n", hw_kind(hw.Type()), vendor == nullptr ? "" : vendor);
            }
        } catch (const Ort::Exception & err) {
            std::printf("vad_ort_hw=fail msg=%s\n", err.what());
        }
        std::fflush(stdout);

        CacheDir cache;
        bool requested_coreml = false;
        Ort::SessionOptions opts;
        opts.SetIntraOpNumThreads(1);
        opts.SetInterOpNumThreads(1);
        opts.SetGraphOptimizationLevel(GraphOptimizationLevel::ORT_ENABLE_EXTENDED);
#ifdef TACO_HAVE_ORT_COREML
        cache.path = make_cache_dir();
        if (!cache.path.empty()) {
            try {
                // ponytail: EnableOnSubgraphs only. ORT 1.31 CoreML has no LSTM or If
                // builder, so the recurrent body stays on CPU until a Mac plan says otherwise.
                const std::unordered_map<std::string, std::string> coreml = {
                    {"MLComputeUnits", "CPUAndGPU"},
                    {"ModelFormat", "MLProgram"},
                    {"ProfileComputePlan", "1"},
                    {"ModelCacheDirectory", cache.path},
                    {"EnableOnSubgraphs", "1"},
                };
                opts.AppendExecutionProvider("CoreML", coreml);
                requested_coreml = true;
                std::printf("vad_ort_request ep=CoreML units=CPUAndGPU format=MLProgram subgraphs=1\n");
            } catch (const Ort::Exception & err) {
                std::printf("vad_ort_request=fail ep=CoreML msg=%s\n", err.what());
                opts = Ort::SessionOptions();
                opts.SetIntraOpNumThreads(1);
                opts.SetInterOpNumThreads(1);
                opts.SetGraphOptimizationLevel(GraphOptimizationLevel::ORT_ENABLE_EXTENDED);
            }
        } else {
            std::printf("vad_ort_request=fail ep=CoreML msg=cache_dir\n");
        }
#else
        std::printf("vad_ort_request ep=CPU reason=coreml_provider_not_linked device=cpu\n");
#endif
        opts.AddConfigEntry("session.record_ep_graph_assignment_info", "1");
        std::fflush(stdout);
        Ort::Session session(env, path, opts);

        Ort::AllocatorWithDefaultOptions allocator;
        const size_t n_in = session.GetInputCount();
        const size_t n_out = session.GetOutputCount();
        std::printf(
            "vad_ort_session=ok version=%s inputs=%zu outputs=%zu\n",
            api == nullptr || api->GetVersionString() == nullptr ? "unknown" : api->GetVersionString(),
            n_in,
            n_out);
        int coreml_nodes = 0;
        int cpu_nodes = 0;
        try {
            const std::vector<Ort::ConstEpAssignedSubgraph> graphs = session.GetEpGraphAssignmentInfo();
            if (graphs.empty()) {
                std::printf("vad_ort_assign ep=none nodes=0\n");
            }
            for (const Ort::ConstEpAssignedSubgraph & graph : graphs) {
                const std::string ep = graph.GetEpName();
                const size_t nodes = graph.GetNodes().size();
                std::printf("vad_ort_assign ep=%s nodes=%zu\n", ep.c_str(), nodes);
                if (ep.find("CoreML") != std::string::npos) {
                    coreml_nodes += static_cast<int>(nodes);
                }
                if (ep.find("CPU") != std::string::npos) {
                    cpu_nodes += static_cast<int>(nodes);
                }
            }
        } catch (const Ort::Exception & err) {
            std::printf("vad_ort_assign=fail msg=%s\n", err.what());
        }
        try {
            const std::vector<Ort::ConstEpDevice> inputs = session.GetEpDeviceForInputs();
            for (size_t i = 0; i < inputs.size(); i++) {
                const char * ep = inputs[i].EpName();
                const char * kind = hw_kind(inputs[i].Device().Type());
                std::printf(
                    "vad_ort_input_ep index=%zu ep=%s hw=%s\n",
                    i,
                    ep == nullptr ? "" : ep,
                    kind);
            }
        } catch (const Ort::Exception & err) {
            std::printf("vad_ort_input_ep=fail msg=%s\n", err.what());
        }
        std::vector<std::string> in_names;
        std::vector<std::string> out_names;
        for (size_t i = 0; i < n_in; i++) {
            auto name = session.GetInputNameAllocated(i, allocator);
            // Tensor type info is unowned and dies with the TypeInfo temporary.
            Ort::TypeInfo type_info = session.GetInputTypeInfo(i);
            auto info = type_info.GetTensorTypeAndShapeInfo();
            in_names.emplace_back(name.get());
            std::printf("vad_ort_in name=%s type=%d ", name.get(), static_cast<int>(info.GetElementType()));
            print_dims(info.GetShape());
            std::printf("\n");
        }
        for (size_t i = 0; i < n_out; i++) {
            auto name = session.GetOutputNameAllocated(i, allocator);
            Ort::TypeInfo type_info = session.GetOutputTypeInfo(i);
            auto info = type_info.GetTensorTypeAndShapeInfo();
            out_names.emplace_back(name.get());
            std::printf("vad_ort_out name=%s type=%d ", name.get(), static_cast<int>(info.GetElementType()));
            print_dims(info.GetShape());
            std::printf("\n");
        }
        std::fflush(stdout);
        if (in_names.size() != 3 || out_names.size() != 2) {
            std::printf("vad_ort=fail reason=io_count\n");
            return 1;
        }

        constexpr int kSamples = 512;
        constexpr int kState = 2 * 1 * 128;
        std::vector<float> audio(kSamples, 0.0f);
        std::vector<float> state(kState, 0.0f);
        int64_t sr = 16000;
        const std::array<int64_t, 2> audio_shape{1, kSamples};
        const std::array<int64_t, 3> state_shape{2, 1, 128};

        auto run_frame = [&](int index, float * prob_out, double * state_l2) -> int {
            Ort::MemoryInfo memory = Ort::MemoryInfo::CreateCpu(OrtArenaAllocator, OrtMemTypeDefault);
            std::array<Ort::Value, 3> inputs = {
                Ort::Value::CreateTensor<float>(memory, audio.data(), audio.size(), audio_shape.data(), audio_shape.size()),
                Ort::Value::CreateTensor<float>(memory, state.data(), state.size(), state_shape.data(), state_shape.size()),
                Ort::Value::CreateTensor<int64_t>(memory, &sr, 1, nullptr, 0),
            };
            std::array<const char *, 3> in_ptrs = {in_names[0].c_str(), in_names[1].c_str(), in_names[2].c_str()};
            std::array<const char *, 2> out_ptrs = {out_names[0].c_str(), out_names[1].c_str()};
            auto outputs = session.Run(Ort::RunOptions{nullptr}, in_ptrs.data(), inputs.data(), inputs.size(), out_ptrs.data(), out_ptrs.size());
            if (outputs.size() != 2 || !outputs[0].IsTensor() || !outputs[1].IsTensor()) {
                std::printf("vad_ort=fail reason=output_missing frame=%d\n", index);
                return 1;
            }
            const auto out_info = outputs[0].GetTensorTypeAndShapeInfo();
            const auto st_info = outputs[1].GetTensorTypeAndShapeInfo();
            if (out_info.GetElementCount() < 1 || st_info.GetElementCount() != state.size()) {
                std::printf(
                    "vad_ort=fail reason=output_shape frame=%d out_n=%zu state_n=%zu\n",
                    index,
                    out_info.GetElementCount(),
                    st_info.GetElementCount());
                return 1;
            }
            const float prob = outputs[0].GetTensorData<float>()[0];
            const float * next = outputs[1].GetTensorData<float>();
            std::memcpy(state.data(), next, state.size() * sizeof(float));
            double l2 = 0.0;
            for (float v : state) {
                l2 += static_cast<double>(v) * static_cast<double>(v);
            }
            l2 = std::sqrt(l2);
            std::printf(
                "vad_ort_frame=%d samples=%d sr=16000 prob=%.6f state_l2=%.6f\n",
                index,
                kSamples,
                static_cast<double>(prob),
                l2);
            std::fflush(stdout);
            *prob_out = prob;
            *state_l2 = l2;
            return 0;
        };

        float first_prob = 0.0f;
        double first_l2 = 0.0;
        if (run_frame(0, &first_prob, &first_l2) != 0) {
            return 1;
        }
        for (int i = 0; i < kSamples; i++) {
            const float t = static_cast<float>(i) / 16000.0f;
            audio[static_cast<size_t>(i)] = 0.2f * std::sin(2.0f * 3.14159265f * 440.0f * t);
        }
        float second_prob = 0.0f;
        double second_l2 = 0.0;
        if (run_frame(1, &second_prob, &second_l2) != 0) {
            return 1;
        }

        std::fill(state.begin(), state.end(), 0.0f);
        std::fill(audio.begin(), audio.end(), 0.0f);
        float reset_prob = 0.0f;
        double reset_l2 = 0.0;
        if (run_frame(2, &reset_prob, &reset_l2) != 0) {
            return 1;
        }
        const int match = std::fabs(reset_prob - first_prob) < 1.0e-5f && std::fabs(reset_l2 - first_l2) < 1.0e-4;
        std::printf(
            "vad_ort_reset=1 match=%d carried_state=%d window=512 state=2x1x128\n",
            match ? 1 : 0,
            second_l2 > 0.0 ? 1 : 0);
        int plan_cpu = 0;
        int plan_gpu = 0;
        int plan_ane = 0;
        int plan_other = 0;
#ifdef TACO_HAVE_ORT_COREML
        if (requested_coreml) {
            taco_vad_profile_cache(cache.path.c_str(), &plan_cpu, &plan_gpu, &plan_ane, &plan_other);
        }
#else
        (void)requested_coreml;
#endif
        const char * device = "cpu";
        if (plan_gpu > 0) {
            device = "MLGPUComputeDevice";
        } else if (plan_ane > 0) {
            device = "MLNeuralEngineComputeDevice";
        } else if (plan_cpu > 0) {
            device = "MLCPUComputeDevice";
        }
        std::printf(
            "vad_ort=%s device=%s plan_gpu=%d plan_cpu=%d plan_ane=%d plan_other=%d coreml_nodes=%d cpu_nodes=%d\n",
            match ? "ok" : "fail",
            device,
            plan_gpu,
            plan_cpu,
            plan_ane,
            plan_other,
            coreml_nodes,
            cpu_nodes);
        std::fflush(stdout);
        return match ? 0 : 1;
    } catch (const Ort::Exception & err) {
        std::printf("vad_ort=fail reason=exception msg=%s\n", err.what());
        std::fflush(stdout);
        return 1;
    }
}

#else

int taco_vad_ort(const char *) {
    std::printf("vad_ort=not_linked device=cpu reason=coreml_ep_not_in_this_binary\n");
    std::fflush(stdout);
    return 1;
}

#endif
