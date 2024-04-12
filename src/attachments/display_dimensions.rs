use core_foundation::string::CFStringRef;

#[link(name = "CoreVideo", kind = "framework")]
extern "C" {

    // attachement keys
    pub static kCVImageBufferDisplayWidthKey: CFStringRef;
    pub static kCVImageBufferDisplayHeightKey: CFStringRef;
}
