use core_foundation::string::CFStringRef;

#[link(name = "CoreVideo", kind = "framework")]
extern "C" {

    // attachement keys
    pub static kCVImageBufferCleanApertureWidthKey: CFStringRef;
    pub static kCVImageBufferCleanApertureHeightKey: CFStringRef;
    pub static kCVImageBufferCleanApertureHorizontalOffsetKey: CFStringRef;
    pub static kCVImageBufferCleanApertureVerticalOffsetKey: CFStringRef;
}
