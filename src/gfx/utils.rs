use crate::color;

use graphics::{Context, DrawState, Transformed};
use graphics::text::Text;
use opengl_graphics::{GlGraphics, GlyphCache};

pub fn draw_text(txt: &str, pos: [f64; 2], size: u32, gc: &mut GlyphCache, c: &Context, gl: &mut GlGraphics) {

    let transform = c.transform.trans(pos[0], pos[1]);

    Text::new_color(color::WHITE, size)
        .draw(txt, gc, &DrawState::default(), transform, gl)
        .unwrap();
}

// Draw text centered in the window
pub fn draw_center(txt: &str, size: u32, bounds: [f64; 2], gc: &mut GlyphCache, c: &Context, gl: &mut GlGraphics) {
    let half_size = f64::from(size) / 2.0;
    let num_chars = txt.len() as f64;

    let x = (bounds[0] / 2.0) - (num_chars * half_size) / 2.0;
    let y = (bounds[1] / 2.0) - half_size;

    draw_text(txt, [x, y], size, gc, c, gl);
}