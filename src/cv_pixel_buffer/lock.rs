use std::ops::{Deref, DerefMut};

use core_utils_rs::lock::{
    self, LockGuard, LockGuardTrait, LockTrait, MutLockGuard, MutLockGuardTrait, MutLockTrait,
};

use crate::cv_pixel_buffer::{error::CVPixelBufferError, CVPixelBuffer};

use super::internal::CVPixelBufferLockFlags;

#[derive(Debug)]
pub struct BaseAddressGuard<'a>(CVPixelBuffer, Option<&'a[u8]>, Option<&'a mut[u8]>) ;

impl Deref for BaseAddressGuard<'_> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.1.expect("Could not get base address")
    }
}

impl DerefMut for BaseAddressGuard<'_> {
    
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.2.as_mut().expect("Could not get base address")
    }
}
impl <'a> LockGuardTrait for BaseAddressGuard<'a> {
    fn unlock(&self) {
        self.0
            .internal_unlock_base_address(CVPixelBufferLockFlags::ReadOnly)
            .expect("Could not unlock base");
    }
}
impl <'a> MutLockGuardTrait for BaseAddressGuard<'a> {
    fn unlock_mut(&mut self) {
        self.0
            .internal_unlock_base_address(CVPixelBufferLockFlags::ReadWrite)
            .expect("Could not unlock base");
    }
}

impl <'a> LockTrait<BaseAddressGuard<'a>, CVPixelBufferError> for CVPixelBuffer {
    fn lock(&self) -> Result<LockGuard<BaseAddressGuard<'a>>, CVPixelBufferError> {
        self.internal_lock_base_address(CVPixelBufferLockFlags::ReadOnly)?;
        Ok(LockGuard(BaseAddressGuard(
            self.clone(),
            self.internal_base_address().ok(),
            None
                    
        )))
    }
}
impl <'a> MutLockTrait<BaseAddressGuard<'a>, CVPixelBufferError> for CVPixelBuffer {
    fn lock_mut(&mut self) -> Result<lock::MutLockGuard<BaseAddressGuard<'a>>, CVPixelBufferError> {
        self.internal_lock_base_address(CVPixelBufferLockFlags::ReadWrite)?;
        Ok(MutLockGuard(BaseAddressGuard(
            self.clone(),
            None,
            self.internal_base_address_mut().ok()
        )))
    }
}
