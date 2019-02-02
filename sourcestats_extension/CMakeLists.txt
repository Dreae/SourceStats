cmake_minimum_required(VERSION 3.5)
project(sourcestats)

set(SOURCE_FILES
    cpp/Extension.cpp
    cpp/include/smsdk_ext.cpp
)


set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 14)
add_definitions(-DSOURCEMOD_BUILD)

if(MSVC)
    add_definitions(-DWIN32 -D_WINDOWS -DCOMPILER_MSVC -DCOMPILER_MSVC32)
else()
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -pipe -fno-strict-aliasing -m32")
    add_definitions(-D_LINUX -DPOSIX)
endif()

if(CMAKE_COMPILER_IS_GNUCXX)
    add_definitions(-DCOMPILER_GCC -D_vsnprintf=vsnprintf -D_snprintf=snprintf -D_stricmp=strcasecmp -Dstricmp=strcasecmp)
endif()

include(CMakeLists.rust.txt)

include_directories(
    sourcemod/public
    sourcemod/public/amtl
    sourcemod/public/amtl/amtl
    sourcemod/sourcepawn/include
)

add_library(sourcestats.ext SHARED ${SOURCE_FILES})
add_dependencies(sourcestats.ext sourcestats_extension)

if(MSVC)
    target_link_libraries(sourcestats.ext
        debug sourcestats_rust
        optimized sourcestats_rust
    )
else()
    target_link_libraries(sourcestats.ext
        general "-Wl,--no-undefined -lpthread -ldl -m32 -static-libgcc -static-libstdc++"
        debug sourcestats_rust
        optimized sourcestats_rust
    )
endif()

set_target_properties(sourcestats.ext PROPERTIES PREFIX "")
