#
# Copyright 2019, DornerWorks
#
# This software may be distributed and modified according to the terms of
# the BSD 2-Clause license. Note that NO WARRANTY is provided.
# See "LICENSE_BSD2.txt" for details.
#
# @TAG(DORNERWORKS_BSD)
#

cmake_minimum_required(VERSION 3.7.2)

project(root_app C)

set(configure_string "")

file(
    GLOB
        static
        src/*.c
)

MakeCPIO(archive.o "$<TARGET_FILE:root_app>")

add_executable(root_app EXCLUDE_FROM_ALL ${static})
target_include_directories(root_app PRIVATE "include")
target_link_libraries(
    root_app
    PUBLIC
        sel4_autoconf
        muslc
        sel4
        sel4runtime
        sel4allocman
        sel4vka
        sel4utils
        sel4rpc
        sel4test
        sel4platsupport
        sel4muslcsys
        rust_app
)
target_compile_options(root_app PRIVATE -Werror -g)

# Set this image as the rootserver
DeclareRootserver(root_app)
