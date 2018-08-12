extern crate ggez;
extern crate nphysics2d;
extern crate ncollide2d;

mod tilesheet;
mod tile;
mod hitbox;
mod player;
mod projectile;
mod main_state;
mod anim;

use main_state::*;
use tile::*;

use ggez::conf;
use ggez::Context;
use ggez::event;

static GAME_TITLE: &'static str = "Ludum Dare";
pub const WINDOW_WIDTH : u32 = ( NUM_TILES_X * TILE_SPACE ) as u32;
pub const WINDOW_HEIGHT : u32 = ( NUM_TILES_Y * TILE_SPACE ) as u32;

pub fn main() {
    println!( "Welcome to <>");
    println!( "Try to hit your opponent with floor tiles and avoid falling off!");
    println!( "P1: WASD, Spacebar");
    println!( "P2: Arrow Keys, Enter");
    println!( "F5 to reset the game");
    let mut c = conf::Conf::new();
    c.window_setup.title = GAME_TITLE.to_string();
    c.window_mode.width = WINDOW_WIDTH;
    c.window_mode.height = WINDOW_HEIGHT;
    let ctx = &mut Context::load_from_conf("ludum_dare", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}