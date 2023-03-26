#[cfg(feature = "png")]
pub mod png;
#[cfg(feature = "window")]
pub mod window;

use std::cell::RefCell;

#[repr(C)]
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Color {
    Rgba(u8,u8,u8,u8)
}

impl Into<(u8,u8,u8,u8)> for Color {
    fn into(self) -> (u8, u8, u8, u8) {
        match self {
            Color::Rgba(r,g,b,a) => (r,g,b,a)
        }
    }
}

#[repr(C)]
pub struct Surface {
    pub(crate) surface: Box<dyn ISurface>
}

trait ISurface {
    fn draw(&mut self,target: &mut Vec<DrawTarget>);
}

#[repr(C)]
enum DrawTarget {
    Clear(Color),
    DrawPoint(u32,u32,Color),
    DrawRectangle(u32,u32,u32,u32,u32,Color),
    FillRectangle(u32,u32,u32,u32,Color)
}

#[repr(C)]
pub struct Azusa {
    x: RefCell<u32>,
    y: RefCell<u32>,
    target: RefCell<Vec<DrawTarget>>
}

impl Azusa {
    pub fn new() -> Self {
        Self {
            x: RefCell::new(0),
            y: RefCell::new(0),
            target: RefCell::new(vec![])
        }
    }

    pub fn move_to(&self,x:u32,y:u32) {
        (*self.x.borrow_mut()) = x;
        (*self.y.borrow_mut()) = y;
    }

    pub fn clear(&self,color: Color) {
        self.target.borrow_mut().push(DrawTarget::Clear(color));
    }

    pub fn draw_point(&self,color: Color) {
        self.target.borrow_mut().push(DrawTarget::DrawPoint(*self.x.borrow_mut(), *self.y.borrow_mut(), color));
    }

    pub fn draw_rectangle(&self,color: Color,width:u32,height:u32,thickness:u32) {
        self.target.borrow_mut().push(DrawTarget::DrawRectangle(*self.x.borrow_mut(), *self.y.borrow_mut(),width,height,thickness,color));
    }

    pub fn fill_rectangle(&self,color: Color,width:u32,height:u32) {
        self.target.borrow_mut().push(DrawTarget::FillRectangle(*self.x.borrow_mut(), *self.y.borrow_mut(),width,height,color));
    }

    pub fn flush(&self,surface: &mut Surface) {
        surface.surface.draw(&mut *self.target.borrow_mut());
    }
}