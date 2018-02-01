extern crate graphics;
extern crate opengl_graphics;

use graphics::{Context, ellipse, Transformed};
use opengl_graphics::GlGraphics;

use color;
use geom;
use super::GameObject;

pub struct Bullet {
    pub pos: geom::Position,
    pub size: f64,
    pub ttl: f64,
}

const BULLET_SIZE: f64 = 5.0;
// Number of seconds til we can delete this bullet from the screen.
const BULLET_LIFETIME: f64 = 2.0;

impl Bullet {
    pub fn new(x: f64, y: f64) -> Bullet {
        return Bullet {
            pos: geom::Position::new(x, y),
            size: BULLET_SIZE,
            ttl: BULLET_LIFETIME
        };
    }

    pub fn radius(&self) -> f64 {
        return self.size / 2.0;
    }
}

impl GameObject for Bullet {
    fn position(&self) -> &geom::Position { &self.pos }
    fn radius(&self) -> f64 { BULLET_SIZE }

    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        let transform = ctxt.transform.trans(self.pos.x, self.pos.y);
        let radius = self.radius();
        ellipse(color::WHITE, [0.0, 0.0, radius, radius], transform, gl);
    }

    fn update(&mut self, dt: f64) {
        self.pos.x += 1.0;
        self.ttl -= dt;
    }
}