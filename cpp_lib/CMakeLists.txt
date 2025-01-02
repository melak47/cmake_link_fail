cmake_minimum_required(VERSION 3.29)
project(cpp_lib)
find_program(CXXBRIDGE cxxbridge)
set(BRIDGE_FILES 
    "${CMAKE_CURRENT_BINARY_DIR}/cpp_lib/bridge.rs.h"
    "${CMAKE_CURRENT_BINARY_DIR}/cpp_lib/bridge.rs.cc"
    "${CMAKE_CURRENT_BINARY_DIR}/rust/cxx.h"
)
make_directory(${CMAKE_CURRENT_BINARY_DIR}/include/cpp_lib)
make_directory(${CMAKE_CURRENT_BINARY_DIR}/include/rust)
include_directories(${CMAKE_CURRENT_BINARY_DIR}/include)
set(BRIDGE_FILE ${CMAKE_CURRENT_SOURCE_DIR}/../src/bridge.rs)
add_custom_target(
    Bridge
    COMMAND ${CXXBRIDGE} ${BRIDGE_FILE} > ${CMAKE_CURRENT_BINARY_DIR}/include/cpp_lib/bridge.rs.cc
    COMMAND ${CXXBRIDGE} --header ${BRIDGE_FILE} > ${CMAKE_CURRENT_BINARY_DIR}/include/cpp_lib/bridge.rs.h
    COMMAND ${CXXBRIDGE} --header > ${CMAKE_CURRENT_BINARY_DIR}/include/rust/cxx.h
    DEPENDS ${BRIDGE_FILE}
    COMMENT "generate the bridge files"
)

add_subdirectory(liba)
add_subdirectory(libb)
