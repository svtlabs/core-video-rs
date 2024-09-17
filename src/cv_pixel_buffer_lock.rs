use std::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use core_foundation::base::TCFType;
use core_utils_rs::lock::{
    self, LockGuard, LockGuardTrait, LockTrait, MutLockGuard, MutLockGuardTrait, MutLockTrait,
};

use crate::{
    cv_pixel_buffer::{CVPixelBuffer, CVPixelBufferRef},
    cv_pixel_buffer_error::{CVPixelBufferError, CV_RETURN_SUCCESS},
    types::CVReturn,
};

#[repr(isize)]
enum CVPixelBufferLockFlags {
    ReadWrite = 0x0,
    ReadOnly = 0x00000001,
}
fn lock_base_address(
    pix_buf: &CVPixelBuffer,
    lock_flags: CVPixelBufferLockFlags,
) -> Result<(), CVPixelBufferError> {
    extern "C" {
        fn CVPixelBufferLockBaseAddress(
            pixelBuffer: CVPixelBufferRef,
            lockFlags: CVPixelBufferLockFlags,
        ) -> CVReturn;
    }

    let result =
        unsafe { CVPixelBufferLockBaseAddress(pix_buf.clone().as_concrete_TypeRef(), lock_flags) };
    if result == CV_RETURN_SUCCESS {
        Ok(())
    } else {
        Err(CVPixelBufferError::from(result))
    }
}

fn get_base_address(pix_buf: &CVPixelBuffer) -> Result<NonNull<u8>, CVPixelBufferError> {
    extern "C" {
        fn CVPixelBufferGetBaseAddress(pixelBuffer: CVPixelBufferRef) -> *mut u8;
    }

    let result = unsafe { CVPixelBufferGetBaseAddress(pix_buf.clone().as_concrete_TypeRef()) };
    if result.is_null() {
        Err(CVPixelBufferError::BaseAddress)
    } else {
        Ok(NonNull::new(result).unwrap())
    }
}

fn unlock_base_address(
    pix_buf: &CVPixelBuffer,
    unlock_flags: CVPixelBufferLockFlags,
) -> Result<(), CVPixelBufferError> {
    extern "C" {
        fn CVPixelBufferUnlockBaseAddress(
            pixelBuffer: CVPixelBufferRef,
            unlockFlags: CVPixelBufferLockFlags,
        ) -> CVReturn;
    }

    let result = unsafe {
        CVPixelBufferUnlockBaseAddress(pix_buf.clone().as_concrete_TypeRef(), unlock_flags)
    };
    if result == CV_RETURN_SUCCESS {
        Ok(())
    } else {
        Err(CVPixelBufferError::from(result))
    }
}

struct BaseAddressGuard(CVPixelBuffer, pub NonNull<u8>);

impl Deref for BaseAddressGuard {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.1.as_ptr(), 10) }
    }
}

impl DerefMut for BaseAddressGuard {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.1.as_ptr(), 10) }
    }
}

impl LockGuardTrait for BaseAddressGuard {
    fn unlock(&self) {
        unlock_base_address(&self.0, CVPixelBufferLockFlags::ReadOnly)
            .expect("Could not unlock base");
    }
}
impl MutLockGuardTrait for BaseAddressGuard {
    fn unlock_mut(&mut self) {
        unlock_base_address(&self.0, CVPixelBufferLockFlags::ReadWrite)
            .expect("Could not unlock base");
    }
}

impl LockTrait<BaseAddressGuard, CVPixelBufferError> for CVPixelBuffer {
    fn lock(&self) -> Result<LockGuard<BaseAddressGuard>, CVPixelBufferError> {
        lock_base_address(self, CVPixelBufferLockFlags::ReadOnly)?;
        let base_address = get_base_address(self)?;
        Ok(LockGuard(BaseAddressGuard(self.clone(), base_address)))
    }
}
impl MutLockTrait<BaseAddressGuard, CVPixelBufferError> for CVPixelBuffer {
    fn lock_mut(&mut self) -> Result<lock::MutLockGuard<BaseAddressGuard>, CVPixelBufferError> {
        lock_base_address(self, CVPixelBufferLockFlags::ReadWrite)?;
        let base_address = get_base_address(self)?;
        Ok(MutLockGuard(BaseAddressGuard(self.clone(), base_address)))
    }
}

#[cfg(test)]
mod tests {
    use core_utils_rs::lock::LockTrait;
    use four_char_code::FourCharCode;

    use crate::{
        cv_pixel_buffer::CVPixelBuffer, cv_pixel_buffer_attributes::PixelBufferAttributes,
    };

    #[test]
    fn test_lock() {
        let pb = CVPixelBuffer::create(
            18,
            8 ,
            FourCharCode::from_str("BGRA").unwrap(),
            PixelBufferAttributes::default(),
        )
        .unwrap();
        println!("AA:{}", pb.get_bytes_per_row());
    }

    #[test]
    fn test_lock_mut() {}
}
