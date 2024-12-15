cmake_minimum_required(VERSION 3.10)

project(aoc2024)

include_directories(${CMAKE_INCLUDE_PATH})

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED YES)

find_package(fmt REQUIRED)
find_library(GTEST_LIB gtest REQUIRED)
find_library(GTEST_MAIN_LIB gtest_main REQUIRED)

if (CMAKE_GENERATOR STREQUAL "Ninja" OR CMAKE_GENERATOR STREQUAL "Unix Makefiles")
    execute_process(
        COMMAND ${CMAKE_COMMAND} -E create_symlink
        ${CMAKE_BINARY_DIR}/compile_commands.json
        ${CMAKE_SOURCE_DIR}/compile_commands.json
    )
endif()

add_executable(day1 day1/main.cpp)
target_link_libraries(day1
    PRIVATE
        fmt::fmt
)

add_executable(day2 day2/main.cpp)
target_link_libraries(day2
    PRIVATE
        fmt::fmt
)

add_executable(day3 day3/main.cpp day3/solution.cpp)
target_link_libraries(day3
    PRIVATE
        fmt::fmt
)

add_executable(day3_test day3/solution.cpp day3/test.cpp)
target_link_libraries(day3_test PRIVATE ${GTEST_LIB} ${GTEST_MAIN_LIB} fmt::fmt)

enable_testing()
add_test(NAME Day3Test COMMAND day3/test.cpp)
