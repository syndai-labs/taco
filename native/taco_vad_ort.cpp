// Incremental Silero on the pinned ONNX, inside this Mach-O.
// CPU execution provider only. This is not a Metal VAD and not whisper_full.

#include <array>
#include <cmath>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <string>
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

}  // namespace

#ifdef TACO_HAVE_ORT

int taco_vad_ort(const char * path) {
    try {
        const OrtApiBase * api = OrtGetApiBase();
        Ort::Env env(ORT_LOGGING_LEVEL_ERROR, "taco");
        Ort::SessionOptions opts;
        opts.SetIntraOpNumThreads(1);
        opts.SetInterOpNumThreads(1);
        opts.SetGraphOptimizationLevel(GraphOptimizationLevel::ORT_ENABLE_EXTENDED);
        Ort::Session session(env, path, opts);

        Ort::AllocatorWithDefaultOptions allocator;
        const size_t n_in = session.GetInputCount();
        const size_t n_out = session.GetOutputCount();
        std::printf(
            "vad_ort_session=ok version=%s inputs=%zu outputs=%zu ep=CPU\n",
            api == nullptr || api->GetVersionString() == nullptr ? "unknown" : api->GetVersionString(),
            n_in,
            n_out);
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
        std::printf("vad_ort=%s device=cpu\n", match ? "ok" : "fail");
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
    std::printf("vad_ort=not_linked\n");
    std::fflush(stdout);
    return 1;
}

#endif
