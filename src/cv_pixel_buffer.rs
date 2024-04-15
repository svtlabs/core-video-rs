mod internal {
    use crate::cv_pixel_buffer_error::{CVPixelBufferError, NO_ERROR};
    use crate::types::{CVOptionFlags, CVReturn, OSType};
    use core_foundation::base::{
        kCFAllocatorDefault, Boolean, CFAllocatorRef, CFNullRef, CFTypeID, TCFType,
    };
    use core_foundation::dictionary::CFDictionaryRef;
    use core_foundation::{declare_TCFType, impl_TCFType};
    use core_utils_rs::ref_con::{cf_trampoline, trampoline_reversed, RefCon, TrampolineRefCon};
    use four_char_code::FourCharCode;
    use io_surface::IOSurfaceRef;
    use std::ffi::c_void;
    use std::ptr;

    #[repr(C)]
    pub struct __CVPixelBufferRef(c_void);

    pub type CVPixelBufferRef = *mut __CVPixelBufferRef;

    declare_TCFType! {CVPixelBuffer, CVPixelBufferRef}
    impl_TCFType!(CVPixelBuffer, CVPixelBufferRef, CVPixelBufferGetTypeID);

    type CVPixelBufferReleaseBytesCallback =
        unsafe extern "C" fn(releaseRefCon: *mut TrampolineRefCon, baseAddress: *mut c_void);

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
            releaseRefCon: *mut TrampolineRefCon,
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
    pub fn create(
        width: usize,
        height: usize,
        pixel_format_type: FourCharCode,
        pixel_buffer_attributes: CFDictionaryRef,
    ) -> Result<CVPixelBuffer, CVPixelBufferError> {
        let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();
        unsafe {
            let result = CVPixelBufferCreate(
                kCFAllocatorDefault,
                width,
                height,
                pixel_format_type.as_u32(),
                pixel_buffer_attributes,
                &mut pixel_buffer_out,
            );
            if result == NO_ERROR {
                Ok(CVPixelBuffer::wrap_under_create_rule(pixel_buffer_out))
            } else {
                Err(CVPixelBufferError::from(result))
            }
        }
    }
    pub fn create_with_bytes<TRefCon, TMakeDataReadyCallback>(
        width: usize,
        height: usize,
        pixel_format_type: FourCharCode,
        base_address: *mut c_void,
        bytes_per_row: usize,
        release_callback: TMakeDataReadyCallback,
        release_ref_con: TRefCon,
        pixel_buffer_attributes: CFDictionaryRef,
    ) -> Result<CVPixelBuffer, CVPixelBufferError>
    where
        TRefCon: Sized,
        TMakeDataReadyCallback: FnMut(TRefCon, *mut c_void) -> Result<(), CVPixelBufferError>,
        TMakeDataReadyCallback: Send + 'static,
    {
        let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();
        unsafe {
            let result = CVPixelBufferCreateWithBytes(
                kCFAllocatorDefault,
                width,
                height,
                pixel_format_type.as_u32(),
                base_address,
                bytes_per_row,
                trampoline_reversed::<
                    *mut c_void,
                    CVPixelBufferError,
                    TRefCon,
                    TMakeDataReadyCallback,
                >,
                TrampolineRefCon::new(Some(release_ref_con), release_callback)
                    .into_leaked_mut_ptr(),
                pixel_buffer_attributes,
                &mut pixel_buffer_out,
            );
            if result == NO_ERROR {
                Ok(CVPixelBuffer::wrap_under_create_rule(pixel_buffer_out))
            } else {
                Err(CVPixelBufferError::from(result))
            }
        }
    }
    #[cfg(test)]
    mod test {
        #[test]
        fn test_create() -> Result<(), crate::cv_pixel_buffer_error::CVPixelBufferError> {
            use super::*;
            let pixel_buffer = create(
                1920,
                1080,
                FourCharCode::from_str("2vuy").unwrap(),
                ptr::null(),
            )?;
            assert_eq!(
                unsafe { CVPixelBufferGetWidth(pixel_buffer.as_concrete_TypeRef()) },
                1920
            );
            assert_eq!(
                unsafe { CVPixelBufferGetHeight(pixel_buffer.as_concrete_TypeRef()) },
                1080
            );
            assert_eq!(
                unsafe { CVPixelBufferGetPixelFormatType(pixel_buffer.as_concrete_TypeRef()) },
                FourCharCode::from_str("2vuy").unwrap().as_u32()
            );
            Ok(())
        }

        #[test]
        fn test_create_with_bytes() -> Result<(), crate::cv_pixel_buffer_error::CVPixelBufferError>
        {
            use super::*;
            let pixel_buffer = create_with_bytes(
                1920,
                1080,
                FourCharCode::from_str("2vuy").unwrap(),
                ptr::null_mut(),
                1920 * 2,
                |_: *mut c_void, _| Ok(()),
                ptr::null_mut(),
                ptr::null(),
            )?;
            assert_eq!(
                unsafe { CVPixelBufferGetWidth(pixel_buffer.as_concrete_TypeRef()) },
                1920
            );
            assert_eq!(
                unsafe { CVPixelBufferGetHeight(pixel_buffer.as_concrete_TypeRef()) },
                1080
            );
            assert_eq!(
                unsafe { CVPixelBufferGetPixelFormatType(pixel_buffer.as_concrete_TypeRef()) },
                FourCharCode::from_str("2vuy").unwrap().as_u32()
            );
            Ok(())
        }
    }
}
pub use internal::{CVPixelBuffer, CVPixelBufferRef};
