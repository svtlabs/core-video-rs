pub mod attributes;
pub mod error;
pub mod internal;
pub mod lock;
pub mod planar_data;

use attributes::PixelBufferAttributes;
use core_utils_rs::four_char_code::FourCharCode;
use error::CVPixelBufferError;
use io_surface::IOSurface;
use planar_data::PlanarDataPointer;

pub use internal::{CVPixelBuffer, CVPixelBufferRef};
impl CVPixelBuffer {
    pub fn is_planar(&self) -> bool {
        self.internal_is_planar()
    }
    pub fn get_bytes_per_row(&self) -> usize {
        self.internal_bytes_per_row()
    }
    pub fn get_width(&self) -> usize {
        self.internal_width()
    }
    pub fn get_height(&self) -> usize {
        self.internal_height()
    }

    pub fn create(
        width: usize,
        height: usize,
        pixel_format_type: FourCharCode,
        pixel_buffer_attributes: PixelBufferAttributes,
    ) -> Result<Self, CVPixelBufferError> {
        Self::internal_create(width, height, pixel_format_type, pixel_buffer_attributes)
    }

    pub fn create_with_io_surface(
        surface: &IOSurface,
        pixel_buffer_attributes: PixelBufferAttributes,
    ) -> Result<Self, CVPixelBufferError> {
        Self::internal_create_with_io_surface(surface, pixel_buffer_attributes)
    }
    pub fn create_with_planar_bytes(
        width: usize,
        height: usize,
        pixel_format_type: FourCharCode,
        data_pointer: PlanarDataPointer,
        pixel_buffer_attributes: PixelBufferAttributes,
    ) -> Result<Self, CVPixelBufferError> {
        Self::internal_create_with_planar_bytes(
            width,
            height,
            pixel_format_type,
            data_pointer,
            |_, _| {},
            (),
            pixel_buffer_attributes,
        )
    }

    pub fn create_with_bytes(
        width: usize,
        height: usize,
        pixel_format_type: FourCharCode,
        base_address: Vec<u8>,
        bytes_per_row: usize,
        pixel_buffer_attributes: PixelBufferAttributes,
    ) -> Result<Self, CVPixelBufferError> {
        Self::internal_create_with_bytes(
            width,
            height,
            pixel_format_type,
            base_address,
            bytes_per_row,
            |_, _| {},
            (),
            pixel_buffer_attributes,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn create_with_bytes_release_cb<TRefCon, TReleaseCallback>(
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
        TRefCon: 'static + Send,
        TReleaseCallback: 'static + Send + FnOnce(TRefCon, Vec<u8>),
    {
        Self::internal_create_with_bytes(
            width,
            height,
            pixel_format_type,
            base_address,
            bytes_per_row,
            release_callback,
            release_ref_con,
            pixel_buffer_attributes,
        )
    }

    pub fn create_with_planar_bytes_release_cb<TRefCon, TReleaseCallback>(
        width: usize,
        height: usize,
        pixel_format_type: FourCharCode,
        data_pointer: PlanarDataPointer,
        release_callback: TReleaseCallback,
        release_ref_con: TRefCon,
        pixel_buffer_attributes: PixelBufferAttributes,
    ) -> Result<Self, CVPixelBufferError>
    where
        TRefCon: 'static + Send,
        TReleaseCallback: 'static + Send + FnOnce(TRefCon, PlanarDataPointer) + 'static,
    {
        Self::internal_create_with_planar_bytes(
            width,
            height,
            pixel_format_type,
            data_pointer,
            release_callback,
            release_ref_con,
            pixel_buffer_attributes,
        )
    }
}
