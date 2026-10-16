cmake_minimum_required(VERSION 3.20)
# Product static link of whisper.cpp and llama.cpp on one ggml.
# Engines are not vendored and nothing is downloaded. No probe executable.
# Same Mach-O rules as the metal-link probe: static libs, Metal shaders embedded,
# no shared engine library, no helper executable.
# A shared libonnxruntime dylib or sidecar is not a ship path. ORT, if linked,
# stays static archives. Dynamic dependencies stay system frameworks (ADR 0004).
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

# llama.cpp adds ggml. whisper.cpp skips its own ggml when that target exists.
add_subdirectory("${LLAMA_SRC}" "${CMAKE_BINARY_DIR}/llama" EXCLUDE_FROM_ALL)
add_subdirectory("${WHISPER_SRC}" "${CMAKE_BINARY_DIR}/whisper" EXCLUDE_FROM_ALL)

add_library(taco_engines STATIC taco_engines.cpp)
target_link_libraries(taco_engines PUBLIC whisper llama)
target_include_directories(taco_engines PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")
