cmake_minimum_required(VERSION 4.0)

find_package(cmake-bare REQUIRED PATHS node_modules/cmake-bare)
find_package(cmake-cargo REQUIRED PATHS node_modules/cmake-cargo)

project(bare_addon C)

add_crate()

add_bare_module(bare_addon)

target_sources(
  ${bare_addon}
  PRIVATE
    binding.c
)

target_link_libraries(
  ${bare_addon}
  PRIVATE
    bare_addon_rust
)
