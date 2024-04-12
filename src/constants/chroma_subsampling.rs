use core_foundation::string::CFStringRef;

#[link(name = "CoreVideo", kind = "framework")]
extern "C" {
    pub static kCVImageBufferChromaSubsampling_420: CFStringRef;
    pub static kCVImageBufferChromaSubsampling_422: CFStringRef;
    pub static kCVImageBufferChromaSubsampling_411: CFStringRef;

}
