pub const WIDTH: i32 = 111;
pub const HEIGHT: i32 = 101;
pub const BOX_SIZE: i32 = 4;

pub mod draw;

pub fn clear_background(frame: &mut [u8], c: Rgba) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            pixel(Point::new(x, y), frame, &c);
        }
    }
}

pub fn draw_to_frame<D>(frame: &mut [u8], shape: D)
where
    D: Drawable
{
    shape.draw(frame);
}

pub trait Drawable {
    fn draw(&self, frame: &mut [u8]);
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Rgba {
    pub fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba { r, g, b, a }
    }

    pub fn from_f32(r: f32, g: f32, b: f32, a: f32) -> Rgba {
        let r = (r * 255.99) as u8;
        let g = (g * 255.99) as u8;
        let b = (b * 255.99) as u8;
        let a = (a * 255.99) as u8;
        Rgba { r, g, b, a }
    }

    pub fn from_pixel(pixel: [u8; 4]) -> Rgba {
        Rgba { r: pixel[0], g: pixel[1], b: pixel[2], a: pixel[3] }
    }

    pub fn to_pixel(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

fn get_pixel_index(x: i32, y: i32) -> usize {
    (((y * WIDTH) + x) * 4) as usize
}

fn pixel(pt: Point, frame: &mut [u8], c: &Rgba) {
    let x = pt.x;
    let y = pt.y;
    if (x < 0) || (y < 0) || (x >= WIDTH) || (y >= HEIGHT) {
        return;
    }

    let y = HEIGHT - y - 1;

    let i = get_pixel_index(x, y);
    let pixel = &mut frame[i..i + 4];
    pixel.copy_from_slice(&c.to_pixel());
}