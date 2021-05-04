use graphics::*;
use opengl_graphics::GlGraphics;

use piston::window::Size;
use crate::geom::Position;

pub mod bullet;
pub mod enemy;
pub mod player;

// Every object that needs to be rendered on screen.
pub trait GameObject {
    // Used to determine whether one object has collided with another
    // object.
    fn collides(&self, other: &dyn GameObject) -> bool {
        // Two circles intersect if the distance between their centers is
        // between the sum and the difference of their radii.
        // TODO: Bounding boxes might be more efficient.
        let x2 = self.position().x - other.position().x;
        let y2 = self.position().y - other.position().y;
        let sum = x2.powf(2.0) + y2.powf(2.0);

        let r_start = self.radius() - other.radius();
        let r_end = self.radius() + other.radius();

        r_start.powf(2.0) <= sum && sum <= r_end.powf(2.0)
    }

    // Use to determine position of the object
    fn position(&self) -> &Position;
    fn radius(&self) -> f64;

    // Main draw function for this GameObject.
    fn render(&self, ctxt: &Context, gl: &mut GlGraphics);
    // Only call if debug mode is turned on.
    fn render_dbg(&self, _: &Context, _: &mut GlGraphics) {}
    // Handle updates to movement/animation/etc.
    fn update(&mut self, _: f64, _: Size) {}
}
