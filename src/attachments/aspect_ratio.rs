use core_foundation::string::CFStringRef;

#[link(name = "CoreVideo", kind = "framework")]
extern "C" {

    // attachement keys
    pub static kCVImageBufferPixelAspectRatioHorizontalSpacingKey: CFStringRef;
    pub static kCVImageBufferPixelAspectRatioVerticalSpacingKey: CFStringRef;
}
