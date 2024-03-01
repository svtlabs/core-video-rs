mod internal {
    use std::ffi::c_void;

    use core_foundation::base::{CFTypeID, TCFType};
    use core_foundation::{declare_TCFType, impl_TCFType};

    #[repr(C)]
    pub struct __CVImageBufferRef(c_void);

    pub type CVImageBufferRef = *mut __CVImageBufferRef;

    declare_TCFType! {CVImageBuffer, CVImageBufferRef}
    impl_TCFType!(CVImageBuffer, CVImageBufferRef, CVImageBufferGetTypeID);

    extern "C" {
        pub fn CVImageBufferGetTypeID() -> CFTypeID;
    }
}
pub use internal::{CVImageBuffer, CVImageBufferRef};
