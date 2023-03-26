
use azusa::{Azusa, Surface};


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Into<azusa::Color> for Color {
    fn into(self) -> azusa::Color {
        azusa::Color::Rgba(self.r, self.g, self.b, self.a)
    }
}

#[no_mangle]
pub extern "C" fn azusa_new() -> *mut Azusa {
    let azusa = Azusa::new();
    let box_azusa = Box::new(azusa);
    Box::into_raw(box_azusa)
}

#[no_mangle]
pub extern "C" fn azusa_move_to(azusa: *mut Azusa, x: u32, y: u32) {
    unsafe {
        let azusa = &*azusa;
        azusa.move_to(x, y);
    }
}

#[no_mangle]
pub extern "C" fn azusa_clear(azusa: *mut Azusa, color: Color) {
    unsafe {
        let azusa = &*azusa;
        azusa.clear(color.into());
    }
}

#[no_mangle]
pub extern "C" fn azusa_draw_point(azusa: *mut Azusa, color: Color) {
    unsafe {
        let azusa = &*azusa;
        azusa.draw_point(color.into());
    }
}

#[no_mangle]
pub extern "C" fn azusa_draw_rectangle(
    azusa: *mut Azusa,
    color: Color,
    width: u32,
    height: u32,
    thickness: u32,
) {
    unsafe {
        let azusa = &*azusa;
        azusa.draw_rectangle(color.into(), width, height, thickness);
    }
}

#[no_mangle]
pub extern "C" fn azusa_flush(azusa: *mut Azusa, surface: *mut Surface) {
    unsafe {
        let azusa = &*azusa;
        let mut surface = &mut *surface;
        azusa.flush(&mut surface);
    }
}
