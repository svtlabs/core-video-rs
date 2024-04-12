use core_foundation::string::CFStringRef;

#[link(name = "CoreVideo", kind = "framework")]
extern "C" {

    // constants
    pub static kCVImageBufferFieldDetailTemporalTopFirst: CFStringRef;
    pub static kCVImageBufferFieldDetailTemporalBottomFirst: CFStringRef;
    pub static kCVImageBufferFieldDetailSpatialFirstLineEarly: CFStringRef;
    pub static kCVImageBufferFieldDetailSpatialFirstLineLate: CFStringRef;
}
