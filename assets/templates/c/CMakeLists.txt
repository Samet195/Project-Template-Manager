cmake_minimum_required(VERSION 3.16)

project(
    "C Project"
    VERSION 1.0
    DESCRIPTION "C Template Project"
    LANGUAGES C
)

set(CMAKE_C_STANDARD 17)

include_directories(./include)

file(GLOB SRCS ./src/*)

add_executable(main ${SRCS})
