use crate::{Drawable, pixel, Point, Rgba};

pub struct Circle {
    center: Point,
    radius: i32,
    c: Rgba
}

impl Circle {
    pub fn new(center: Point, radius: i32, c: Rgba) -> Circle {
        Circle { center, radius, c }
    }
}

impl Drawable for Circle {
    fn draw(&self, frame: &mut [u8]) {
        let x = self.center.x;
        let y = self.center.y;
        let mut t1 = self.radius as f32 / 16.0;
        let mut x1 = self.radius as f32;
        let mut y1 = 0.0;

        while x1 >= y1 {
            pixel(Point::new(x1 as i32 + x, y1 as i32 + y), frame, &self.c);
            pixel(Point::new(-x1 as i32 + x, y1 as i32 + y), frame, &self.c);
            pixel(Point::new(x1 as i32 + x, -y1 as i32 + y), frame, &self.c);
            pixel(Point::new(-x1 as i32 + x, -y1 as i32 + y), frame, &self.c);
            pixel(Point::new(y1 as i32 + x, x1 as i32 + y), frame, &self.c);
            pixel(Point::new(-y1 as i32 + x, x1 as i32 + y), frame, &self.c);
            pixel(Point::new(y1 as i32 + x, -x1 as i32 + y), frame, &self.c);
            pixel(Point::new(-y1 as i32 + x, -x1 as i32 + y), frame, &self.c);
            y1 += 1.0;
            t1 += y1;
            let t2 = t1 - x1;

            if t2 >= 0.0 {
                t1 = t2;
                x1 -= 1.0;
            }
        }
    }
}