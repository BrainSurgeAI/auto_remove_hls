cmake_minimum_required(VERSION 3.24.1)

project(Helper VERSION 0.0.1)
set(CMAKE_CXX_STANDARD 17)

find_package(Catch2 REQUIRED)
add_library(${PROJECT_NAME} helper.cc)

add_executable(foo test.cc)
target_link_libraries(foo PRIVATE Catch2::Catch2WithMain)

include(CTest)
include(Catch)
catch_discover_tests(foo)