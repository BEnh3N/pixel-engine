use error_iter::ErrorIter as _;
use image::io::Reader;
use image::GenericImageView;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: i32 = 111;
const HEIGHT: i32 = 101;
const BOX_SIZE: i32 = 4;

/// Representation of the application state.
struct World {
    mouse: Point,
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH * BOX_SIZE, HEIGHT * BOX_SIZE);
        WindowBuilder::new()
            .with_title("Pixel Engine")
            .with_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };
    let mut world = World::new();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(mouse) = input.mouse() {
                world.mouse = Point::new(
                    (mouse.0 / BOX_SIZE as f32 / 2.0) as i32,
                    (HEIGHT as f32 - (mouse.1 / BOX_SIZE as f32 / 2.0)) as i32,
                );
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        let mouse = Point::new(0, 0);
        Self { mouse }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {}

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        clear_background(frame, &[0x00, 0x00, 0x55, 0xff]);

        draw_line(
            Point::new(WIDTH / 2, HEIGHT / 2),
            Point::new(self.mouse.x, self.mouse.y),
            frame,
            &[255, 255, 255, 255],
        );

        draw_circle(
            WIDTH / 2,
            HEIGHT / 2,
            (((self.mouse.x - (WIDTH / 2)).pow(2)
                + (self.mouse.y - (HEIGHT / 2)).pow(2)) as f32
        ).sqrt() as i32,
            frame,
            &[255, 255, 255, 255],
        );

        let pts = vec![
            Point::new(10, 10),
            Point::new(100, 10),
            Point::new(self.mouse.x + 90, self.mouse.y),
            Point::new(self.mouse.x, self.mouse.y),
        ];
        draw_polygon(&pts, frame, &[255, 0, 0, 255]);

        draw_sprite("bird.png", self.mouse.x, self.mouse.y, frame);
    }
}

fn get_pixel_index(x: i32, y: i32) -> usize {
    (((y * WIDTH) + x) * 4) as usize
}

fn pixel(pt: Point, frame: &mut [u8], c: &[u8; 4]) {
    let x = pt.x;
    let y = pt.y;
    if (x < 0) || (y < 0) || (x >= WIDTH) || (y >= HEIGHT) {
        return;
    }

    let y = HEIGHT as i32 - y - 1;

    let i = get_pixel_index(x, y);
    let pixel = &mut frame[i..i + 4];
    pixel.copy_from_slice(c);
}

fn clear_background(frame: &mut [u8], c: &[u8; 4]) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            pixel(Point::new(x, y), frame, c);
        }
    }
}

fn draw_line(pt1: Point, pt2: Point, frame: &mut [u8], c: &[u8; 4]) {
    let mut x0 = pt1.x;
    let mut y0 = pt1.y;
    let x1 = pt2.x;
    let y1 = pt2.y;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;

    loop {
        pixel(Point::new(x0, y0), frame, c);
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

fn draw_circle(x: i32, y: i32, r: i32, frame: &mut [u8], c: &[u8; 4]) {
    let mut t1 = r as f32 / 16.0;
    let mut x1 = r as f32;
    let mut y1 = 0.0;

    while x1 >= y1 {
        pixel(Point::new(x1 as i32 + x, y1 as i32 + y), frame, c);
        pixel(Point::new(-x1 as i32 + x, y1 as i32 + y), frame, c);
        pixel(Point::new(x1 as i32 + x, -y1 as i32 + y), frame, c);
        pixel(Point::new(-x1 as i32 + x, -y1 as i32 + y), frame, c);
        pixel(Point::new(y1 as i32 + x, x1 as i32 + y), frame, c);
        pixel(Point::new(-y1 as i32 + x, x1 as i32 + y), frame, c);
        pixel(Point::new(y1 as i32 + x, -x1 as i32 + y), frame, c);
        pixel(Point::new(-y1 as i32 + x, -x1 as i32 + y), frame, c);
        y1 += 1.0;
        t1 += y1;
        let t2 = t1 - x1;

        if t2 >= 0.0 {
            t1 = t2;
            x1 -= 1.0;
        }
    }
}

fn draw_polygon(pts: &Vec<Point>, frame: &mut [u8], c: &[u8; 4]) {
    for p in pts.windows(2) {
        let pt1 = p[0];
        let pt2 = p[1];
        draw_line(Point::new(pt1.x, pt1.y), Point::new(pt2.x, pt2.y), frame, c);
    }
    draw_line(pts[pts.len() - 1], pts[0], frame, c)
}

fn draw_sprite(filename: &str, x: i32, y: i32, frame: &mut [u8]) {
    let img = Reader::open(filename).unwrap().decode().unwrap();
    for p in img.pixels() {
        if p.2 .0[3] == 0x00 {
            continue;
        }
        pixel(
            Point::new(p.0 as i32 + x, -(p.1 as i32) + y + img.height() as i32),
            frame,
            &p.2 .0,
        );
    }
}
