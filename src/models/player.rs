extern crate graphics;
extern crate opengl_graphics;

use graphics::{Context, rectangle, Transformed};
use opengl_graphics::GlGraphics;

use color;
use geom;
use super::GameObject;

pub struct Player {
    pub pos: geom::Position,
    pub size: f64,
    pub move_x: f64,
    pub move_y: f64,
}

impl Player {
    pub fn new(x: f64, y: f64, size: f64) -> Player {
        return Player {
            pos: geom::Position::new(x, y),
            size,
            move_x: 0.0,
            move_y: 0.0,
        };
    }
}

impl GameObject for Player {
    fn radius(&self) -> f64 { self.size / 2.0 }

    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        // Render the player as a little square
        let square = rectangle::square(0.0, 0.0, self.size);
        let radius = self.radius();

        let transform = ctxt.transform
            .trans(self.pos.x, self.pos.y)
            .trans(-radius, -radius);

        rectangle(color::RED, square, transform, gl);
    }

    fn render_dbg(&self, ctxt: &Context, gl: &mut GlGraphics) {
        // Render collison box
        let radius = self.radius();
        let diam = radius * 2.0;

        let circle = rectangle::Rectangle::new_round_border(color::WHITE, radius, 1.0);
        // Center on x/y
        let transform = ctxt.transform
            .trans(self.pos.x, self.pos.y)
            .trans(-radius, -radius);

        circle.draw([0.0, 0.0, diam, diam], &ctxt.draw_state, transform, gl);
    }

    fn update(&mut self, dt: f64) {
        // TODO: Prevent movement outside of boundaries.
        self.pos.x += self.move_x;
        self.pos.y += self.move_y;
    }
}