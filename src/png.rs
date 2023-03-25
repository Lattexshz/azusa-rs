use std::fs::File;
use std::io::BufWriter;
use crate::{DrawTarget, ISurface, Surface};

pub struct PngSurface {
    file: String,
    width: u32,
    height: u32
}

impl PngSurface {
    pub fn new(file:&str,width:u32,height:u32) -> Surface {
        Surface {
            surface: Box::new(Self {
                file:file.to_string(),
                width,
                height,
            }),
        }
    }
}

impl ISurface for PngSurface {
    fn draw(&self, target: &mut Vec<DrawTarget>) {
        let file = File::create(&self.file).unwrap();
        let w = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width,self.height); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();

        let mut data:Vec<u8> = vec![];

        for i in target {
            match i {
                DrawTarget::Clear(color) => {
                    let (r,g,b,a) = (*color).into();
                    for i in 0..(self.width*self.height) {
                        data.push(r);
                        data.push(g);
                        data.push(b);
                        data.push(a);
                    }
                }
                DrawTarget::DrawPoint(x,y,color) => {
                    let (r,g,b,a) = (*color).into();
                    let (x,y) = (*x,*y);

                    let index = ((x+y*self.width)*4) as usize;
                    data[index] = r;
                    data[index + 1] = g;
                    data[index + 2] = b;
                    data[index + 3] = a;
                }
            }
        }

        writer.write_image_data(&data).unwrap();

    }
}