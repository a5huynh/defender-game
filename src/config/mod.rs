use glutin_window::GlutinWindow as Window;
use piston::window::{ Size, WindowSettings };
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct GraphicsConfig {
    // OpenGL drawing backend
    pub gl: GlGraphics,
    // Window
    pub settings: Window,
    // Window size
    pub size: Size,
}

impl GraphicsConfig {
    pub fn new(title: &'static str, width: u32, height: u32) -> GraphicsConfig {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_3;
        // Setup a new window
        let size = Size { width, height };
        let settings: Window = WindowSettings::new(title, [width, height])
            // Sets the OpenGL version
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        GraphicsConfig {
            gl: GlGraphics::new(opengl),
            settings,
            size
        }
    }
}