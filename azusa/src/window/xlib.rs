use std::ffi::c_ulong;
use crate::Color;
use crate::window::Platform;
use safex::xlib::*;

pub struct XLibPlatform {
    display: Display,
    window: Window
}

impl XLibPlatform {
    pub fn new(window: c_ulong) -> Self {
        let display = Display::open(None);
        let screen = Screen::default(&display);
        let window = unsafe { Window::from_raw(&display, &screen, window, Some(())) };
        Self {
            display,
            window
        }
    }
}

impl Platform for XLibPlatform {
    fn begin(&mut self) {

    }

    fn clear(&mut self, color: Color) {
        let (r,g,b,a) = color.into();
        let geometry = self.window.get_geometry();
        let rect = Rectangle {
            x: 0,
            y: 0,
            width: geometry.width,
            height: geometry.height,
            pixel: Pixel::from_rgb(r,g,b),
        };

        self.window.fill_rectangle(rect);
    }

    fn fill_rectangle(&mut self,x:u32,y:u32,width:u32,height:u32,color: Color,border_color:Color) {
        let (r,g,b,a) = color.into();
        let rect = Rectangle {
            x,
            y,
            width,
            height,
            pixel: Pixel::from_rgb(r,g,b),
        };

        self.window.fill_rectangle(rect);
    }

    fn end(&mut self) {
        self.window.copy_to_buffer();
    }
}