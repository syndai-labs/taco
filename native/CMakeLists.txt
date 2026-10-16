cmake_minimum_required(VERSION 3.20)
# Product static link of whisper.cpp and llama.cpp on one ggml, plus static
# ONNX Runtime when a local CPU archive build is present.
# Engines are not vendored and nothing is downloaded. No separate probe executable.
# Static libs, Metal shaders embedded, no shared engine library.
project(taco_engines LANGUAGES C CXX OBJC OBJCXX)

if(NOT LLAMA_SRC OR NOT WHISPER_SRC)
    message(FATAL_ERROR "Set LLAMA_SRC and WHISPER_SRC to the pinned engine checkouts")
endif()

set(BUILD_SHARED_LIBS OFF CACHE BOOL "" FORCE)
set(CMAKE_POSITION_INDEPENDENT_CODE OFF)
set(CMAKE_OSX_DEPLOYMENT_TARGET "14.2" CACHE STRING "" FORCE)

set(GGML_METAL ON CACHE BOOL "" FORCE)
set(GGML_METAL_EMBED_LIBRARY ON CACHE BOOL "" FORCE)
set(GGML_NATIVE ON CACHE BOOL "" FORCE)
set(GGML_BLAS ON CACHE BOOL "" FORCE)
set(GGML_METAL_NDEBUG ON CACHE BOOL "" FORCE)

set(LLAMA_STANDALONE OFF CACHE BOOL "" FORCE)
set(LLAMA_BUILD_COMMON OFF CACHE BOOL "" FORCE)
set(LLAMA_BUILD_TESTS OFF CACHE BOOL "" FORCE)
set(LLAMA_BUILD_TOOLS OFF CACHE BOOL "" FORCE)
set(LLAMA_BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)
set(LLAMA_BUILD_SERVER OFF CACHE BOOL "" FORCE)
set(LLAMA_BUILD_APP OFF CACHE BOOL "" FORCE)
set(LLAMA_BUILD_MTMD OFF CACHE BOOL "" FORCE)
set(LLAMA_OPENSSL OFF CACHE BOOL "" FORCE)
set(LLAMA_CURL OFF CACHE BOOL "" FORCE)
set(LLAMA_BUILD_UI OFF CACHE BOOL "" FORCE)

set(WHISPER_STANDALONE OFF CACHE BOOL "" FORCE)
set(WHISPER_BUILD_TESTS OFF CACHE BOOL "" FORCE)
set(WHISPER_BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)
set(WHISPER_BUILD_SERVER OFF CACHE BOOL "" FORCE)
set(WHISPER_SDL2 OFF CACHE BOOL "" FORCE)
set(WHISPER_CURL OFF CACHE BOOL "" FORCE)
set(WHISPER_COREML OFF CACHE BOOL "" FORCE)

# Pinned whisper.cpp deletes a failed init context without freeing backend
# buffers. That leaves Metal residency sets alive until process exit.
set(_whisper_cpp "${WHISPER_SRC}/src/whisper.cpp")
file(READ "${_whisper_cpp}" _whisper_src)
if(NOT _whisper_src MATCHES "taco: release Metal buffers")
    execute_process(
        COMMAND patch -p1 --forward --input=${CMAKE_CURRENT_SOURCE_DIR}/patches/whisper-free-buffers-on-init-fail.patch
        WORKING_DIRECTORY "${WHISPER_SRC}"
        RESULT_VARIABLE _whisper_patch_rc
    )
    if(NOT _whisper_patch_rc EQUAL 0)
        message(FATAL_ERROR "whisper init-fail patch did not apply (${_whisper_patch_rc})")
    endif()
endif()

# llama.cpp adds ggml. whisper.cpp skips its own ggml when that target exists.
add_subdirectory("${LLAMA_SRC}" "${CMAKE_BINARY_DIR}/llama" EXCLUDE_FROM_ALL)
add_subdirectory("${WHISPER_SRC}" "${CMAKE_BINARY_DIR}/whisper" EXCLUDE_FROM_ALL)

