extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use opengl_graphics::{GlyphCache, TextureSettings};
use piston::input::*;

mod color;
mod geom;
mod gfx;
mod models;
pub mod config;

use crate::gfx::utils::{draw_center, draw_text};
use crate::models::{GameObject};
use crate::models::bullet::Bullet;
use crate::models::enemy::Enemy;
use crate::models::player::Player;

const FIRE_COOLDOWN: f64 = 0.1; // Only allow user to shoot 10 bullets/sec.

enum GameStatus {
    // Normal fighting mode
    Normal,
    // Player died
    Died,
    // Player won!
    Win,
}

struct GameState {
    debug_mode: bool,
    // Overall game state
    game_status: GameStatus,
    // User shooting state
    fire_bullets: bool,
    fire_cooldown: f64,
}

pub struct App<'a> {
    pub window: config::GraphicsConfig,
    glyph_cache: GlyphCache<'a>,
    player: Player,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    // Player score
    score: u32,
    // Game-wide events
    state: GameState,
}

impl<'a> App<'a> {
    pub fn new(window: config::GraphicsConfig) -> App<'a> {
        let size = window.size;

        let (x, y) = (f64::from(size.width / 2),
                      f64::from(size.height / 2));

        let player = Player::new(x, y);

        let state = GameState {
            debug_mode: false,
            fire_bullets: false,
            fire_cooldown: 0.0,
            game_status: GameStatus::Normal,
        };

        // Load font(s) used in the game.
        let glyph_cache = GlyphCache::new("./assets/fonts/PxPlus_IBM_VGA8.ttf", (), TextureSettings::new())
            .expect("Unable to load font");

        App {
            glyph_cache,
            player,
            state,
            window,
            bullets: Vec::new(),
            enemies: Vec::new(),
            score: 0,
        }
    }

    fn reset(&mut self) {
        self.state.game_status = GameStatus::Normal;
        self.score = 0;
        self.enemies.clear();
    }

    pub fn input(&mut self, button: &Button, is_press: bool) {
        match (&button, is_press) {
            (Button::Keyboard(key), true) => {
                match key {
                    Key::Up => self.player.start_move(geom::Direction::NORTH),
                    Key::Down => self.player.start_move(geom::Direction::SOUTH),
                    Key::Left => self.player.start_move(geom::Direction::WEST),
                    Key::Right => self.player.start_move(geom::Direction::EAST),
                    Key::Space => {
                        if self.state.fire_cooldown <= 0.0 {
                            self.state.fire_cooldown = FIRE_COOLDOWN;
                            self.state.fire_bullets = true;
                        }
                    }
                    // Toggle debug mode.
                    Key::D => {
                        self.state.debug_mode = !self.state.debug_mode;
                        println!("Debug mode: {}", self.state.debug_mode);
                    }
                    // Reset game
                    Key::Return => {
                        match self.state.game_status {
                            GameStatus::Died => self.reset(),
                            GameStatus::Win => self.reset(),
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
            (Button::Keyboard(key), false) => {
                match key {
                    Key::Up => self.player.stop_move(geom::Direction::NORTH),
                    Key::Down => self.player.stop_move(geom::Direction::SOUTH),
                    Key::Left => self.player.stop_move(geom::Direction::WEST),
                    Key::Right => self.player.stop_move(geom::Direction::EAST),
                    _ => (),
                }
            }
            _ => {}
        }
    }

    // Render stuff on the screen.
    pub fn render(&mut self, args: &RenderArgs) {
        // Grab list of objects to render.
        let bullets = &self.bullets;
        let enemies = &self.enemies;
        let player = &self.player;
        let gc = &mut self.glyph_cache;
        let state = &self.state;

        let debug_mode = self.state.debug_mode;
        let score = self.score;
        let size = self.window.size;

        // Render stuff.
        self.window.gl.draw(args.viewport(), |c, gl| {
            use graphics::*;

            // Clear the screen.
            clear(crate::color::BLACK, gl);

            // Check game status
            match state.game_status {
                GameStatus::Died => {
                    draw_center("YOU DIED!", 32, [f64::from(size.width), f64::from(size.height)], gc, &c, gl);
                    return;
                }
                GameStatus::Win => {
                    draw_center("YOU WIN!", 32, [f64::from(size.width), f64::from(size.height)], gc, &c, gl);
                    return;
                }
                _ => (),
            }

            // Render the current score
            let score_str = format!("Score: {}", score);
            draw_text(score_str.as_str(), [0.0, 16.0], 16, gc, &c, gl);

            // Render objects
            for bullet in bullets.iter() {
                bullet.render(&c, gl);
            }

            for enemy in enemies.iter() {
                enemy.render(&c, gl);
            }

            player.render(&c, gl);

            if debug_mode {
                player.render_dbg(&c, gl);
            }
        });
    }

    // Update any animation, etc.
    // dt is the delta since the last update.
    pub fn update(&mut self, args: UpdateArgs) {
        match self.state.game_status {
            GameStatus::Died => return,
            GameStatus::Win => return,
            _ => (),
        }

        let size = self.window.size;

        // Handle game events
        if self.state.fire_cooldown > 0.0 {
            self.state.fire_cooldown -= args.dt;
        }

        if self.state.fire_bullets {
            self.state.fire_bullets = false;
            self.bullets.push(
                Bullet::new(self.player.pos.x, self.player.pos.y, self.player.dir)
            );
        }

        for bullet in &mut self.bullets {
            bullet.update(args.dt, size);
            // Did bullet collide with any enemies
            for enemy in &mut self.enemies {
                if bullet.collides(enemy) {
                    // Destroy bullet
                    bullet.ttl = 0.0;
                    // Destroy enemy
                    enemy.health -= 1;
                    self.score += 20;
                }
            }
        }
        // Remove bullets that have outlived their TTL
        self.bullets.retain(|bullet| bullet.ttl > 0.0);
        self.enemies.retain(|enemy| enemy.health > 0);
        // Update player & enemies
        self.player.update(args.dt, size);
        // If number of enemies is zero... spawn more!
        if self.enemies.is_empty() {
            let size = self.window.size;
            for _ in 0..10 {
                self.enemies.push(Enemy::new_rand(f64::from(size.width), f64::from(size.height)));
            }
        }

        for enemy in &mut self.enemies {
            enemy.update(args.dt, size);
            // If the player collides with an enemy, game over!
            if enemy.collides(&self.player) {
                self.state.game_status = GameStatus::Died;
            }
        }
        // Did we kill all the enemies?
        if self.score == 100 {
            self.state.game_status = GameStatus::Win;
        }
    }
}