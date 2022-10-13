cmake_minimum_required (VERSION 3.8)

project(soulband)
set (CMAKE_CXX_STANDARD 17)

file(
    GLOB all_srcs
    src/main.cpp
    src/core/*/c*  src/core/*.h*
)

# Build the game. Hope you enjoy!
if (MSVC)
    add_executable(${PROJECT_NAME} WIN32 ${all_srcs} )
else()
    add_executable(${PROJECT_NAME} ${all_srcs} )
endif()