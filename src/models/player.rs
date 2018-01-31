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
    pub move_x: f64,
    pub move_y: f64,
}

impl Player {
    pub fn new(x: f64, y: f64, size: f64) -> Player {
        return Player {
            x, y, size,
            rotation: 0.0,
            move_x: 0.0,
            move_y: 0.0,
        };
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

    fn render_dbg(&self, ctxt: &Context, gl: &mut GlGraphics) {
        // Render collison box
        let radius = self.radius();
        let transform = ctxt.transform.trans(self.x, self.y);
        graphics::circle_arc(
            color::WHITE,
            radius,
            0.0,
            2.0 * 3.14 * radius,
            [0.0, 0.0, radius, radius],
            transform,
            gl
        )
    }

    fn update(&mut self, dt: f64) {
        // TODO: Prevent movement outside of boundaries.
        self.x += self.move_x;
        self.y += self.move_y;
    }
}