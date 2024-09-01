mod internal {
    #![allow(dead_code)]
    #![allow(clippy::too_many_arguments)]
    use crate::cv_pixel_buffer_error::{CVPixelBufferError, CV_RETURN_SUCCESS};
    use crate::types::{CVReturn, OSType};
    use core_foundation::base::{kCFAllocatorDefault, CFAllocatorRef, CFTypeID, TCFType};
    use core_foundation::dictionary::CFDictionaryRef;
    use core_foundation::{declare_TCFType, impl_TCFType};
    use core_utils_rs::ref_con::{trampoline, ClosureCaller, ClosurePointer};
    use four_char_code::FourCharCode;
    use io_surface::IOSurfaceRef;
    use std::ffi::c_void;
    use std::marker::PhantomPinned;
    use std::ptr::{self, NonNull};

    #[repr(C)]
    pub struct __CVPixelBufferRef(c_void);

    pub type CVPixelBufferRef = *mut __CVPixelBufferRef;

    declare_TCFType! {CVPixelBuffer, CVPixelBufferRef}
    impl_TCFType!(CVPixelBuffer, CVPixelBufferRef, CVPixelBufferGetTypeID);

    pub struct CVPixelBufferPlanarDataPointer {
        data: Option<Vec<u8>>,
        number_of_planes: usize,
        plane_bytes_per_row: Vec<usize>,
        plane_width: Vec<usize>,
        plane_height: Vec<usize>,
        base_addresses: Vec<NonNull<[u8]>>,
        _pin: PhantomPinned,
    }

    impl CVPixelBufferPlanarDataPointer {
        pub fn new(
            data: Option<Vec<u8>>,
            plane_bytes_per_row: Vec<usize>,
            plane_width: Vec<usize>,
            plane_height: Vec<usize>,
            base_addresses: Vec<NonNull<[u8]>>,
        ) -> CVPixelBufferPlanarDataPointer {
            CVPixelBufferPlanarDataPointer {
                data,
                number_of_planes: base_addresses.len(),
                plane_bytes_per_row,
                plane_width,
                plane_height,
                base_addresses,
                _pin: PhantomPinned,
            }
        }
        fn number_of_planes(&self) -> usize {
            self.number_of_planes
        }
        fn as_ptr(&self) -> *mut u8 {
            self.data
                .as_ref()
                .map(|v| v.as_ptr())
                .unwrap_or(ptr::null_mut())
                .cast_mut()
        }

        fn data_size(&self) -> usize {
            self.data.as_ref().map(|v| v.len()).unwrap_or(0)
        }

        fn raw_base_addresses(&self) -> *const *const u8 {
            self.base_addresses.as_ptr().cast()
        }
        fn plane_bytes_per_row(&self) -> *const usize {
            self.plane_bytes_per_row.as_ptr()
        }
        fn plane_width(&self) -> *const usize {
            self.plane_width.as_ptr()
        }
        fn plane_height(&self) -> *const usize {
            self.plane_height.as_ptr()
        }
    }

    #[link(name = "CoreVideo", kind = "framework")]
    extern "C" {
        static kCVPixelFormatType_32BGRA: [u8; 4];
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
            base_address: *const u8,
            bytes_per_row: usize,
            raw_release_callback: ClosureCaller,
            raw_release_ref_con: ClosurePointer,
            pixel_buffer_attributes: CFDictionaryRef,
            pixel_buffer_out: *mut CVPixelBufferRef,
        ) -> CVReturn;

        fn CVPixelBufferCreateWithPlanarBytes(
            allocator: CFAllocatorRef,
            width: usize,
            height: usize,
            pixel_format_type: OSType,
            data_ptr: *mut u8,
            data_size: usize,
            number_of_planes: usize,
            base_addresses: *const *const u8,
            plane_width: *const usize,
            plane_height: *const usize,
            plane_bytes_per_row: *const usize,
            release_callback: ClosureCaller,
            release_ref_con: ClosurePointer,
            pixel_buffer_attributes: CFDictionaryRef,
            pixel_buffer_out: *mut CVPixelBufferRef,
        ) -> CVReturn;

        fn CVPixelBufferCreateWithIOSurface(
            allocator: CFAllocatorRef,
            surface: IOSurfaceRef,
            pixelBufferAttributes: CFDictionaryRef,
            pixelBufferOut: *mut CVPixelBufferRef,
        ) -> CVReturn;

    }
    pub fn create_with_planar_bytes(
        width: usize,
        height: usize,
        pixel_format_type: FourCharCode,
        data_pointer: CVPixelBufferPlanarDataPointer,
        pixel_buffer_attributes: CFDictionaryRef,
    ) -> Result<CVPixelBuffer, CVPixelBufferError> {
        let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();
        let data_size = data_pointer.data_size();
        let data_ptr = data_pointer.as_ptr();
        let number_of_planes = data_pointer.number_of_planes();
        let base_addresses = data_pointer.raw_base_addresses();
        let plane_width = data_pointer.plane_width();
        let plane_height = data_pointer.plane_height();
        let plane_bytes_per_row = data_pointer.plane_bytes_per_row();
        todo!();
        // unsafe {
        //     let result = CVPixelBufferCreateWithPlanarBytes(
        //         kCFAllocatorDefault,
        //         width,
        //         height,
        //         pixel_format_type.as_u32(),
        //         data_ptr,
        //         data_size,
        //         number_of_planes,
        //         base_addresses,
        //         plane_width,
        //         plane_height,
        //         plane_bytes_per_row,
        //         None,
        //         None,
        //         pixel_buffer_attributes,
        //         &mut pixel_buffer_out,
        //     );
        //
        //     if result == CV_RETURN_SUCCESS {
        //         Ok(CVPixelBuffer::wrap_under_create_rule(pixel_buffer_out))
        //     } else {
        //         Err(CVPixelBufferError::from(result))
        //     }
        // }
    }
    pub fn create_with_planar_bytes_and_release_callback<TRefCon, TReleaseCallback>(
        width: usize,
        height: usize,
        pixel_format_type: FourCharCode,
        data_pointer: CVPixelBufferPlanarDataPointer,
        mut release_callback: TReleaseCallback,
        release_ref_con: TRefCon,
        pixel_buffer_attributes: CFDictionaryRef,
    ) -> Result<CVPixelBuffer, CVPixelBufferError>
    where
        TRefCon: 'static,
        TReleaseCallback:
            FnMut(TRefCon, Option<Vec<u8>>, usize, usize, Vec<NonNull<[u8]>>) + 'static,
    {
        let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();
        let data_size = data_pointer.data_size();
        let data_ptr = data_pointer.as_ptr();
        let number_of_planes = data_pointer.number_of_planes();
        let base_addresses = data_pointer.raw_base_addresses();
        let plane_width = data_pointer.plane_width();
        let plane_height = data_pointer.plane_height();
        let plane_bytes_per_row = data_pointer.plane_bytes_per_row();
        let (caller, closure) = trampoline(move || {
            release_callback(
                release_ref_con,
                data_pointer.data,
                data_size,
                number_of_planes,
                data_pointer.base_addresses,
            )
        });
        unsafe {
            let result = CVPixelBufferCreateWithPlanarBytes(
                kCFAllocatorDefault,
                width,
                height,
                pixel_format_type.as_u32(),
                data_ptr,
                data_size,
                number_of_planes,
                base_addresses,
                plane_width,
                plane_height,
                plane_bytes_per_row,
                caller,
                closure,
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
        let (caller, closure) = trampoline(move || {
            println!("Release callback called");
        });
        unsafe {
            let result = CVPixelBufferCreateWithBytes(
                kCFAllocatorDefault,
                width,
                height,
                pixel_format_type.as_u32(),
                base_address_ptr,
                bytes_per_row,
                caller,
                closure,
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

        todo!();
        // unsafe {
        //     let result = CVPixelBufferCreateWithBytes(
        //         kCFAllocatorDefault,
        //         width,
        //         height,
        //         pixel_format_type.as_u32(),
        //         base_address_ptr,
        //         bytes_per_row,
        //         trampoline.caller,
        //         trampoline.closure,
        //         pixel_buffer_attributes,
        //         &mut pixel_buffer_out,
        //     );
        //     if result == CV_RETURN_SUCCESS {
        //         let mut px = CVPixelBuffer::wrap_under_create_rule(pixel_buffer_out);
        //         px.store_trampoline(trampoline);
        //         Ok(px)
        //     } else {
        //         Err(CVPixelBufferError::from(result))
        //     }
        // }
    }
    fn create_with_io_surface(
        surface: IOSurfaceRef,
        pixel_buffer_attributes: CFDictionaryRef,
    ) -> Result<CVPixelBuffer, CVPixelBufferError> {
        let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();
        unsafe {
            let result = CVPixelBufferCreateWithIOSurface(
                kCFAllocatorDefault,
                surface,
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

    fn is_planar(pixel_buffer: &CVPixelBuffer) -> bool {
        extern "C" {
            fn CVPixelBufferIsPlanar(pixel_buffer_ref: CVPixelBufferRef) -> i32;
        }

        unsafe { CVPixelBufferIsPlanar(pixel_buffer.as_concrete_TypeRef()) == 1 }
    }
    fn get_width(pixel_buffer: &CVPixelBuffer) -> usize {
        extern "C" {
            fn CVPixelBufferGetWidth(pixel_buffer_ref: CVPixelBufferRef) -> usize;
        }

        unsafe { CVPixelBufferGetWidth(pixel_buffer.as_concrete_TypeRef()) }
    }
    fn get_height(pixel_buffer: &CVPixelBuffer) -> usize {
        extern "C" {
            fn CVPixelBufferGetHeight(pixel_buffer_ref: CVPixelBufferRef) -> usize;
        }

        unsafe { CVPixelBufferGetHeight(pixel_buffer.as_concrete_TypeRef()) }
    }

    fn create(
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

    #[cfg(test)]
    mod test {
        use core_foundation::dictionary::CFDictionary;
        use core_foundation::number::CFNumber;
        use core_foundation::string::CFString;
        use io_surface::{
            kIOSurfaceBytesPerElement, kIOSurfaceHeight, kIOSurfacePixelFormat, kIOSurfaceWidth,
        };
        use std::error::Error;

        const WIDTH: usize = 100;
        const BYTE_PER_ROW: usize = 100 * 4;
        const HEIGHT: usize = 100;
        const SIZE: usize = WIDTH * HEIGHT * 4;
        const PIXEL_VALUE: u8 = 0xF2;

        #[test]
        fn test_is_planar() -> Result<(), Box<dyn Error>> {
            use super::*;
            let pixel_buffer = create(
                WIDTH,
                HEIGHT,
                FourCharCode::from_str("BGRA").unwrap(),
                ptr::null(),
            )?;

            assert!(!is_planar(&pixel_buffer));

            let data = vec![PIXEL_VALUE; SIZE];
            let base_address = NonNull::from(data.as_slice());
            let pixel_buffer = create_with_planar_bytes(
                WIDTH,
                HEIGHT,
                FourCharCode::from_str("BGRA").unwrap(),
                CVPixelBufferPlanarDataPointer::new(
                    Some(data),
                    vec![BYTE_PER_ROW, BYTE_PER_ROW],
                    vec![WIDTH, WIDTH],
                    vec![HEIGHT, HEIGHT],
                    vec![base_address, base_address],
                ),
                ptr::null(),
            )?;
            assert!(is_planar(&pixel_buffer));
            Ok(())
        }

        #[test]
        fn test_create_with_io_surface() -> Result<(), Box<dyn Error>> {
            use super::*;
            let properties = unsafe {
                CFDictionary::from_CFType_pairs(&[
                    (
                        CFString::wrap_under_get_rule(kIOSurfaceWidth),
                        CFNumber::from(WIDTH as i32).as_CFType(),
                    ),
                    (
                        CFString::wrap_under_get_rule(kIOSurfaceHeight),
                        CFNumber::from(HEIGHT as i32).as_CFType(),
                    ),
                    (
                        CFString::wrap_under_get_rule(kIOSurfaceBytesPerElement),
                        CFNumber::from(4).as_CFType(),
                    ),
                    (
                        CFString::wrap_under_get_rule(kIOSurfacePixelFormat),
                        CFNumber::from(FourCharCode::from_str("BGRA").unwrap().as_u32() as i64)
                            .as_CFType(),
                    ),
                ])
            };
            let io_surface = io_surface::new(&properties);
            let pixel_buffer =
                create_with_io_surface(io_surface.as_concrete_TypeRef(), ptr::null())?;
            assert_eq!(get_width(&pixel_buffer), WIDTH);
            assert_eq!(get_height(&pixel_buffer), HEIGHT);
            Ok(())
        }
        #[test]
        fn test_create() -> Result<(), Box<dyn Error>> {
            use super::*;
            let pixel_buffer = create(
                WIDTH,
                HEIGHT,
                FourCharCode::from_str("BGRA").unwrap(),
                ptr::null(),
            )?;
            assert_eq!(get_width(&pixel_buffer), WIDTH);
            assert_eq!(get_height(&pixel_buffer), HEIGHT);
            Ok(())
        }
        #[derive(Debug)]
        struct TestMoveStruct {
            var1: u32,
        }

        #[test]
        fn test_create_with_planar_bytes_and_released() -> Result<(), Box<dyn Error>> {
            use super::*;
            let data = vec![PIXEL_VALUE; SIZE];
            let base_address = NonNull::from(data.as_slice());
            let expected_data = data.clone();
            let b = TestMoveStruct { var1: 33 };
            let pixel_buffer = create_with_planar_bytes_and_release_callback(
                WIDTH,
                HEIGHT,
                FourCharCode::from_str("BGRA").unwrap(),
                CVPixelBufferPlanarDataPointer::new(
                    Some(data),
                    vec![BYTE_PER_ROW, BYTE_PER_ROW],
                    vec![WIDTH, WIDTH],
                    vec![HEIGHT, HEIGHT],
                    vec![base_address, base_address],
                ),
                move |refcon, data, size, num, base| {
                    println!(
                        "Release callback called:{:?} {:?} {:?} {:?} ",
                        b, refcon, size, num
                    );
                    assert_eq!(size, SIZE);
                    assert_eq!(num, 2);
                    assert_eq!(data.unwrap(), expected_data);
                    assert_eq!(base.len(), 2);
                },
                (),
                ptr::null(),
            )?;
            assert_eq!(get_width(&pixel_buffer), WIDTH);
            assert_eq!(get_height(&pixel_buffer), HEIGHT);
            Ok(())
        }

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
            assert_eq!(get_width(&pixel_buffer), WIDTH);
            assert_eq!(get_height(&pixel_buffer), HEIGHT);
            Ok(())
        }

        #[test]
        fn test_create_with_bytes() -> Result<(), Box<dyn Error>> {
            use super::*;
            // Create pixel buffer attributes
            let pixel_buffer_attributes = {
                CFDictionary::from_CFType_pairs(&[
                    (
                        CFString::from_static_string("IOSurfaceOpenGLESTextureCompatibility"),
                        CFNumber::from(1).as_CFType(),
                    ),
                    (
                        CFString::from_static_string("IOSurfaceOpenGLESFBOCompatibility"),
                        CFNumber::from(1).as_CFType(),
                    ),
                    (
                        CFString::from_static_string("IOSurfaceCoreAnimationCompatibility"),
                        CFNumber::from(1).as_CFType(),
                    ),
                ])
            };
            let pixel_buffer = create_with_bytes(
                WIDTH,
                HEIGHT,
                FourCharCode::from_str("BGRA").unwrap(),
                vec![PIXEL_VALUE; SIZE],
                BYTE_PER_ROW,
                pixel_buffer_attributes.as_concrete_TypeRef(),
            )?;

            assert_eq!(get_width(&pixel_buffer), WIDTH);
            assert_eq!(get_height(&pixel_buffer), HEIGHT);
            Ok(())
        }
    }
}
pub use internal::{CVPixelBuffer, CVPixelBufferRef};
