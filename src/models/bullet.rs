use graphics::{Context, ellipse, Transformed};
use opengl_graphics::GlGraphics;

use piston::window::Size;
use crate::color;
use crate::geom;
use super::GameObject;

pub struct Bullet {
    pub pos: geom::Position,
    pub dir: geom::Direction,
    pub size: f64,
    pub ttl: f64,
}

const BULLET_SPEED: f64 = 2.0;
const BULLET_SIZE: f64 = 5.0;
// Number of seconds til we can delete this bullet from the screen (if it hasn't
// collided with an enemy yet).
const BULLET_LIFETIME: f64 = 2.0;

impl Bullet {
    pub fn new(x: f64, y: f64, dir: geom::Direction) -> Bullet {
        Bullet {
            dir,
            pos: geom::Position::new(x, y),
            size: BULLET_SIZE,
            ttl: BULLET_LIFETIME
        }
    }

    pub fn radius(&self) -> f64 {
        self.size / 2.0
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

    fn update(&mut self, dt: f64, _: Size) {
        self.ttl -= dt;
        // Move the bullet in the direction the player was facing.
        match self.dir {
            geom::Direction::EAST => self.pos.x += BULLET_SPEED,
            geom::Direction::NORTH => self.pos.y -= BULLET_SPEED,
            geom::Direction::WEST => self.pos.x -= BULLET_SPEED,
            geom::Direction::SOUTH => self.pos.y += BULLET_SPEED,
        }
    }
}