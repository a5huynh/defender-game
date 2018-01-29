extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use graphics::{clear, Transformed, rectangle};
use piston::input::*;
use piston::window::Window;

pub mod config;
use config::GraphicsConfig;

mod models;
use models::Player;

const UNIT_MOVE: f64 = 10.0;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub struct App {
    pub window: GraphicsConfig,
    player: Player,
}

impl App {
    pub fn new(window: GraphicsConfig) -> App {
        let size = window.settings.size();

        let (x, y) = ((size.width / 2) as f64,
                      (size.height / 2) as f64);

        let player = Player::new(x, y, 20.0);

        return App { window, player };
    }

    pub fn input(&mut self, button: &Button) {
        if let Button::Keyboard(key) = *button {
            match key {
                Key::Up => self.player.y -= UNIT_MOVE,
                Key::Down => self.player.y += UNIT_MOVE,
                Key::Left => self.player.x -= UNIT_MOVE,
                Key::Right => self.player.x += UNIT_MOVE,
                Key::Space => (),
                _ => (),
            }
        }
    }

    // Render stuff on the screen.
    pub fn render(&mut self, args: &RenderArgs) {

        let player = &self.player;

        let square = rectangle::square(0.0, 0.0, player.size);

        let rotation = self.player.rotation;

        let x = player.x;
        let y = player.y;

        self.window.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = c.transform.trans(x, y)
                                .rot_rad(rotation)
                                .trans(-(player.size / 2.0), -(player.size / 2.0));
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
        self.player.rotation += 2.0 * args.dt;
    }
}