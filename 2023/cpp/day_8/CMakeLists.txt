project(cpp_project)
cmake_minimum_required(VERSION 3.5)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
execute_process(COMMAND ln -sf ${CMAKE_BINARY_DIR}/compile_commands.json ${CMAKE_SOURCE_DIR}/compile_commands.json)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_COMPILER g++)

find_package(GTest REQUIRED)
find_package(FTXUI REQUIRED)

add_executable(cpp_project main.cpp)
target_link_libraries(cpp_project ftxui::component)

add_executable(tests test.cpp)
target_link_libraries(tests GTest::GTest GTest::Main)

add_executable(day_8 day_8.cpp)
