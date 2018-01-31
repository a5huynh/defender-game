extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use graphics::clear;
use piston::input::*;
use piston::window::Window;
use rand::Rng;

mod color;
pub mod config;

mod models;
use models::{GameObject};
use models::enemy::Enemy;
use models::player::Player;

const UNIT_MOVE: f64 = 10.0;

pub struct App {
    pub window: config::GraphicsConfig,
    player: Player,
    enemy: Enemy,
}

impl App {
    pub fn new(window: config::GraphicsConfig) -> App {
        let size = window.settings.size();

        let (x, y) = ((size.width / 2) as f64,
                      (size.height / 2) as f64);

        let player = Player::new(x, y, 20.0);

        // Choose a random spot in the window to render the enemy.
        let mut rng = rand::thread_rng();

        let enemy = Enemy::new(
            rng.gen_range(0.0, size.width as f64),
            rng.gen_range(0.0, size.height as f64),
            20.0
        );


        return App {
            window,
            player,
            enemy,
            bullets: Vec::new(),
        };
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
        let enemy = &self.enemy;
        let player = &self.player;

        // Render stuff.
        self.window.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(color::BLACK, gl);

            // Render objects
            enemy.render(&c, gl);
            player.render(&c, gl);
        });
    }

    // Update any animation, etc.
    // dt is the delta since the last update.
    pub fn update(&mut self, args: &UpdateArgs) {
        self.player.animate(args.dt);
    }
}