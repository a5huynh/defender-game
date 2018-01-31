extern crate graphics;
extern crate opengl_graphics;

use graphics::{Context, rectangle, Transformed};
use opengl_graphics::GlGraphics;

use color;
use super::GameObject;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
    pub size: f64,
}

impl Player {
    pub fn new(x: f64, y: f64, size: f64) -> Player {
        return Player { x, y, size, rotation: 0.0 };
    }

    pub fn animate(&mut self, dt: f64) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * dt;
    }

    pub fn radius(&self) -> f64 {
        return self.size / 2.0;
    }
}

impl GameObject for Player {
    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        // Render the player as a little square
        let square = rectangle::square(0.0, 0.0, self.size);
        let radius = self.radius();
        let transform = ctxt.transform.trans(self.x, self.y)
            .rot_rad(self.rotation)
            .trans(-radius, -radius);

        rectangle(color::RED, square, transform, gl);
    }
}