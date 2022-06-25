cmake_minimum_required(VERSION 3.16)

project(
    "C++ Project"
    VERSION 1.0
    DESCRIPTION "C++ Template Project"
    LANGUAGES CXX
)

set(CMAKE_CXX_STANDARD 17)

include_directories(./include)

file(GLOB SRCS ./src/*)

add_executable(main ${SRCS})
