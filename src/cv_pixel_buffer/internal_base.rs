use core::fmt;

use std::{ffi::c_void, fmt::Formatter};

use core_foundation::base::{TCFType, CFTypeID};
use core_utils_rs::{declare_TCFType, impl_TCFType};


#[repr(C)]
pub struct __CVPixelBufferRef(c_void);

pub type CVPixelBufferRef = *mut __CVPixelBufferRef;

declare_TCFType! {CVPixelBuffer<'a>, CVPixelBufferRef}
impl_TCFType!(CVPixelBuffer<'a>, CVPixelBufferRef, CVPixelBufferGetTypeID);

impl fmt::Debug for CVPixelBuffer<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "CVPixelBuffer")
    }
}

extern "C" {
    fn CVPixelBufferGetTypeID() -> CFTypeID;
}

