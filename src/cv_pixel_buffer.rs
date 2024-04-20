mod internal {
    #![allow(dead_code)]
    #![allow(clippy::too_many_arguments)]
    use crate::cv_pixel_buffer_error::{CVPixelBufferError, CV_RETURN_SUCCESS};
    use crate::types::{CVOptionFlags, CVReturn, OSType};
    use core_foundation::base::{kCFAllocatorDefault, Boolean, CFAllocatorRef, CFTypeID, TCFType};
    use core_foundation::dictionary::CFDictionaryRef;
    use core_foundation::{declare_TCFType, impl_TCFType};
    use core_utils_rs::declare_trampoline;
    use core_utils_rs::ref_con::TrampolineRefCon;
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
        unsafe extern "C" fn(releaseRefCon: *mut TrampolineRefCon, planebaseaddresses: *mut [u8]);

    type CVPixelBufferReleaseBytesPlanarCallback = unsafe extern "C" fn(
        releaseRefCon: *mut TrampolineRefCon,
        dataPtr: *mut c_void,
        dataSize: usize,
        numberOfPlanes: usize,
        planebaseaddresses: *mut [u8],
    );

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
            pixel_format_type: OSType,
            base_address: *mut c_void,
            bytes_per_row: usize,
            release_callback: CVPixelBufferReleaseBytesCallback,
            release_ref_con: *mut TrampolineRefCon,
            pixel_buffer_attributes: CFDictionaryRef,
            pixel_buffer_out: *mut CVPixelBufferRef,
        ) -> CVReturn;

        fn CVPixelBufferCreateWithPlanarBytes(
            allocator: CFAllocatorRef,
            width: usize,
            height: usize,
            pixelFormatType: OSType,
            dataPtr: *mut c_void,
            dataSize: usize,
            numberOfPlanes: usize,
            planeBaseAddresses: *const *mut u8,
            planeWidth: *const usize,
            planeHeight: *const usize,
            planeBytesPerRow: *const usize,
            releaseCallback: CVPixelBufferReleaseBytesPlanarCallback,
            releaseRefCon: *mut TrampolineRefCon,
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
            if result == CV_RETURN_SUCCESS {
                Ok(CVPixelBuffer::wrap_under_create_rule(pixel_buffer_out))
            } else {
                Err(CVPixelBufferError::from(result))
            }
        }
    }

    pub fn create_with_bytes<TRefCon, TReleaseCallback>(
        width: usize,
        height: usize,
        pixel_format_type: FourCharCode,
        base_address: &mut [u8],
        bytes_per_row: usize,
        release_callback: TReleaseCallback,
        release_ref_con: TRefCon,
        pixel_buffer_attributes: CFDictionaryRef,
    ) -> Result<CVPixelBuffer, CVPixelBufferError>
    where
        TRefCon: Sized,
        TReleaseCallback: FnMut(TRefCon, &mut [u8]),
        TReleaseCallback: Send + 'static,
    {
        let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();
        if base_address.len() < bytes_per_row * height {
            return Err(CVPixelBufferError::InvalidSize);
        }
        pub unsafe extern "C" fn release_callback_trampoline<
            TRefcon,
            FCallback: FnMut(TRefcon, &mut [u8]) + Send + 'static,
        >(
            refcon: *mut TrampolineRefCon,
            planebaseaddresses: *mut [u8],
        ) {
            let refcon_data = &*(refcon);
            let mut user_data: FCallback = ptr::read(refcon_data.0.cast());
            user_data(ptr::read(refcon_data.0.cast()), &mut *planebaseaddresses);
            ptr::drop_in_place(refcon);
        };
        println!("first: {:p}", base_address);
        unsafe {
            let result = CVPixelBufferCreateWithBytes(
                kCFAllocatorDefault,
                width,
                height,
                pixel_format_type.as_u32(),
                base_address as *mut _ as *mut c_void,
                bytes_per_row,
                release_callback_trampoline::<TRefCon, TReleaseCallback>,
                TrampolineRefCon::new(Some(release_ref_con), release_callback)
                    .into_leaked_mut_ptr(),
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

    pub fn create_with_bytes_planar<TReleaseRefcon, TReleaseCallback, TDataPtr>(
        width: usize,
        height: usize,
        pixel_format_type: FourCharCode,
        data_ptr: Option<&mut TDataPtr>,
        data_size: Option<usize>,
        number_of_planes: usize,
        plane_base_addresses: &[&mut [u8]],
        plane_width: &[usize],
        plane_height: &[usize],
        plane_bytes_per_row: &[usize],
        release_callback: TReleaseCallback,
        release_ref_con: TReleaseRefcon,
        pixel_buffer_attributes: CFDictionaryRef,
    ) -> Result<CVPixelBuffer, CVPixelBufferError>
    where
        TReleaseRefcon: Sized,
        TReleaseCallback: FnMut(
            TReleaseRefcon,
            Option<TDataPtr>,
            Option<usize>,
            usize,
            &mut [&mut [u8]],
        ) -> Result<(), CVPixelBufferError>,
        TReleaseCallback: Send + 'static,
    {
        let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();

        if plane_base_addresses.len() < number_of_planes
            || plane_width.len() < number_of_planes
            || plane_height.len() < number_of_planes
            || plane_bytes_per_row.len() < number_of_planes
        {
            return Err(CVPixelBufferError::InvalidSize);
        }
        todo!();
        // unsafe {
        //     let result = CVPixelBufferCreateWithPlanarBytes(
        //         kCFAllocatorDefault,
        //         width,
        //         height,
        //         pixel_format_type.as_u32(),
        //         data_ptr.map_or(ptr::null_mut(), |v| v as *mut TDataPtr as *mut c_void),
        //         data_size.map_or(0, |v| v),
        //         number_of_planes,
        //         plane_base_addresses.as_ptr().cast(),
        //         plane_width.as_ptr(),
        //         plane_height.as_ptr(),
        //         plane_bytes_per_row.as_ptr(),
        //         release_callback_trampoline::<TReleaseCallback>,
        //         TrampolineRefCon::new(Some(release_ref_con), release_callback)
        //             .into_leaked_mut_ptr(),
        //         pixel_buffer_attributes,
        //         &mut pixel_buffer_out,
        //     );
        //     if result == CV_RETURN_SUCCESS {
        //         Ok(CVPixelBuffer::wrap_under_create_rule(pixel_buffer_out))
        //     } else {
        //         Err(CVPixelBufferError::from(result))
        //     }
        // }
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
            let mut buffer = &mut vec![];
            buffer.resize(1920 * 1080 * 2, 4);
            let b = buffer.as_mut_slice();
            println!("second: {:p}", b);
            let pixel_buffer = create_with_bytes(
                1920,
                1080,
                FourCharCode::from_str("2vuy").unwrap(),
                b,
                1920 * 2,
                |refcon, address| println!("Release {refcon} {address:?}"),
                32,
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

        // #[test]
        // fn test_create_with_bytes_planar(
        // ) -> Result<(), crate::cv_pixel_buffer_error::CVPixelBufferError> {
        //     use super::*;
        //     let buffer = &mut vec![0];
        //     buffer.resize(1920 * 1080 * 2, 0);
        //     let pixel_buffer = create_with_bytes_planar(
        //         1920,
        //         1080,
        //         FourCharCode::from_str("2vuy").unwrap(),
        //         None,
        //         None,
        //         1,
        //         &[buffer],
        //         &[1920],
        //         &[1080],
        //         &[1920 * 2],
        //         |_: (), _: (), _, _, _| Ok(()),
        //         (),
        //         ptr::null(),
        //     )?;
        //     assert_eq!(
        //         unsafe { CVPixelBufferGetWidth(pixel_buffer.as_concrete_TypeRef()) },
        //         1920
        //     );
        //     assert_eq!(
        //         unsafe { CVPixelBufferGetHeight(pixel_buffer.as_concrete_TypeRef()) },
        //         1080
        //     );
        //     assert_eq!(
        //         unsafe { CVPixelBufferGetPixelFormatType(pixel_buffer.as_concrete_TypeRef()) },
        //         FourCharCode::from_str("2vuy").unwrap().as_u32()
        //     );
        //     Ok(())
        // }
    }
}
pub use internal::{CVPixelBuffer, CVPixelBufferRef};
