cmake_minimum_required(VERSION 4.0)

find_package(cmake-bare REQUIRED PATHS node_modules/cmake-bare)

project(bare_addon C)

add_bare_module(bare_addon)

target_sources(
  ${bare_addon}
  PRIVATE
    binding.c
)

add_library(bare_addon_rust STATIC IMPORTED)

set_target_properties(
  bare_addon_rust
  PROPERTIES
  IMPORTED_LOCATION ${CMAKE_CURRENT_LIST_DIR}/target/debug/libbare_addon_rust.a
)

target_link_libraries(
  ${bare_addon}
  PRIVATE
    bare_addon_rust
)
