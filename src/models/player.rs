use graphics::{Context, rectangle, polygon, Transformed};
use opengl_graphics::GlGraphics;

use color;
use geom;
use super::GameObject;

const PLAYER_MOVE: f64 = 5.0;
const PLAYER_SIZE: f64 = 20.0;

pub struct Player {
    pub pos: geom::Position,
    pub dir: geom::Direction,
    pub size: f64,
    pub move_x: f64,
    pub move_y: f64,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Player {
        return Player {
            dir: geom::Direction::EAST,
            move_x: 0.0,
            move_y: 0.0,
            pos: geom::Position::new(x, y),
            size: PLAYER_SIZE,
        };
    }
}

impl GameObject for Player {
    fn position(&self) -> &geom::Position { &self.pos }
    fn radius(&self) -> f64 { self.size / 2.0 }

    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        // Render the player as a little square
        let shape = polygon::Polygon::new(color::RED);

        // Rotate the player to the direction they're facing
        let dir = match self.dir {
            geom::Direction::WEST => 0.0,
            geom::Direction::NORTH => 90.0,
            geom::Direction::EAST => 180.0,
            geom::Direction::SOUTH => 270.0,
        };

        let radius = self.radius();
        let transform = ctxt.transform
            .trans(self.pos.x, self.pos.y)
            .rot_deg(dir)
            .trans(-radius, -radius);

        let points = [
            [0.0, radius],
            [self.size, self.size],
            [self.size, 0.0]
        ];

        shape.draw(
            &points,
            &ctxt.draw_state,
            transform,
            gl
        );
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
        if self.move_x < 0.0 {
            self.dir = geom::Direction::WEST;
        } else if self.move_x > 0.0 {
            self.dir = geom::Direction::EAST;
        }

        self.pos.y += self.move_y;
        if self.move_y < 0.0 {
            self.dir = geom::Direction::NORTH;
        } else if self.move_y > 0.0 {
            self.dir = geom::Direction::SOUTH;
        }

    }
}