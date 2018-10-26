use graphics::{Context, rectangle, polygon, Transformed};
use opengl_graphics::GlGraphics;

use piston::window::Size;
use crate::color;
use crate::geom;
use crate::geom::Direction;

use super::GameObject;

const PLAYER_SPEED: f64 = 2.0;
const PLAYER_SIZE: f64 = 20.0;
// Drift for this long after movement key is released.
// You don't came to a hard stop in space!
const PLAYER_DRIFT: f64 = 0.2;

pub struct Player {
    pub pos: geom::Position,
    pub dir: Direction,
    pub size: f64,
    pub drift_ttl: f64,
    move_offset: geom::Position,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Player {
        Player {
            dir: Direction::EAST,
            drift_ttl: 0.0,
            move_offset: geom::Position::new(0.0, 0.0),
            pos: geom::Position::new(x, y),
            size: PLAYER_SIZE,
        }
    }

    pub fn start_move(&mut self, dir: Direction) {
        self.dir = dir;
        match dir {
            Direction::WEST => self.move_offset.x = -PLAYER_SPEED,
            Direction::NORTH => self.move_offset.y = -PLAYER_SPEED,
            Direction::EAST => self.move_offset.x = PLAYER_SPEED,
            Direction::SOUTH => self.move_offset.y = PLAYER_SPEED,
        }
    }

    pub fn stop_move(&mut self, dir: Direction) {
        self.drift_ttl = PLAYER_DRIFT;
        match dir {
            Direction::WEST => self.move_offset.x = 0.0,
            Direction::NORTH => self.move_offset.y = 0.0,
            Direction::EAST => self.move_offset.x = 0.0,
            Direction::SOUTH => self.move_offset.y = 0.0,
        }
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
            Direction::WEST => 0.0,
            Direction::NORTH => 90.0,
            Direction::EAST => 180.0,
            Direction::SOUTH => 270.0,
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

    fn update(&mut self, dt: f64, size: Size) {
        // TODO: Prevent movement outside of boundaries.
        let radius = self.radius();

        self.pos.x += self.move_offset.x;
        self.pos.y += self.move_offset.y;

        if self.drift_ttl > 0.0 {
            self.drift_ttl -= dt;
            let drift_speed = PLAYER_SPEED / 2.0;
            match self.dir {
                Direction::NORTH => self.pos.y -= drift_speed,
                Direction::EAST => self.pos.x += drift_speed,
                Direction::SOUTH => self.pos.y += drift_speed,
                Direction::WEST => self.pos.x -= drift_speed,
            }
        }

        geom::restrict_to_bounds(
            &mut self.pos,
            [radius, radius, f64::from(size.width), f64::from(size.height)]
        );

    }
}