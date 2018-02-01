extern crate graphics;
extern crate opengl_graphics;
extern crate rand;

use graphics::{Context, rectangle, Transformed};
use opengl_graphics::GlGraphics;
use rand::Rng;

use color;
use geom;
use super::GameObject;

// The max movement of the enemy in a rando direction.
const MOVE_RADIUS: f64 = 5.0;
const MOVE_TTL: f64 = 0.1; // 100 millisecond

pub struct Enemy {
    pub pos: geom::Position,
    pub size: f64,
    move_ttl: f64,
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
    fn position(&self) -> &geom::Position { &self.pos }
    fn radius(&self) -> f64 { self.size / 2.0 }

    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        // Render the player as a little square
        let square = rectangle::square(0.0, 0.0, self.size);
        let radius = self.radius();
        let transform = ctxt.transform.trans(self.pos.x, self.pos.y)
            .trans(-radius, -radius);

        rectangle(color::GREEN, square, transform, gl);
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
        // Only move every <MOVE_TTL> seconds.
        self.move_ttl -= dt;
        if self.move_ttl <= 0.0 {
            // Randomly move in a random direction.
            let mut rng = rand::thread_rng();
            self.pos.x += rng.gen_range(0.0, MOVE_RADIUS * 2.0) - MOVE_RADIUS;
            self.pos.y += rng.gen_range(0.0, MOVE_RADIUS * 2.0) - MOVE_RADIUS;
            self.move_ttl = MOVE_TTL;
        }
    }
}