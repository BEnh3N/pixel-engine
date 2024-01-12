use image::{DynamicImage, GenericImageView};
use image::io::Reader;
use crate::{Drawable, pixel, Point, Rgba};

pub struct Sprite {
    img: DynamicImage,
    point: Point,
}

impl Sprite {
    pub fn new(filename: &str, point: Point) -> Sprite {
        let img = Reader::open(filename).unwrap().decode().unwrap();
        Sprite { img, point }
    }
}

impl Drawable for Sprite {
    fn draw(&self, frame: &mut [u8]) {
        for p in self.img.pixels() {
            if p.2.0[3] == 0x00 {
                continue;
            }
            let c = Rgba::from_pixel(p.2.0);
            pixel(
                Point::new(p.0 as i32 + self.point.x, -(p.1 as i32) + self.point.y + self.img.height() as i32),
                frame,
                &c
            );
        }
    }
}