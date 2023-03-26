use crate::window::Platform;
use crate::Color;
use safex::xlib::*;
use std::ffi::c_ulong;

pub struct XLibPlatform {
    display: Display,
    window: Window,
    cmap: ColorMap
}

impl XLibPlatform {
    pub fn new(window: c_ulong) -> Self {
        let display = Display::open(None);
        let screen = Screen::default(&display);
        let window = unsafe { Window::from_raw(&display, &screen, window, Some(())) };
        let cmap = ColorMap::default(&display, &screen);
        Self { display, window, cmap }
    }
}

impl Platform for XLibPlatform {
    fn begin(&mut self) {
        self.window.flush_gc();
        self.display.flush();
    }

    fn clear(&mut self, color: Color) {
        let (r, g, b, a) = color.into();
        let geometry = self.window.get_geometry();
        let cmap = ColorMap::default(&display, &screen);
        let color = safex::xlib::Color::from_rgb(&display, &cmap, (r as u16)*257,(g as u16)*257,(b as u16)*257).get_pixel();
        let rect = Rectangle {
            x: 0,
            y: 0,
            width: geometry.width,
            height: geometry.height,
            pixel: color,
        };

        self.window.fill_rectangle(rect);
    }

    fn fill_rectangle(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        color: Color,
        border_color: Color,
    ) {
        let (r, g, b, a) = color.into();
        let color = safex::xlib::Color::from_rgb(&display, &cmap, (r as u16)*257,(g as u16)*257,(b as u16)*257).get_pixel();
        let rect = Rectangle {
            x,
            y,
            width,
            height,
            pixel: color,
        };

        self.window.fill_rectangle(rect);
    }

    fn end(&mut self) {
        self.window.copy_to_buffer();
    }
}
