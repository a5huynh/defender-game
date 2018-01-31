extern crate graphics;
extern crate opengl_graphics;

use graphics::*;
use opengl_graphics::GlGraphics;

pub mod bullet;
pub mod enemy;
pub mod player;

// Every object that needs to be rendered on screen.
pub trait GameObject {
    // Main draw function for this GameObject.
    fn render(&self, ctxt: &Context, gl: &mut GlGraphics);
    // Only call if debug mode is turned on.
    fn render_dbg(&self, _: &Context, _: &mut GlGraphics) {}
    // Handle updates to movement/animation/etc.
    fn update(&mut self, _: f64) {}
}
