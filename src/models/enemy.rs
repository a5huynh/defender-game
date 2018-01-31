extern crate graphics;
extern crate opengl_graphics;

use graphics::{Context, rectangle, Transformed};
use opengl_graphics::GlGraphics;

use color;
use super::GameObject;

pub struct Enemy {
    pub x: f64,
    pub y: f64,
    pub size: f64,
}

impl Enemy {
    pub fn new(x: f64, y: f64, size: f64) -> Enemy {
        return Enemy { x, y, size };
    }

    pub fn radius(&self) -> f64 {
        return self.size / 2.0;
    }
}

impl GameObject for Enemy {
    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        // Render the player as a little square
        let square = rectangle::square(0.0, 0.0, self.size);
        let radius = self.radius();
        let transform = ctxt.transform.trans(self.x, self.y)
            .trans(-radius, -radius);

        rectangle(color::GREEN, square, transform, gl);
    }
}