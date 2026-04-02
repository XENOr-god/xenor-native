cmake_minimum_required(VERSION 3.20)

project(xenor_native_cpp LANGUAGES CXX)

add_library(
  xenor_native_cpp
  STATIC
  src/xenor_sim.cpp
  src/xenor_state.cpp
  src/xenor_math.cpp
  src/xenor_checksum.cpp
)

target_include_directories(
  xenor_native_cpp
  PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
  PRIVATE
  ${CMAKE_CURRENT_SOURCE_DIR}/src
)

target_compile_features(xenor_native_cpp PUBLIC cxx_std_20)

if(CMAKE_CXX_COMPILER_ID MATCHES "Clang|GNU")
  target_compile_options(xenor_native_cpp PRIVATE -Wall -Wextra -Wpedantic)
endif()
