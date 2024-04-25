mod internal {
    #![allow(dead_code)]
    #![allow(clippy::too_many_arguments)]
    use crate::cv_pixel_buffer_error::{CVPixelBufferError, CV_RETURN_SUCCESS};
    use crate::types::{CVReturn, OSType};
    use core_foundation::base::{kCFAllocatorDefault, CFAllocatorRef, CFTypeID, TCFType};
    use core_foundation::dictionary::CFDictionaryRef;
    use core_utils_rs::ref_con::{ClosureCaller, ClosurePointer, VoidTrampoline};
    use four_char_code::FourCharCode;
    use std::ffi::c_void;
    use std::ptr::{self};

    #[repr(C)]
    pub struct __CVPixelBufferRef(c_void);

    pub type CVPixelBufferRef = *mut __CVPixelBufferRef;

    pub struct CVPixelBuffer(CVPixelBufferRef, Vec<VoidTrampoline>);

    impl Drop for CVPixelBuffer {
        fn drop(&mut self) {
            unsafe { core_foundation::base::CFRelease(self.as_CFTypeRef()) }
        }
    }

    unsafe impl core_foundation::ConcreteCFType for CVPixelBuffer {}
    impl core_foundation::base::TCFType for CVPixelBuffer {
        type Ref = CVPixelBufferRef;

        #[inline]
        fn as_concrete_TypeRef(&self) -> CVPixelBufferRef {
            self.0
        }

        #[inline]
        fn as_CFTypeRef(&self) -> core_foundation::base::CFTypeRef {
            self.as_concrete_TypeRef() as core_foundation::base::CFTypeRef
        }

        #[inline]
        unsafe fn wrap_under_get_rule(reference: CVPixelBufferRef) -> Self {
            assert!(!reference.is_null(), "Attempted to create a NULL object.");
            let reference =
                core_foundation::base::CFRetain(reference as *const ::std::os::raw::c_void)
                    as CVPixelBufferRef;
            core_foundation::base::TCFType::wrap_under_create_rule(reference)
        }

        #[inline]
        unsafe fn wrap_under_create_rule(reference: CVPixelBufferRef) -> Self {
            assert!(!reference.is_null(), "Attempted to create a NULL object.");
            CVPixelBuffer(reference, Vec::new())
        }

        fn type_id() -> CFTypeID {
            unsafe { CVPixelBufferGetTypeID() }
        }
    }

    impl Clone for CVPixelBuffer {
        #[inline]
        fn clone(&self) -> CVPixelBuffer {
            unsafe { CVPixelBuffer::wrap_under_get_rule(self.0) }
        }
    }

    impl PartialEq for CVPixelBuffer {
        #[inline]
        fn eq(&self, other: &CVPixelBuffer) -> bool {
            self.as_CFType().eq(&other.as_CFType())
        }
    }

    impl Eq for CVPixelBuffer {}

    unsafe impl<'a> core_foundation::base::ToVoid<CVPixelBuffer> for &'a CVPixelBuffer {
        fn to_void(&self) -> *const ::std::os::raw::c_void {
            use core_foundation::base::TCFTypeRef;
            self.as_concrete_TypeRef().as_void_ptr()
        }
    }

    unsafe impl core_foundation::base::ToVoid<CVPixelBuffer> for CVPixelBuffer {
        fn to_void(&self) -> *const ::std::os::raw::c_void {
            use core_foundation::base::TCFTypeRef;
            self.as_concrete_TypeRef().as_void_ptr()
        }
    }

    unsafe impl core_foundation::base::ToVoid<CVPixelBuffer> for CVPixelBufferRef {
        fn to_void(&self) -> *const ::std::os::raw::c_void {
            use core_foundation::base::TCFTypeRef;
            self.as_void_ptr()
        }
    }
    impl CVPixelBuffer {
        fn store_trampoline(&mut self, trampoline: VoidTrampoline) {
            self.1.push(trampoline);
        }
    }

    #[link(name = "CoreVideo", kind = "framework")]
    extern "C" {
        static kCVPixelFormatType_32BGRA: [u8; 4];
        fn CVPixelBufferGetTypeID() -> CFTypeID;
        fn CVPixelBufferGetHeight(pixelBuffer: CVPixelBufferRef) -> usize;
        fn CVPixelBufferGetWidth(pixelBuffer: CVPixelBufferRef) -> usize;
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
            pixel_format_type: OSType,
            base_address: *const u8,
            bytes_per_row: usize,
            raw_release_callback: Option<ClosureCaller>,
            raw_release_ref_con: ClosurePointer,
            pixel_buffer_attributes: CFDictionaryRef,
            pixel_buffer_out: *mut CVPixelBufferRef,
        ) -> CVReturn;

    }
    pub fn create_with_bytes(
        width: usize,
        height: usize,
        pixel_format_type: FourCharCode,
        base_address: Vec<u8>,
        bytes_per_row: usize,
        pixel_buffer_attributes: CFDictionaryRef,
    ) -> Result<CVPixelBuffer, CVPixelBufferError> {
        if base_address.len() < bytes_per_row * height {
            return Err(CVPixelBufferError::InvalidSize);
        }

        let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();
        let base_address_ptr = base_address.as_ptr();

        unsafe {
            let result = CVPixelBufferCreateWithBytes(
                kCFAllocatorDefault,
                width,
                height,
                pixel_format_type.as_u32(),
                base_address_ptr,
                bytes_per_row,
                None,
                ptr::null(),
                pixel_buffer_attributes,
                &mut pixel_buffer_out,
            );
            if result == CV_RETURN_SUCCESS {
                Ok(CVPixelBuffer::wrap_under_create_rule(pixel_buffer_out))
            } else {
                Err(CVPixelBufferError::from(result))
            }
        }
    }
    pub fn create_with_bytes_with_release<TRefCon, TReleaseCallback>(
        width: usize,
        height: usize,
        pixel_format_type: FourCharCode,
        base_address: Vec<u8>,
        bytes_per_row: usize,
        mut release_callback: TReleaseCallback,
        release_ref_con: TRefCon,
        pixel_buffer_attributes: CFDictionaryRef,
    ) -> Result<CVPixelBuffer, CVPixelBufferError>
    where
        TRefCon: 'static,
        TReleaseCallback: FnMut(TRefCon, Vec<u8>) + 'static,
    {
        if base_address.len() < bytes_per_row * height {
            return Err(CVPixelBufferError::InvalidSize);
        }

        let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();
        let base_address_ptr = base_address.as_ptr();

        let trampoline =
            VoidTrampoline::new(move || release_callback(release_ref_con, base_address));

        unsafe {
            let result = CVPixelBufferCreateWithBytes(
                kCFAllocatorDefault,
                width,
                height,
                pixel_format_type.as_u32(),
                base_address_ptr,
                bytes_per_row,
                Some(trampoline.caller),
                trampoline.as_code_ptr(),
                pixel_buffer_attributes,
                &mut pixel_buffer_out,
            );
            if result == CV_RETURN_SUCCESS {
                let mut pixel_buffer = CVPixelBuffer::wrap_under_create_rule(pixel_buffer_out);
                pixel_buffer.store_trampoline(trampoline);
                Ok(pixel_buffer)
            } else {
                Err(CVPixelBufferError::from(result))
            }
        }
    }

    #[cfg(test)]
    mod test {
        use std::error::Error;

        const WIDTH: usize = 100;
        const BYTE_PER_ROW: usize = 100 * 4;
        const HEIGHT: usize = 100;
        const SIZE: usize = WIDTH * HEIGHT * 4;
        const PIXEL_VALUE: u8 = 0xF2;

        #[test]
        fn test_create_with_bytes_and_release() -> Result<(), Box<dyn Error>> {
            use super::*;
            let move_into_closure = vec![1, 2, 3];
            let pixel_buffer = create_with_bytes_with_release(
                WIDTH,
                HEIGHT,
                FourCharCode::from_str("BGRA").unwrap(),
                vec![PIXEL_VALUE; SIZE],
                BYTE_PER_ROW,
                move |refcon, address| {
                    println!(
                        "Release callback called:{:?} {:?} {:?}",
                        move_into_closure, refcon, address
                    );
                },
                32,
                ptr::null(),
            )?;
            assert_eq!(
                unsafe { CVPixelBufferGetWidth(pixel_buffer.as_concrete_TypeRef()) },
                WIDTH
            );
            assert_eq!(
                unsafe { CVPixelBufferGetHeight(pixel_buffer.as_concrete_TypeRef()) },
                HEIGHT
            );
            Ok(())
        }

        #[test]
        fn test_create_with_bytes() -> Result<(), Box<dyn Error>> {
            use super::*;
            let pixel_buffer = create_with_bytes(
                WIDTH,
                HEIGHT,
                FourCharCode::from_str("BGRA").unwrap(),
                vec![PIXEL_VALUE; SIZE],
                BYTE_PER_ROW,
                ptr::null(),
            )?;
            assert_eq!(
                unsafe { CVPixelBufferGetWidth(pixel_buffer.as_concrete_TypeRef()) },
                WIDTH
            );
            assert_eq!(
                unsafe { CVPixelBufferGetHeight(pixel_buffer.as_concrete_TypeRef()) },
                HEIGHT
            );
            Ok(())
        }
    }
}
pub use internal::{CVPixelBuffer, CVPixelBufferRef};
