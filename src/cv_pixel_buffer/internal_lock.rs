use core_foundation::base::TCFType;

use crate::{cv_pixel_buffer::{error::CV_RETURN_SUCCESS, internal_base::CVPixelBufferRef}, types::CVReturn};

use super::{error::CVPixelBufferError, internal_base::CVPixelBuffer};

#[repr(u32)]
pub enum CVPixelBufferLockFlags {
    ReadWrite = 0x0,
    ReadOnly = 0x00000001,
}
impl<'a> CVPixelBuffer<'a> {
    pub fn internal_lock_base_address(
        &self,
        lock_flags: CVPixelBufferLockFlags,
    ) -> Result<(), CVPixelBufferError> {
        extern "C" {
            fn CVPixelBufferLockBaseAddress(
                pixelBuffer: CVPixelBufferRef,
                lockFlags: CVPixelBufferLockFlags,
            ) -> CVReturn;
        }

        let result =
            unsafe { CVPixelBufferLockBaseAddress(self.as_concrete_TypeRef(), lock_flags) };
        if result == CV_RETURN_SUCCESS {
            Ok(())
        } else {
            Err(CVPixelBufferError::from(result))
        }
    }

    pub fn internal_unlock_base_address(
        &self,
        unlock_flags: CVPixelBufferLockFlags,
    ) -> Result<(), CVPixelBufferError> {
        extern "C" {
            fn CVPixelBufferUnlockBaseAddress(
                pixelBuffer: CVPixelBufferRef,
                unlockFlags: CVPixelBufferLockFlags,
            ) -> CVReturn;
        }

        let result =
            unsafe { CVPixelBufferUnlockBaseAddress(self.as_concrete_TypeRef(), unlock_flags) };
        if result == CV_RETURN_SUCCESS {
            Ok(())
        } else {
            Err(CVPixelBufferError::from(result))
        }
    }
    pub fn internal_base_address(&self) -> Result<&'a [u8], CVPixelBufferError> {
        extern "C" {
            fn CVPixelBufferGetBaseAddress(pixelBuffer: CVPixelBufferRef) -> *mut u8;
        }

        let result = unsafe { CVPixelBufferGetBaseAddress(self.as_concrete_TypeRef()) };
        if result.is_null() {
            Err(CVPixelBufferError::BaseAddress)
        } else {
            let size = self.internal_bytes_per_row() * self.internal_height();
            Ok(unsafe { std::slice::from_raw_parts(result, size) })
        }
    }
    pub fn internal_base_address_mut(&self) -> Result<&'a mut [u8], CVPixelBufferError> {
        extern "C" {
            fn CVPixelBufferGetBaseAddress(pixelBuffer: CVPixelBufferRef) -> *mut u8;
        }

        let result = unsafe { CVPixelBufferGetBaseAddress(self.as_concrete_TypeRef()) };
        if result.is_null() {
            Err(CVPixelBufferError::BaseAddress)
        } else {
            let size = self.internal_bytes_per_row() * self.internal_height();
            Ok(unsafe { std::slice::from_raw_parts_mut(result, size) })
        }
    }
}
