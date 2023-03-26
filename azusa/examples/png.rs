use azusa::{Azusa, Color};
use azusa::png::PngSurface;

fn main() {
    let mut surface = PngSurface::new("png-sample.png",16,16);
    let azusa = Azusa::new();
    azusa.clear(Color::Rgba(255,255,255,255));

    azusa.draw_rectangle(Color::Rgba(0,0,0,255),15,15,1);
    azusa.flush(&mut surface);
}