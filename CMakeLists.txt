#
# Copyright 2019, Data61
# Commonwealth Scientific and Industrial Research Organisation (CSIRO)
# ABN 41 687 119 230.
#
# This software may be distributed and modified according to the terms of
# the BSD 2-Clause license. Note that NO WARRANTY is provided.
# See "LICENSE_BSD2.txt" for details.
#
# @TAG(DATA61_BSD)
#

cmake_minimum_required(VERSION 3.7.2)

list(APPEND CMAKE_MODULE_PATH ${sel4-application_SOURCE_DIR}/tools/CMakeRust/cmake)

enable_language(Rust)
include(CMakeCargo)

add_subdirectory(apps/root_app)
add_subdirectory(apps/rust_app)

if(KernelSel4ArchX86_64)
    SetSimulationScriptProperty(MEM_SIZE "3G")
endif()

GenerateSimulateScript()
