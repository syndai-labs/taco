cmake_minimum_required(VERSION 3.20)
# One static Mach-O link of whisper.cpp and llama.cpp on a shared ggml.
# Engines are not vendored. Point LLAMA_SRC and WHISPER_SRC at pinned checkouts.
project(taco_m0_link LANGUAGES C CXX OBJC OBJCXX)

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

add_library(taco_engines STATIC taco_engines.cpp)
target_link_libraries(taco_engines PUBLIC whisper llama)
target_include_directories(taco_engines PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

add_executable(taco_link_probe taco_link_probe.cpp)
target_link_libraries(taco_link_probe PRIVATE taco_engines)
