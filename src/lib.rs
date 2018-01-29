extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::ops::*;
use graphics::{clear, Transformed, rectangle};
use piston::input::*;
use piston::window::Window;

pub mod config;
use config::GraphicsConfig;

const UNIT_MOVE: f64 = 10.0;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub struct App {
    pub window: GraphicsConfig,
    // Rotation for the square.
    pub rotation: f64,
    pub x: f64,
    pub y: f64,
}

impl App {
    pub fn new(window: GraphicsConfig) -> App {
        let size = window.settings.size();

        let (x, y) = ((size.width / 2) as f64,
                      (size.height / 2) as f64);

        return App { window, x, y, rotation: 0.0 };
    }

    pub fn input(&mut self, button: &Button) {
        if let Button::Keyboard(key) = *button {
            match key {
                Key::Up => self.y -= UNIT_MOVE,
                Key::Down => self.y += UNIT_MOVE,
                Key::Left => self.x -= UNIT_MOVE,
                Key::Right => self.x += UNIT_MOVE,
                Key::Space => (),
                _ => (),
            }
        }
    }

    // Render stuff on the screen.
    pub fn render(&mut self, args: &RenderArgs) {

        let square = rectangle::square(0.0, 0.0, 50.0);

        let rotation = self.rotation;

        let x = self.x;
        let y = self.y;

        self.window.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = c.transform.trans(x, y)
                                .rot_rad(rotation)
                                .trans(-25.0, -25.0);
            // let transform = c.transform.trans(x, y)
            //                            .rot_rad(rotation)
            //                            .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    // Update any animation, etc.
    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}