mod internal {
    #![allow(dead_code)]
    #![allow(clippy::too_many_arguments)]
    use crate::cv_pixel_buffer_attributes::PixelBufferAttributes;
    use crate::cv_pixel_buffer_error::{CVPixelBufferError, CV_RETURN_SUCCESS};
    use crate::cv_pixel_buffer_planar_data::CVPixelBufferPlanarDataPointer;
    use crate::types::{CVReturn, OSType};
    use core_foundation::base::{kCFAllocatorDefault, CFAllocatorRef, CFType, CFTypeID, TCFType};
    use core_foundation::dictionary::CFDictionaryRef;
    use core_foundation::string::CFString;
    use core_foundation::{declare_TCFType, impl_TCFType};
    use core_graphics::display::CFDictionary;
    use core_utils_rs::trampoline::{
        create_left_trampoline, TrampolineLeftCallback, TrampolineRefcon,
    };
    use four_char_code::FourCharCode;
    use io_surface::{IOSurface, IOSurfaceRef};
    use std::ffi::c_void;
    use std::ptr::{self, NonNull};

    #[repr(C)]
    pub struct __CVPixelBufferRef(c_void);

    pub type CVPixelBufferRef = *mut __CVPixelBufferRef;

    declare_TCFType! {CVPixelBuffer, CVPixelBufferRef}
    impl_TCFType!(CVPixelBuffer, CVPixelBufferRef, CVPixelBufferGetTypeID);

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
            base_address: *const u8,
            bytes_per_row: usize,
            raw_release_callback: TrampolineLeftCallback,
            raw_release_ref_con: TrampolineRefcon,
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
            release_callback: TrampolineLeftCallback,
            release_ref_con: TrampolineRefcon,
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
    impl CVPixelBuffer {
        pub fn create_with_planar_bytes(
            width: usize,
            height: usize,
            pixel_format_type: FourCharCode,
            data_pointer: CVPixelBufferPlanarDataPointer,
            pixel_buffer_attributes: PixelBufferAttributes,
        ) -> Result<Self, CVPixelBufferError> {
            let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();
            let data_size = 1234;
            let data_ptr = data_pointer.as_ptr();
            let number_of_planes = data_pointer.number_of_planes();
            let base_addresses = data_pointer.raw_base_addresses();
            let plane_width = data_pointer.plane_width();
            let plane_height = data_pointer.plane_height();
            let plane_bytes_per_row = data_pointer.plane_bytes_per_row();
            let (caller, closure) = create_left_trampoline(move |param| {
                println!("Release callback called, {:p}", &param);
            });
            let pixel_buffer_attributes: CFDictionary<CFString, CFType> =
                pixel_buffer_attributes.into();
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
                    pixel_buffer_attributes.as_concrete_TypeRef(),
                    &mut pixel_buffer_out,
                );

