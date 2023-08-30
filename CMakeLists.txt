cmake_minimum_required(VERSION 3.15.0)

set(CMAKE_VERBOSE_MAKEFILE on)

if (CMAKE_HOST_WIN32 OR CMAKE_HOST_APPLE)
  set(TOOLCHAIN_PREFIX "x86_64-elf-")
elseif (CMAKE_HOST_WIN32)
  set(TOOLCHAIN_PREFIX "x86_64-linux-gnu-")
endif()

set(CMAKE_C_COMPILER "${TOOLCHAIN_PREFIX}gcc")
set(CMAKE_C_FLAGS "-g -c -O0 -m32 -fno-pie -fno-stack-protector -fno-asynchronous-unwind-tables")

set(CMAKE_ASM_COMPILER "${TOOLCHAIN_PREFIX}gcc")
set(CMAKE_ASM_FLAGS "-m32 -g")
set(CMAKE_ASM_SOURCE_FILE_EXTENSIONS "asm")

set(LINKER_TOOL "${TOOLCHAIN_PREFIX}ld")

set(OBJCOPY_TOOL "${TOOLCHAIN_PREFIX}objcopy")
set(OBJDUMP_TOOL "${TOOLCHAIN_PREFIX}objdump")
set(READELF_TOOL "${TOOLCHAIN_PREFIX}readelf")

add_definitions(-DNET_DRIVER_RTL8139)
add_definitions(-DSYS_PLAT_X86OS)

project(os LANGUAGES C)
enable_language(ASM)

include_directories(
  ${PROJECT_SOURCE_DIR}/src
  ${PROJECT_SOURCE_DIR}/src/kernel/include())

add_subdirectory(./source/boot)
add_subdirectory(./source/init)
add_subdirectory(./source/kernel)
add_subdirectory(./source/loader)
add_subdirectory(./source/shell)

add_dependencies(init app)
add_dependencies(shell app)
add_dependencies(kernel app)
add_dependencies(loop app)
