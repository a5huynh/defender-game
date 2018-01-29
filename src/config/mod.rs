use glutin_window::GlutinWindow;
use piston::window::WindowSettings;
use opengl_graphics::*;

pub struct GraphicsConfig {
    // OpenGL drawing backend
    pub gl: GlGraphics,
    // Window
    pub settings: GlutinWindow,
}

impl GraphicsConfig {
    pub fn new(title: &'static str, width: u32, height: u32) -> GraphicsConfig {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;
        // Setup a new window
        let settings = WindowSettings::new(title, [width, height])
            // Sets the OpenGL version
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        return GraphicsConfig {
            gl: GlGraphics::new(opengl),
            settings
        };
    }
}