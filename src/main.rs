use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use pixel_engine::{BOX_SIZE, clear_background, draw_to_frame, HEIGHT, Point, Rgba, WIDTH};
use pixel_engine::draw::circle::Circle;
use pixel_engine::draw::line::Line;
use pixel_engine::draw::polygon::Polygon;
use pixel_engine::draw::sprite::Sprite;

struct World {
    mouse: Point,
}

fn main() -> Result<(), Error> {
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
                eprintln!("{}", err);
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

impl World {
    fn new() -> Self {
        let mouse = Point::new(0, 0);
        Self { mouse }
    }

    fn update(&mut self) {}

    fn draw(&self, frame: &mut [u8]) {
        clear_background(frame, Rgba::from_u8(0x00, 0x00, 0x55, 0xff));

        let line = Line::new(
            Point::new(WIDTH / 2, HEIGHT / 2),
            Point::new(self.mouse.x, self.mouse.y),
            Rgba::from_u8(0xff, 0xff, 0xff, 0xff)
        );
        draw_to_frame(frame, line);

        let circle = Circle::new(
            Point::new(WIDTH / 2, HEIGHT / 2),
            (((self.mouse.x - (WIDTH / 2)).pow(2) + (self.mouse.y - (HEIGHT / 2)).pow(2)) as f32).sqrt() as i32,
            Rgba::from_u8(0xff, 0xff, 0xff, 0xff)
        );
        draw_to_frame(frame, circle);

        let pts = vec![
            Point::new(10, 10),
            Point::new(100, 10),
            Point::new(self.mouse.x + 90, self.mouse.y),
            Point::new(self.mouse.x, self.mouse.y),
        ];
        let polygon = Polygon::new(pts, Rgba::from_u8(0xff, 0, 0, 0xff));
        draw_to_frame(frame, polygon);

        let sprite = Sprite::new("bird.png", Point::new(self.mouse.x, self.mouse.y));
        draw_to_frame(frame, sprite);
    }
}

