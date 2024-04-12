mod internal {
    use core_foundation::base::{Boolean, CFTypeID, TCFType};
    use core_foundation::dictionary::CFDictionaryRef;
    use core_foundation::string::CFStringRef;
    use core_foundation::{declare_TCFType, impl_TCFType};
    use core_graphics::sys::CGColorSpace;
    use std::ffi::c_void;
    type CGColorSpaceRef = CGColorSpace;
    #[repr(C)]
    pub struct __CVImageBufferRef(c_void);

    pub type CVImageBufferRef = *mut __CVImageBufferRef;

    declare_TCFType! {CVImageBuffer, CVImageBufferRef}
    impl_TCFType!(CVImageBuffer, CVImageBufferRef, CVImageBufferGetTypeID);

    #[link(name = "CoreVideo", kind = "framework")]
    extern "C" {

        fn CVImageBufferGetTypeID() -> CFTypeID;
        // Inspecting Image buffers
        fn CVImageBufferGetColorSpace(image_buffer: CVImageBufferRef) -> CGColorSpace;
        fn CVImageBufferGetEncodedSize(image_buffer: CVImageBufferRef) -> Boolean;
        fn CVImageBufferGetDisplaySize(image_buffer: CVImageBufferRef) -> Boolean;
        fn CVImageBufferGetCleanRect(image_buffer: CVImageBufferRef) -> Boolean;
        fn CVImageBufferGetIsDataReady(image_buffer: CVImageBufferRef) -> Boolean;
        fn CVImageBufferCreateColorSpaceFromAttachments(
            attachments: CFDictionaryRef,
        ) -> CGColorSpaceRef;
        fn CVColorPrimariesGetIntegerCodePointForString(primaries: CFStringRef) -> i32;
        fn CVColorPrimariesGetStringForIntegerCodePoint(code_point: i32) -> CFStringRef;
        fn CVTransferFunctionGetIntegerCodePointForString(transfer_punction: CFStringRef) -> i32;
        fn CVTransferFunctionGetStringForIntegerCodePoint(codePoint: i32) -> CFStringRef;
        fn CVYCbCrMatrixGetIntegerCodePointForString(matrix: CFStringRef) -> i32;
        fn CVYCbCrMatrixGetStringForIntegerCodePoint(code_point: i32) -> CFStringRef;

    }
}
pub use internal::{CVImageBuffer, CVImageBufferRef};
