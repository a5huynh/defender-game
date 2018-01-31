extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use graphics::{clear};
use piston::input::*;
use piston::window::Window;

pub mod config;
use config::GraphicsConfig;

mod models;
use models::{GameObject};
use models::player::Player;

const UNIT_MOVE: f64 = 10.0;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
// const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
// const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

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
        // Grab list of objects to render.
        let player = &self.player;
        // Render stuff.
        self.window.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            // Place object on screen
            player.render(&c, gl);
        });
    }

    // Update any animation, etc.
    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.player.rotation += 2.0 * args.dt;
    }
}