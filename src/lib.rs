pub mod png;

use std::cell::RefCell;

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

pub struct Surface {
    pub(crate) surface: Box<dyn ISurface>
}

trait ISurface {
    fn draw(&self,target: &mut Vec<DrawTarget>);
}

enum DrawTarget {
    Clear(Color),
    DrawPoint(u32,u32,Color),
    DrawRectangle(u32,u32,u32,u32,u32,Color)
}

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

    pub fn draw_rectangle(&self,color: Color,x:u32,y:u32,width:u32,height:u32,thickness:u32) {
        self.target.borrow_mut().push(DrawTarget::DrawRectangle(*self.x.borrow_mut(), *self.y.borrow_mut(),width,height,thickness,color));
    }

    pub fn flush(&self,surface: Surface) {
        surface.surface.draw(&mut *self.target.borrow_mut());
    }
}