

#[cxx::bridge(namespace="bridge")]
mod ffi {
    unsafe extern "C++" {

        // include!("Other.h");
        fn from_cpp();
    }
}
pub use ffi::*;