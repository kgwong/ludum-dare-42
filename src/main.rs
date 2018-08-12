extern crate ggez;
extern crate nphysics2d;
extern crate ncollide2d;

mod hitbox;
mod tile;
mod player;
mod projectile;
mod main_state;
mod tilesheet;

use main_state::MainState;

use ggez::conf;
use ggez::Context;
use ggez::event;

static GAME_TITLE: &'static str = "Ludum Dare";
const WINDOW_HEIGHT : u32 = 16 * 34;
const WINDOW_WIDTH : u32 = 8 * 34;

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_setup.title = GAME_TITLE.to_string();
    c.window_mode.width = WINDOW_WIDTH;
    c.window_mode.height = WINDOW_HEIGHT;
    let ctx = &mut Context::load_from_conf("ludum_dare", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}