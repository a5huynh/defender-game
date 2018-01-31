extern crate graphics;
extern crate opengl_graphics;

use graphics::*;
use opengl_graphics::GlGraphics;

pub mod bullet;
pub mod enemy;
pub mod player;

// Every object that needs to be rendered on screen.
pub trait GameObject {
    fn render(&self, ctxt: &Context, gl: &mut GlGraphics);
    fn update(&mut self, _: f64) {
        // By default do nothing in the animation function
    }
}
