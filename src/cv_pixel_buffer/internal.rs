#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]
use crate::cv_pixel_buffer::error::CV_RETURN_SUCCESS;
use crate::types::{CVReturn, OSType};
use core::fmt;
use core_foundation::base::{kCFAllocatorDefault, CFAllocatorRef, CFType, CFTypeID, TCFType};
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::string::CFString;
use core_foundation::{declare_TCFType, impl_TCFType};
use core_graphics::display::CFDictionary;
use core_utils_rs::four_char_code::FourCharCode;
use core_utils_rs::trampoline::{create_left_trampoline, TrampolineLeftCallback, TrampolineRefcon};
use io_surface::{IOSurface, IOSurfaceRef};
use std::ffi::c_void;
use std::fmt::Formatter;
use std::ptr::{self};

use super::attributes::PixelBufferAttributes;
use super::error::CVPixelBufferError;
use super::planar_data::PlanarDataPointer;

#[repr(C)]
pub struct __CVPixelBufferRef(c_void);

pub type CVPixelBufferRef = *mut __CVPixelBufferRef;

declare_TCFType! {CVPixelBuffer, CVPixelBufferRef}
impl_TCFType!(CVPixelBuffer, CVPixelBufferRef, CVPixelBufferGetTypeID);

impl fmt::Debug for CVPixelBuffer {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "CVPixelBuffer")
    }
}

extern "C" {
    fn CVPixelBufferGetTypeID() -> CFTypeID;
}

impl CVPixelBuffer {
    pub(super) fn internal_create_with_planar_bytes<TRefCon, TReleaseCallback>(
        width: usize,
        height: usize,
        pixel_format_type: FourCharCode,
        data_pointer: PlanarDataPointer,
        release_callback: TReleaseCallback,
        release_ref_con: TRefCon,
        pixel_buffer_attributes: PixelBufferAttributes,
    ) -> Result<Self, CVPixelBufferError>
    where
        TRefCon: 'static,
        TReleaseCallback: FnOnce(TRefCon, PlanarDataPointer) + 'static,
    {
        extern "C" {
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

        }
        let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();
        let data_ptr = data_pointer.as_ptr();
        let data_size = data_pointer.data_size();
        let number_of_planes = data_pointer.number_of_planes();
        let base_addresses = data_pointer.raw_base_addresses();
        let plane_width = data_pointer.plane_width();
        let plane_height = data_pointer.plane_height();
        let plane_bytes_per_row = data_pointer.plane_bytes_per_row();
        let (caller, closure) =
            create_left_trampoline(move |_| release_callback(release_ref_con, data_pointer));
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
    pub(super) fn internal_create_with_bytes<TRefCon, TReleaseCallback>(
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
        extern "C" {
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

        }
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
    pub(super) fn internal_create_with_io_surface(
        surface: &IOSurface,
        pixel_buffer_attributes: PixelBufferAttributes,
    ) -> Result<CVPixelBuffer, CVPixelBufferError> {
        extern "C" {
            fn CVPixelBufferCreateWithIOSurface(
                allocator: CFAllocatorRef,
                surface: IOSurfaceRef,
                pixelBufferAttributes: CFDictionaryRef,
                pixelBufferOut: *mut CVPixelBufferRef,
            ) -> CVReturn;

        }
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

    pub(super) fn internal_create(
        width: usize,
        height: usize,
        pixel_format_type: FourCharCode,
        pixel_buffer_attributes: PixelBufferAttributes,
    ) -> Result<Self, CVPixelBufferError> {
        extern "C" {
            fn CVPixelBufferCreate(
                allocator: CFAllocatorRef,
                width: usize,
                height: usize,
                pixelFormatType: OSType,
                pixelBufferAttributes: CFDictionaryRef,
                pixelBufferOut: *mut CVPixelBufferRef,
            ) -> CVReturn;
        }
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
#[repr(u32)]
pub(super) enum CVPixelBufferLockFlags {
    ReadWrite = 0x0,
    ReadOnly = 0x00000001,
}
impl CVPixelBuffer {
    pub(super) fn internal_lock_base_address(
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

    pub(super) fn internal_unlock_base_address(
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
    pub(super) fn internal_base_address<'a>(&self) -> Result<&'a [u8], CVPixelBufferError> {
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
    pub(super) fn internal_base_address_mut<'a>(&self) -> Result<&'a mut [u8], CVPixelBufferError> {
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
    pub(super) fn internal_is_planar(&self) -> bool {
        extern "C" {
            fn CVPixelBufferIsPlanar(pixel_buffer_ref: CVPixelBufferRef) -> i32;
        }

        unsafe { CVPixelBufferIsPlanar(self.as_concrete_TypeRef()) == 1 }
    }
    pub(super) fn internal_bytes_per_row(&self) -> usize {
        extern "C" {
            fn CVPixelBufferGetBytesPerRow(pixel_buffer_ref: CVPixelBufferRef) -> usize;
        }

        unsafe { CVPixelBufferGetBytesPerRow(self.as_concrete_TypeRef()) }
    }
    pub(super) fn internal_width(&self) -> usize {
        extern "C" {
            fn CVPixelBufferGetWidth(pixel_buffer_ref: CVPixelBufferRef) -> usize;
        }

        unsafe { CVPixelBufferGetWidth(self.as_concrete_TypeRef()) }
    }
    pub(super) fn internal_height(&self) -> usize {
        extern "C" {
            fn CVPixelBufferGetHeight(pixel_buffer_ref: CVPixelBufferRef) -> usize;
        }

        unsafe { CVPixelBufferGetHeight(self.as_concrete_TypeRef()) }
    }
}
