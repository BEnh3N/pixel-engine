use crate::{Drawable, Point, Rgba};
use crate::draw::line::Line;

pub struct Polygon {
    points: Vec<Point>,
    c: Rgba
}

impl Polygon {
    pub fn new(points: Vec<Point>, c: Rgba) -> Polygon {
        Polygon { points, c }
    }
}

impl Drawable for Polygon {
    fn draw(&self, frame: &mut [u8]) {
        for p in self.points.windows(2) {
            let pt1 = p[0];
            let pt2 = p[1];
            let line = Line::new(Point::new(pt1.x, pt1.y), Point::new(pt2.x, pt2.y), self.c);
            line.draw(frame);
        }
        let line = Line::new(self.points[self.points.len() - 1], self.points[0], self.c);
        line.draw(frame);
    }
}
