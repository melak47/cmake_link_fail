add_library(
    OtherLib
    src/lib.cpp
)
add_dependencies(OtherLib Bridge LibA)
include_directories(include)
target_link_libraries(OtherLib PUBLIC LibA)
install(TARGETS OtherLib LibA)