add_library(taco_engines STATIC taco_engines.cpp taco_vad_ort.cpp taco_runtime.cpp)
target_link_libraries(taco_engines PUBLIC whisper llama)
target_include_directories(taco_engines PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")
set_target_properties(taco_engines PROPERTIES CXX_STANDARD 17 CXX_STANDARD_REQUIRED YES)

# Static ONNX Runtime CPU archives from a local build. Absent archives leave
# VAD without ORT. Do not point this at a shared ONNX Runtime library.
set(ORT_ROOT "" CACHE PATH "Local ONNX Runtime build root")
if(NOT ORT_ROOT)
    set(ORT_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../.cache/m0-onnx")
endif()
set(ORT_RELEASE "${ORT_ROOT}/src/build/MacOS/Release")
set(ORT_SESSION "${ORT_RELEASE}/libonnxruntime_session.a")
set(ORT_LINK_FILE "${CMAKE_BINARY_DIR}/ort-link.txt")
if(EXISTS "${ORT_SESSION}")
    target_compile_definitions(taco_engines PRIVATE TACO_HAVE_ORT=1)
    target_include_directories(taco_engines PRIVATE
        "${ORT_ROOT}/src/include/onnxruntime/core/session")
    set(ORT_FORCE
        "${ORT_SESSION}"
        "${ORT_RELEASE}/libonnxruntime_optimizer.a"
        "${ORT_RELEASE}/libonnxruntime_providers.a"
        "${ORT_RELEASE}/libonnxruntime_lora.a"
        "${ORT_RELEASE}/libonnxruntime_framework.a"
        "${ORT_RELEASE}/libonnxruntime_graph.a"
        "${ORT_RELEASE}/libonnxruntime_util.a"
        "${ORT_RELEASE}/libonnxruntime_mlas.a"
        "${ORT_RELEASE}/libonnxruntime_common.a"
        "${ORT_RELEASE}/libonnxruntime_flatbuffers.a"
        "${ORT_RELEASE}/_deps/onnx-build/libonnx.a"
        "${ORT_RELEASE}/_deps/protobuf-build/libprotobuf-lite.a"
        "${ORT_RELEASE}/_deps/protobuf-build/third_party/utf8_range/libutf8_validity.a"
        "${ORT_RELEASE}/_deps/kleidiai-build/libkleidiai.a"
        "${ORT_RELEASE}/_deps/pytorch_cpuinfo-build/libcpuinfo.a"
        "${ORT_RELEASE}/_deps/re2-build/libre2.a"
        "${ORT_RELEASE}/model_package/libmodel_package.a"
    )
    file(GLOB_RECURSE ORT_ABSL "${ORT_RELEASE}/_deps/abseil_cpp-build/*.a")
    list(APPEND ORT_FORCE ${ORT_ABSL})
    # libonnx.a already contains the protobuf objects in libonnx_proto.a.
    # libflatbuffers.a shares template symbols with libonnxruntime_flatbuffers.a,
    # so it is a normal archive and only the members that resolve undefined symbols are pulled.
    set(ORT_NORMAL "${ORT_RELEASE}/_deps/flatbuffers-build/libflatbuffers.a")
    if(NOT EXISTS "${ORT_NORMAL}")
        message(FATAL_ERROR "missing static ORT archive ${ORT_NORMAL}")
    endif()
    set(_ort_lines "normal\t${ORT_NORMAL}\n")
    foreach(lib ${ORT_FORCE})
        if(NOT EXISTS "${lib}")
            message(FATAL_ERROR "missing static ORT archive ${lib}")
        endif()
        string(APPEND _ort_lines "force\t${lib}\n")
    endforeach()
    file(WRITE "${ORT_LINK_FILE}" "${_ort_lines}")
    message(STATUS "static ORT archives: ${ORT_RELEASE}")
else()
    file(WRITE "${ORT_LINK_FILE}" "")
    message(STATUS "ORT static archives not found at ${ORT_SESSION}")
endif()
