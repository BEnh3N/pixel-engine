use crate::{Drawable, pixel, Point, Rgba};

pub struct Line {
    p1: Point,
    p2: Point,
    c: Rgba
}

impl Line {
    pub fn new(p1: Point, p2: Point, c: Rgba) -> Line {
        Line {
            p1,
            p2,
            c
        }
    }

    pub fn from_points(p1: Point, p2: Point) -> Line {
        Line {
            p1,
            p2,
            c: Rgba::from_u8(0, 0, 0, 0xff)
        }
    }

    pub fn color(&mut self, c: Rgba) {
        self.c = c
    }
}

impl Drawable for Line {
    fn draw(&self, frame: &mut [u8]) {
        let mut x0 = self.p1.x;
        let mut y0 = self.p1.y;
        let x1 = self.p2.x;
        let y1 = self.p2.y;
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut error = dx + dy;

        loop {
            pixel(Point::new(x0, y0), frame, &self.c);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * error;
            if e2 >= dy {
                if x0 == x1 {
                    break;
                }
                error += dy;
                x0 += sx;
            }
            if e2 <= dx {
                if y0 == y1 {
                    break;
                }
                error += dx;
                y0 += sy;
            }
        }
    }
}