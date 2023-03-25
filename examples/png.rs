use azusa::{Azusa, Color};
use azusa::png::PngSurface;

fn main() {
    let surface = PngSurface::new("png-sample.png",16,16);
    let azusa = Azusa::new();
    azusa.clear(Color::Rgba(255,255,255,255));
    azusa.move_to(15,0);
    azusa.draw_point(Color::Rgba(0,0,0,255));
    azusa.flush(surface);
}