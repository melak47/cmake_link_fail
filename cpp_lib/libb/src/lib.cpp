#include "Other.h"
#include "liba.h"
#include <iostream>
void bridge::from_cpp() {
    A::from_a();
    std::cout<<"from b";
}

//compile the bridging code
#include "cpp_lib/bridge.rs.cc"