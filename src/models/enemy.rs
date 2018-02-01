extern crate graphics;
extern crate opengl_graphics;

use graphics::{Context, rectangle, Transformed};
use opengl_graphics::GlGraphics;

use color;
use geom;
use super::GameObject;

pub struct Enemy {
    pub pos: geom::Position,
    pub size: f64,
}

impl Enemy {
    pub fn new(x: f64, y: f64, size: f64) -> Enemy {
        return Enemy {
            size,
            pos: geom::Position::new(x, y),
            move_ttl: MOVE_TTL
        };
    }
}

impl GameObject for Enemy {
    fn radius(&self) -> f64 { self.size / 2.0 }

    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        // Render the player as a little square
        let square = rectangle::square(0.0, 0.0, self.size);
        let radius = self.radius();
        let transform = ctxt.transform.trans(self.pos.x, self.pos.y)
            .trans(-radius, -radius);

        rectangle(color::GREEN, square, transform, gl);
    }
}