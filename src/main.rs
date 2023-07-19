use error_iter::ErrorIter as _;
use image::GenericImageView;
use image::io::Reader;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: i32 = 320;
const HEIGHT: i32 = 240;
const BOX_SIZE: i32 = 2;

/// Representation of the application state.
struct World {
    mouse: (f32, f32),
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
                world.mouse = (
                    (mouse.0 / BOX_SIZE as f32 / 2.0),
                    HEIGHT as f32 - (mouse.1 / BOX_SIZE as f32 / 2.0),
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
        Self { mouse: (0.0, 0.0) }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {}

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        clear_background(frame, &[0x00, 0x00, 0x55, 0xff]);

        draw_line(
            WIDTH / 2,
            HEIGHT / 2,
            self.mouse.0 as i32,
            self.mouse.1 as i32,
            frame,
            &[255, 255, 255, 255],
        );

        draw_circle(
            WIDTH / 2,
            HEIGHT / 2,
            ((self.mouse.0 - (WIDTH/2) as f32).powi(2) + (self.mouse.1 - (HEIGHT/2) as f32).powi(2)).sqrt() as i32,
            frame,
            &[255, 255, 255, 255],
        );

        draw_rectangle(
            10,
            10,
            100,
            10,
            self.mouse.0 as i32 + 90,
            self.mouse.1 as i32,
            self.mouse.0 as i32,
            self.mouse.1 as i32,
            frame,
            &[255, 0, 0, 255],
        );

        draw_sprite("block.png", self.mouse.0 as i32, self.mouse.1 as i32, frame);
    }
}

fn get_pixel_index(x: i32, y: i32) -> usize {
    (((y * WIDTH) + x) * 4) as usize
}

fn pixel(x: i32, y: i32, frame: &mut [u8], c: &[u8; 4]) {
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
            pixel(x, y, frame, c);
        }
    }
}

fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, frame: &mut [u8], c: &[u8; 4]) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;

    loop {
        pixel(x0, y0, frame, c);
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
        pixel(x1 as i32 + x, y1 as i32 + y, frame, c);
        pixel(-x1 as i32 + x, y1 as i32 + y, frame, c);
        pixel(x1 as i32 + x, -y1 as i32 + y, frame, c);
        pixel(-x1 as i32 + x, -y1 as i32 + y, frame, c);
        pixel(y1 as i32 + x, x1 as i32 + y, frame, c);
        pixel(-y1 as i32 + x, x1 as i32 + y, frame, c);
        pixel(y1 as i32 + x, -x1 as i32 + y, frame, c);
        pixel(-y1 as i32 + x, -x1 as i32 + y, frame, c);
        y1 += 1.0;
        t1 += y1;
        let t2 = t1 - x1;

        if t2 >= 0.0 {
            t1 = t2;
            x1 -= 1.0;
        }
    }
}

fn draw_rectangle(
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
    frame: &mut [u8],
    c: &[u8; 4],
) {
    draw_line(x0, y0, x1, y1, frame, c);
    draw_line(x1, y1, x2, y2, frame, c);
    draw_line(x2, y2, x3, y3, frame, c);
    draw_line(x3, y3, x0, y0, frame, c);
}

fn draw_sprite(filename: &str, x: i32, y: i32, frame: &mut [u8]) {
    let img = Reader::open(filename).unwrap().decode().unwrap();
    for p in img.pixels() {
        if p.2.0[3] == 0x00 { continue; }
        pixel(p.0 as i32 + x, -(p.1 as i32) + y + img.height() as i32, frame, &p.2.0);
    }
}
