mod internal {
    use crate::types::{CVOptionFlags, CVReturn, OSType};
    use core_foundation::base::{Boolean, CFAllocatorRef, CFTypeID, TCFType};
    use core_foundation::dictionary::CFDictionaryRef;
    use core_foundation::{declare_TCFType, impl_TCFType};
    use core_utils_rs::ref_con::RefCon;
    use io_surface::IOSurfaceRef;
    use std::ffi::c_void;

    #[repr(C)]
    pub struct __CVPixelBufferRef(c_void);

    pub type CVPixelBufferRef = *mut __CVPixelBufferRef;

    declare_TCFType! {CVPixelBuffer, CVPixelBufferRef}
    impl_TCFType!(CVPixelBuffer, CVPixelBufferRef, CVPixelBufferGetTypeID);

    type CVPixelBufferReleaseBytesCallback =
        extern "C" fn(releaseRefCon: RefCon, baseAddress: *const c_void);

    #[link(name = "CoreVideo", kind = "framework")]
    extern "C" {

        fn CVPixelBufferGetTypeID() -> CFTypeID;
        // Create Pixel buffers
        fn CVPixelBufferCreate(
            allocator: CFAllocatorRef,
            width: usize,
            height: usize,
            pixelFormatType: OSType,
            pixelBufferAttributes: CFDictionaryRef,
            pixelBufferOut: *mut CVPixelBufferRef,
        ) -> CVReturn;

        fn CVPixelBufferCreateWithBytes(
            allocator: CFAllocatorRef,
            width: usize,
            height: usize,
            pixelFormatType: OSType,
            baseAddress: *mut c_void,
            bytesPerRow: usize,
            releaseCallback: CVPixelBufferReleaseBytesCallback,
            releaseRefCon: *mut c_void,
            pixelBufferAttributes: CFDictionaryRef,
            pixelBufferOut: *mut CVPixelBufferRef,
        ) -> CVReturn;

        fn CVPixelBufferCreateWithPlanarBytes(
            allocator: CFAllocatorRef,
            width: usize,
            height: usize,
            pixelFormatType: OSType,
            dataPtr: *mut *mut c_void,
            dataSize: *mut usize,
            numberOfPlanes: usize,
            planeBaseAddress: *mut *mut c_void,
            planeWidth: *mut usize,
            planeHeight: *mut usize,
            planeBytesPerRow: *mut usize,
            releaseCallback: CVPixelBufferReleaseBytesCallback,
            releaseRefCon: *mut c_void,
            pixelBufferAttributes: CFDictionaryRef,
            pixelBufferOut: *mut CVPixelBufferRef,
        ) -> CVReturn;

        fn CVPixelBufferCreateWithIOSurface(
            allocator: CFAllocatorRef,
            surface: IOSurfaceRef,
            pixelBufferAttributes: CFDictionaryRef,
            pixelBufferOut: *mut CVPixelBufferRef,
        ) -> CVReturn;

        // Inspecting Pixel Buffers
        fn CVPixelBufferGetBaseAddress(pixelBuffer: CVPixelBufferRef) -> *mut c_void;
        fn CVPixelBufferGetBaseAddressOfPlane(
            pixelBuffer: CVPixelBufferRef,
            planeIndex: usize,
        ) -> *mut c_void;
        fn CVPixelBufferGetBytesPerRow(pixelBuffer: CVPixelBufferRef) -> usize;
        fn CVPixelBufferGetBytesPerRowOfPlane(
            pixelBuffer: CVPixelBufferRef,
            planeIndex: usize,
        ) -> usize;
        fn CVPixelBufferGetHeight(pixelBuffer: CVPixelBufferRef) -> usize;
        fn CVPixelBufferGetHeightOfPlane(pixelBuffer: CVPixelBufferRef, planeIndex: usize)
            -> usize;
        fn CVPixelBufferGetWidth(pixelBuffer: CVPixelBufferRef) -> usize;
        fn CVPixelBufferGetWidthOfPlane(pixelBuffer: CVPixelBufferRef, planeIndex: usize) -> usize;
        fn CVPixelBufferIsPlanar(pixelBuffer: CVPixelBufferRef) -> Boolean;
        fn CVPixelBufferGetPlaneCount(pixelBuffer: CVPixelBufferRef) -> usize;
        fn CVPixelBufferGetDataSize(pixelBuffer: CVPixelBufferRef) -> usize;
        fn CVPixelBufferGetPixelFormatType(pixelBuffer: CVPixelBufferRef) -> OSType;
        fn CVPixelBufferGetExtendedPixels(
            pixelBuffer: CVPixelBufferRef,
            extraColumnsOnLeft: *mut usize,
            extraColumnsOnRight: *mut usize,
            extraRowsOnTop: *mut usize,
            extraRowsOnBottom: *mut usize,
        );
        fn CVPixelBufferGetIOSurface(pixelBuffer: CVPixelBufferRef) -> IOSurfaceRef;
        fn CVPixelBufferCreateResolvedAttributesDictionary(
            allocator: CFAllocatorRef,
            attributes: CFDictionaryRef,
            resolvedDictionaryOut: *mut CFDictionaryRef,
        ) -> CVReturn;

        // Modifying pixel buffers
        fn CVPixelBufferLockBaseAddress(
            pixelBuffer: CVPixelBufferRef,
            lockFlags: CVOptionFlags,
        ) -> CVReturn;
        fn CVPixelBufferUnlockBaseAddress(
            pixelBuffer: CVPixelBufferRef,
            unlockFlags: CVOptionFlags,
        ) -> CVReturn;
        fn CVPixelBufferFillExtendedPixels(pixelBuffer: CVPixelBufferRef) -> CVReturn;

        // Release/Retain
        fn CVPixelBufferRetain(pixelBuffer: CVPixelBufferRef) -> CVPixelBufferRef;
        fn CVPixelBufferRelease(pixelBuffer: CVPixelBufferRef);

    }
}
pub use internal::{CVPixelBuffer, CVPixelBufferRef};
