cmake_minimum_required(VERSION 3.22.2)

project(Helper VERSION 0.0.1)
set(CMAKE_CXX_STANDARD 17)

#find_package(Catch2 REQUIRED)
add_library(${PROJECT_NAME} helper.cc)

#add_executable(foo test.cc)
#target_link_libraries(foo PRIVATE Catch2::Catch2WithMain)

#include(CTest)
#include(Catch)
#catch_discover_tests(foo)
include(FetchContent)
FetchContent_Declare(
  googletest
  GIT_REPOSITORY https://github.com/google/googletest.git
  GIT_TAG release-1.12.1
)
# For Windows: Prevent overriding the parent project's compiler/linker settings
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

enable_testing()

add_executable(
  hello_test
  test.cc
)
target_link_libraries(
  hello_test
  GTest::gtest_main
)

include(GoogleTest)
gtest_discover_tests(hello_test)