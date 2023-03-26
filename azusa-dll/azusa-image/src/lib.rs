use std::ffi::{c_char, CStr};
use azusa::png::PngSurface;
use azusa::Surface;

#[no_mangle]
pub extern "C" fn azusa_new_image_surface(file_name: *const c_char,width:u32,height:u32) -> *mut Surface {
    unsafe {
        let surface = PngSurface::new(CStr::from_ptr(file_name).to_str().unwrap(),width,height);
        let box_suface = Box::new(surface);
        Box::into_raw(box_suface)
    }
}