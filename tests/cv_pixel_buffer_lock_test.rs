use std::{error::Error, io::Write};

use core_utils_rs::four_char_code::FourCharCode;
use core_utils_rs::lock::{LockTrait, MutLockTrait};

use core_video_rs::cv_pixel_buffer::{attributes::PixelBufferAttributes, CVPixelBuffer};
const WIDTH: usize = 10;
const HEIGHT: usize = 10;

#[test]
fn test_create_lock_write() -> Result<(), Box<dyn Error>> {
    let mut pixel_buffer = CVPixelBuffer::create(
        WIDTH,
        HEIGHT,
        FourCharCode::from_str("BGRA").unwrap(),
        PixelBufferAttributes::default(),
    )?;

    {
        let mut b = pixel_buffer.lock_mut()?;
        b.fill(2);
        println!("{:?}", b);
    };
    {
        let  b = pixel_buffer.lock()?;
               
        println!("{:?}", b);
    };
    Ok(())
}
