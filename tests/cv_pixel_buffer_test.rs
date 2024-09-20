use core_foundation::{base::TCFType, number::CFNumber, string::CFString};
use core_graphics::display::CFDictionary;
use core_video_rs::cv_pixel_buffer::{
    attributes::PixelBufferAttributes, planar_data::PlanarDataPointer, CVPixelBuffer,
};
use four_char_code::FourCharCode;
use io_surface::{
    kIOSurfaceBytesPerElement, kIOSurfaceHeight, kIOSurfacePixelFormat, kIOSurfaceWidth,
};
use std::{error::Error, ptr::NonNull};

const WIDTH: usize = 100;
const BYTE_PER_ROW: usize = 100 * 4;
const HEIGHT: usize = 100;
const SIZE: usize = WIDTH * HEIGHT * 4;
const PIXEL_VALUE: u8 = 0xF2;

#[test]
fn test_create_with_io_surface() -> Result<(), Box<dyn Error>> {
    let pixel_buffer = CVPixelBuffer::create_with_io_surface(
        &io_surface::new(&unsafe {
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
        }),
        PixelBufferAttributes::default(),
    )?;

    assert_eq!(pixel_buffer.get_width(), WIDTH);
    assert_eq!(pixel_buffer.get_height(), HEIGHT);
    Ok(())
}
#[test]
fn test_create() -> Result<(), Box<dyn Error>> {
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
    let data = vec![PIXEL_VALUE; SIZE];
    let base_address = NonNull::from(data.as_slice());
    let expected_data = data.clone();
    let b = TestMoveStruct { var1: 33 };
    let pixel_buffer = {
        CVPixelBuffer::create_with_planar_bytes_release_cb(
            WIDTH,
            HEIGHT,
            FourCharCode::from_str("BGRA").unwrap(),
            PlanarDataPointer::new(
                Some(data),
                vec![BYTE_PER_ROW, BYTE_PER_ROW],
                vec![WIDTH, WIDTH],
                vec![HEIGHT, HEIGHT],
                vec![base_address, base_address],
            ),
            move |refcon, data| {
                assert_eq!(refcon, 1337);
                assert_eq!(b.var1, 33);
                assert_eq!(data.data_size(), SIZE);
                assert_eq!(data.number_of_planes(), 2);
                assert_eq!(data.data.unwrap(), expected_data);
                assert_eq!(data.base_addresses.len(), 2);
            },
            1337,
            PixelBufferAttributes::default(),
        )
    }?;
    assert!(pixel_buffer.is_planar());
    assert_eq!(pixel_buffer.get_width(), WIDTH);
    assert_eq!(pixel_buffer.get_height(), HEIGHT);
    Ok(())
}

#[test]
fn test_create_with_bytes_and_release() -> Result<(), Box<dyn Error>> {
    let move_into_closure = vec![1, 2, 3];
    let pixel_buffer = CVPixelBuffer::create_with_bytes_release_cb(
        WIDTH,
        HEIGHT,
        FourCharCode::from_str("BGRA").unwrap(),
        vec![PIXEL_VALUE; SIZE],
        BYTE_PER_ROW,
        move |refcon, address| {
            assert_eq!(move_into_closure, vec![1, 2, 3]);
            assert_eq!(refcon, 32);
            assert!(!address.is_empty());
        },
        32,
        PixelBufferAttributes::default(),
    )?;
    assert_eq!(pixel_buffer.get_width(), WIDTH);
    assert_eq!(pixel_buffer.get_height(), HEIGHT);
    Ok(())
}
