mod gdi;
mod xlib;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use crate::{Color, DrawTarget, ISurface, Surface};

pub(crate) trait Platform {
    fn begin(&mut self);
    fn clear(&mut self,color: Color);
    fn fill_rectangle(&mut self,x:u32,y:u32,width:u32,height:u32,color: Color,border_color:Color);
    fn end(&mut self);
}

#[repr(C)]
pub struct WindowSurface {
    platform: Box<dyn Platform>
}

impl WindowSurface {
    pub fn new(handle: &impl HasRawWindowHandle) -> Surface {
        Surface {
            surface: Box::new(Self::new_from_handle(handle.raw_window_handle())),
        }
    }

    fn new_from_handle(handle:RawWindowHandle) -> Self {
        match handle {
            RawWindowHandle::UiKit(_) => panic!(""),
            RawWindowHandle::AppKit(_) => panic!(""),
            RawWindowHandle::Orbital(_) => panic!(""),
            RawWindowHandle::Xlib(handle) => {
                Self {
                    platform: Box::new(xlib::XLibPlatform::new(handle.window)),
                }
            },
            RawWindowHandle::Xcb(_) => panic!(""),
            RawWindowHandle::Wayland(_) => panic!(""),
            RawWindowHandle::Drm(_) => panic!(""),
            RawWindowHandle::Gbm(_) => panic!(""),
            RawWindowHandle::Win32(handle) => {
                Self {
                    platform: Box::new(gdi::GDIPlatform::new(handle.hwnd)),
                }
            }
            RawWindowHandle::WinRt(_) => panic!(""),
            RawWindowHandle::Web(_) => panic!(""),
            RawWindowHandle::AndroidNdk(_) => panic!(""),
            RawWindowHandle::Haiku(_) => panic!(""),
            _ => panic!("")
        }
    }
}

impl From<RawWindowHandle> for WindowSurface {
    fn from(value: RawWindowHandle) -> Self {
        Self::new_from_handle(value)
    }
}

impl ISurface for WindowSurface {
    fn draw(&mut self, target: &mut Vec<DrawTarget>) {
        self.platform.begin();
        for i in target {
            match i {
                DrawTarget::Clear(color) => {
                    self.platform.clear(*color);
                }
                DrawTarget::DrawPoint(_, _, _) => {}
                DrawTarget::DrawRectangle(_, _, _, _, _, _) => {}
                DrawTarget::FillRectangle(x,y,width,height,color) => {
                    self.platform.fill_rectangle(*x,*y,*width,*height,*color,*color);
                }
            }
        }
        self.platform.end();
    }
}