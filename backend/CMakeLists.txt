cmake_minimum_required(VERSION 3.15)
project(GraphicsBackend)

set(SDL_STATIC ON CACHE BOOL "Use Static SDL" FORCE)
add_subdirectory(deps/software-rendering)

add_library(backend SHARED window.cpp draw.cpp)

target_link_libraries(backend PUBLIC srRenderer SDL2-static)
target_include_directories(backend SYSTEM PUBLIC deps/sdl2/include)
target_include_directories(backend PUBLIC deps/software-rendering/src)
target_compile_definitions(backend PRIVATE INTERFACE_EXPORT)

# set c++17
set_target_properties(backend PROPERTIES CXX_STANDARD 17)

install(TARGETS backend DESTINATION .)