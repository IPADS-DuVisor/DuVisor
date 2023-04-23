cmake_minimum_required(VERSION 3.14)

set(CMAKE_VERBOSE_MAKEFILE on)

project(virt-test-img C ASM)

set(CROSS_COMPILE "riscv64-linux-gnu-")
set(CMAKE_C_COMPILER "${CROSS_COMPILE}gcc")
set(CMAKE_ASM_COMPILER "${CROSS_COMPILE}gcc")

add_compile_options(-Wall -fPIC -nostdlib -nostartfiles -ffreestanding -Werror -nostdinc)
add_compile_definitions($<$<COMPILE_LANGUAGE:ASM>:__ASM__>)
add_link_options(-T ${CMAKE_CURRENT_LIST_DIR}/vm.ld)

include_directories(..)

macro(add_vm_test_img IMG_NAME CODE_ENTRY)
	add_executable(${IMG_NAME}.img ${ARGN})
	target_link_options(${IMG_NAME}.img PUBLIC -e ${CODE_ENTRY})
	target_compile_definitions(${IMG_NAME}.img PUBLIC __FILENAME__=${IMG_NAME})
endmacro(add_vm_test_img)

FILE(GLOB image_files "*.S")
foreach(image_file ${image_files})
	get_filename_component(image  ${image_file} NAME_WE)
	add_vm_test_img(${image} ${image} ${image_file})
endforeach()