                if result == CV_RETURN_SUCCESS {
                    Ok(CVPixelBuffer::wrap_under_create_rule(pixel_buffer_out))
                } else {
                    Err(CVPixelBufferError::from(result))
                }
            }
        }
        pub fn create_with_planar_bytes_and_release_callback<TRefCon, TReleaseCallback>(
            width: usize,
            height: usize,
            pixel_format_type: FourCharCode,
            data_pointer: CVPixelBufferPlanarDataPointer,
            release_callback: TReleaseCallback,
            release_ref_con: TRefCon,
            pixel_buffer_attributes: PixelBufferAttributes,
        ) -> Result<Self, CVPixelBufferError>
        where
            TRefCon: 'static,
            TReleaseCallback:
                FnOnce(TRefCon, Option<Vec<u8>>, usize, usize, Vec<NonNull<[u8]>>) + 'static,
        {
            let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();
            let data_ptr = data_pointer.as_ptr();
            let data_size = data_pointer.data_size();
            let number_of_planes = data_pointer.number_of_planes();
            let base_addresses = data_pointer.raw_base_addresses();
            let plane_width = data_pointer.plane_width();
            let plane_height = data_pointer.plane_height();
            let plane_bytes_per_row = data_pointer.plane_bytes_per_row();
            let (caller, closure) = create_left_trampoline(move |_| {
                println!("Release callback called, {:?}", data_size);

                release_callback(
                    release_ref_con,
                    data_pointer.data,
                    data_size,
                    number_of_planes,
                    data_pointer.base_addresses,
                )
            });
            let pixel_buffer_attributes: CFDictionary<CFString, CFType> =
                pixel_buffer_attributes.into();
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
                    pixel_buffer_attributes.as_concrete_TypeRef(),
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
            pixel_buffer_attributes: PixelBufferAttributes,
        ) -> Result<Self, CVPixelBufferError> {
            if base_address.len() < bytes_per_row * height {
                return Err(CVPixelBufferError::InvalidSize);
            }

            let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();
            let base_address_ptr = base_address.as_ptr();
            let (caller, closure) = create_left_trampoline(|_| {
                println!("Release callback called");
            });

            unsafe {
                let pixel_buffer_attributes: CFDictionary<CFString, CFType> =
                    pixel_buffer_attributes.into();
                let result = CVPixelBufferCreateWithBytes(
                    kCFAllocatorDefault,
                    width,
                    height,
                    pixel_format_type.as_u32(),
                    base_address_ptr,
                    bytes_per_row,
                    caller,
                    closure,
                    pixel_buffer_attributes.as_concrete_TypeRef(),
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
            release_callback: TReleaseCallback,
            release_ref_con: TRefCon,
            pixel_buffer_attributes: PixelBufferAttributes,
        ) -> Result<Self, CVPixelBufferError>
        where
            TRefCon: 'static,
            TReleaseCallback: FnOnce(TRefCon, Vec<u8>) + 'static,
        {
            if base_address.len() < bytes_per_row * height {
                return Err(CVPixelBufferError::InvalidSize);
            }

            let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();
            let base_address_ptr = base_address.as_ptr();
            let (caller, closure) = create_left_trampoline(move |_| {
                release_callback(release_ref_con, base_address);
            });
            unsafe {
                let pixel_buffer_attributes: CFDictionary<CFString, CFType> =
                    pixel_buffer_attributes.into();
                let result = CVPixelBufferCreateWithBytes(
                    kCFAllocatorDefault,
                    width,
                    height,
                    pixel_format_type.as_u32(),
                    base_address_ptr,
                    bytes_per_row,
                    caller,
                    closure,
                    pixel_buffer_attributes.as_concrete_TypeRef(),
                    &mut pixel_buffer_out,
                );
                if result == CV_RETURN_SUCCESS {
                    Ok(CVPixelBuffer::wrap_under_create_rule(pixel_buffer_out))
                } else {
                    Err(CVPixelBufferError::from(result))
                }
            }
        }
        pub fn create_with_io_surface(
            surface: &IOSurface,
            pixel_buffer_attributes: PixelBufferAttributes,
        ) -> Result<CVPixelBuffer, CVPixelBufferError> {
            let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();
            let pixel_buffer_attributes: CFDictionary<CFString, CFType> =
                pixel_buffer_attributes.into();
            unsafe {
                let result = CVPixelBufferCreateWithIOSurface(
                    kCFAllocatorDefault,
                    surface.clone().as_concrete_TypeRef(),
                    pixel_buffer_attributes.as_concrete_TypeRef(),
                    &mut pixel_buffer_out,
                );
                if result == CV_RETURN_SUCCESS {
                    Ok(CVPixelBuffer::wrap_under_create_rule(pixel_buffer_out))
                } else {
                    Err(CVPixelBufferError::from(result))
                }
            }
        }

        pub fn is_planar(&self) -> bool {
            extern "C" {
                fn CVPixelBufferIsPlanar(pixel_buffer_ref: CVPixelBufferRef) -> i32;
            }

            unsafe { CVPixelBufferIsPlanar(self.as_concrete_TypeRef()) == 1 }
        }
        pub fn get_bytes_per_row(&self) -> usize {
            extern "C" {
                fn CVPixelBufferGetBytesPerRow(pixel_buffer_ref: CVPixelBufferRef) -> usize;
            }

            unsafe { CVPixelBufferGetBytesPerRow(self.as_concrete_TypeRef()) }
        }
        pub fn get_width(&self) -> usize {
            extern "C" {
                fn CVPixelBufferGetWidth(pixel_buffer_ref: CVPixelBufferRef) -> usize;
            }

            unsafe { CVPixelBufferGetWidth(self.as_concrete_TypeRef()) }
        }
        pub fn get_height(&self) -> usize {
            extern "C" {
                fn CVPixelBufferGetHeight(pixel_buffer_ref: CVPixelBufferRef) -> usize;
            }

            unsafe { CVPixelBufferGetHeight(self.as_concrete_TypeRef()) }
        }

        pub fn create(
            width: usize,
            height: usize,
            pixel_format_type: FourCharCode,
            pixel_buffer_attributes: PixelBufferAttributes,
        ) -> Result<Self, CVPixelBufferError> {
            let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();
            let pixel_buffer_attributes: CFDictionary<CFString, CFType> =
                pixel_buffer_attributes.into();
            unsafe {
                let result = CVPixelBufferCreate(
                    kCFAllocatorDefault,
                    width,
                    height,
                    pixel_format_type.as_u32(),
                    pixel_buffer_attributes.as_concrete_TypeRef(),
                    &mut pixel_buffer_out,
                );
                if result == CV_RETURN_SUCCESS {
                    Ok(CVPixelBuffer::wrap_under_create_rule(pixel_buffer_out))
                } else {
                    Err(CVPixelBufferError::from(result))
                }
            }
        }
    }
    #[cfg(test)]
    mod test {

        use core_foundation::number::CFNumber;
        use io_surface::{
            kIOSurfaceBytesPerElement, kIOSurfaceHeight, kIOSurfacePixelFormat, kIOSurfaceWidth,
        };
        use std::error::Error;

        use crate::cv_pixel_buffer_attributes::PixelBufferAttribute;

        const WIDTH: usize = 100;
        const BYTE_PER_ROW: usize = 100 * 4;
        const HEIGHT: usize = 100;
        const SIZE: usize = WIDTH * HEIGHT * 4;
        const PIXEL_VALUE: u8 = 0xF2;

        #[test]
        fn test_is_planar() -> Result<(), Box<dyn Error>> {
            use super::*;
            let pixel_buffer = CVPixelBuffer::create(
                WIDTH,
                HEIGHT,
                FourCharCode::from_str("BGRA").unwrap(),
                PixelBufferAttributes::default(),
            )?;

            assert!(!pixel_buffer.is_planar());

            let data = vec![PIXEL_VALUE; SIZE];
            let base_address = NonNull::from(data.as_slice());
            let pixel_buffer = CVPixelBuffer::create_with_planar_bytes(
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
                PixelBufferAttributes::default(),
            )?;
            assert!(pixel_buffer.is_planar());
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
            let pixel_buffer = CVPixelBuffer::create_with_io_surface(
                &io_surface,
                PixelBufferAttributes::default(),
            )?;

            assert_eq!(pixel_buffer.get_width(), WIDTH);
            assert_eq!(pixel_buffer.get_height(), HEIGHT);
            Ok(())
        }
        #[test]
        fn test_create() -> Result<(), Box<dyn Error>> {
            use super::*;
            let pixel_buffer = CVPixelBuffer::create(
                WIDTH,
                HEIGHT,
                FourCharCode::from_str("BGRA").unwrap(),
                PixelBufferAttributes::default(),
            )?;
            assert_eq!(pixel_buffer.get_width(), WIDTH);
            assert_eq!(pixel_buffer.get_height(), HEIGHT);
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
            let pixel_buffer = CVPixelBuffer::create_with_planar_bytes_and_release_callback(
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
                PixelBufferAttributes::default(),
            )?;
            assert_eq!(pixel_buffer.get_width(), WIDTH);
            assert_eq!(pixel_buffer.get_height(), HEIGHT);
            Ok(())
        }

        #[test]
        fn test_create_with_bytes_and_release() -> Result<(), Box<dyn Error>> {
            use super::*;
            let move_into_closure = vec![1, 2, 3];
            let pixel_buffer = CVPixelBuffer::create_with_bytes_with_release(
                WIDTH,
                HEIGHT,
                FourCharCode::from_str("BGRA").unwrap(),
                vec![PIXEL_VALUE; SIZE],
                BYTE_PER_ROW,
                move |refcon, address| {
                    println!(
                        "Release callback called:{:?} {:?} {:p}",
                        move_into_closure, refcon, &address
                    );
                },
                32,
                PixelBufferAttributes::default(),
            )?;
            assert_eq!(pixel_buffer.get_width(), WIDTH);
            assert_eq!(pixel_buffer.get_height(), HEIGHT);
            Ok(())
        }

        #[test]
        fn test_create_with_bytes() -> Result<(), Box<dyn Error>> {
            use super::*;
            // Create pixel buffer attributes
            let pixel_buffer_attributes = PixelBufferAttributes::new(&[
                PixelBufferAttribute::Width(WIDTH),
                PixelBufferAttribute::Height(HEIGHT),
            ]);
            let pixel_buffer = CVPixelBuffer::create_with_bytes(
                WIDTH,
                HEIGHT,
                FourCharCode::from_str("BGRA").unwrap(),
                vec![PIXEL_VALUE; SIZE],
                BYTE_PER_ROW,
                pixel_buffer_attributes,
            )?;

            assert_eq!(pixel_buffer.get_width(), WIDTH);
            assert_eq!(pixel_buffer.get_height(), HEIGHT);
            Ok(())
        }
    }
}
pub use internal::{CVPixelBuffer, CVPixelBufferRef};
