use std::cell::RefCell;
use std::ffi::{c_int, c_void};
use winapi::shared::windef::{DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, HBITMAP, HDC, HGDIOBJ, HWND, RECT};
use winapi::um::wingdi::{BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DC_BRUSH, DC_PEN, DeleteDC, DeleteObject, GetStockObject, Rectangle, RGB, SelectObject, SetDCBrushColor, SetDCPenColor, SRCCOPY};
use winapi::um::winuser::{GetClientRect, GetDC, ReleaseDC, SetProcessDpiAwarenessContext};
use crate::Color;
use crate::window::Platform;

pub struct GDIPlatform {
    hwnd: HWND,
    hdc: HDC,
    dc: HDC,

    bitmap: HBITMAP,
    obmp: HGDIOBJ,

    rect: RECT,
    clear_color: Color,
}

impl GDIPlatform {
    pub fn new(hwnd: *mut c_void) -> Self {
        unsafe {
            SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
        }

        Self {
            hwnd: hwnd as HWND,
            hdc: 0 as HDC,
            dc: 0 as HDC,
            bitmap: 0 as HBITMAP,
            obmp: 0 as HGDIOBJ,
            rect: RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
            clear_color: Color::Rgba(0,0,0,255),
        }
    }

    #[inline]
    fn set_color(&self, color: Color, border_color: Color) {
        unsafe {
            let (r,g,b,a) = color.into();
            SetDCBrushColor(self.hdc, RGB(r,g,b));
            let (r,g,b,a) = border_color.into();
            SetDCPenColor(
                self.hdc,
                RGB(
                    r,g,b
                ),
            );

            SelectObject(self.hdc, GetStockObject(DC_PEN as c_int));
            SelectObject(self.hdc, GetStockObject(DC_BRUSH as c_int));
        }
    }
}

impl Platform for GDIPlatform {
    fn begin(&mut self) {
        unsafe {
            GetClientRect(self.hwnd, &mut self.rect);

            self.dc = GetDC(self.hwnd);
            self.hdc = CreateCompatibleDC(self.dc);
            self.bitmap = CreateCompatibleBitmap(self.dc, self.rect.right, self.rect.bottom);
            self.obmp = SelectObject(self.hdc, self.bitmap as HGDIOBJ);
        }
    }

    fn clear(&mut self, color: Color) {
        self.clear_color = color;

        unsafe {
            self.set_color(color, color);
            Rectangle(
                self.hdc,
                self.rect.left,
                self.rect.top,
                self.rect.right,
                self.rect.bottom,
            );
        }
    }

    fn end(&mut self) {
        unsafe {
            BitBlt(
                self.dc,
                self.rect.left,
                self.rect.top,
                self.rect.right,
                self.rect.bottom,
                self.hdc,
                0,
                0,
                SRCCOPY,
            );
            SelectObject(self.hdc, self.obmp);

            DeleteDC(self.hdc);
            DeleteObject(self.bitmap as HGDIOBJ);

            ReleaseDC(self.hwnd, self.dc);
        }
    }
}