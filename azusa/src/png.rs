use crate::{DrawTarget, ISurface, Surface};
use immo::png::Png;
use std::fs::File;
use std::io::BufWriter;

#[repr(C)]
pub struct PngSurface {
    file: String,
    width: u32,
    height: u32,
}

impl PngSurface {
    pub fn new(file: &str, width: u32, height: u32) -> Surface {
        Surface {
            surface: Box::new(Self {
                file: file.to_string(),
                width,
                height,
            }),
        }
    }
}

impl ISurface for PngSurface {
    fn draw(&mut self, target: &mut Vec<DrawTarget>) {
        let file = File::create(&self.file).unwrap();
        let w = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();

        let mut png = Png::new(self.width, self.height);

        for i in target {
            match i {
                DrawTarget::Clear(color) => {
                    png.clear((*color).into());
                }
                DrawTarget::DrawPoint(x, y, color) => {
                    png.point(*x, *y, (*color).into()).unwrap();
                }

                DrawTarget::DrawRectangle(x, y, width, height, thickness, color) => {
                    png.draw_rectangle(*x, *y, *width, *height, *thickness, (*color).into())
                        .unwrap();
                }
                DrawTarget::FillRectangle(x, y, width, height, color) => {
                    png.fill_rectangle(*x, *y, *width, *height, (*color).into())
                        .unwrap();
                }
            }
        }

        writer.write_image_data(png.as_slice()).unwrap();
    }
}
