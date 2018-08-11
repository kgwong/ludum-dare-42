use ggez::*;
use ggez::event::*;
use ggez::timer::*;
use ggez::graphics::{DrawMode, Point2};

use player::Player;
use tile::TileMap;
use tile::Tile;

const NUM_TILES_X : usize = 30;
const NUM_TILES_Y : usize = 15;

const PLAYER_SPEED : f32 = 2.0;

const P1_UP : event::Keycode = Keycode::W;
const P1_DOWN : event::Keycode = Keycode::S;
const P1_LEFT : event::Keycode = Keycode::A;
const P1_RIGHT : event::Keycode = Keycode::D;

const P2_UP : event::Keycode = Keycode::Up;
const P2_DOWN : event::Keycode = Keycode::Down;
const P2_LEFT : event::Keycode = Keycode::Left;
const P2_RIGHT : event::Keycode = Keycode::Right;

pub struct MainState 
{
    player1 : Player,
    player2 : Player, 
    tile_map : TileMap, 
}

impl MainState 
{
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> 
    {
        let s = MainState 
        { 
            player1 : Player::new( _ctx, 1, 100, 100 ),
            player2 : Player::new( _ctx, 2, 200, 200 ),
            tile_map: TileMap::new( _ctx, NUM_TILES_X, NUM_TILES_Y ),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState 
{
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> 
    {
        println!("FPS: {}", timer::get_fps(_ctx ) );
        self.player1.update();
        self.player2.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> 
   {
        graphics::clear(ctx);
        self.tile_map.draw( ctx );
        self.player1.draw( ctx );
        self.player2.draw( ctx );
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event( &mut self, ctx: &mut Context, keycode: Keycode, _: Mod, _: bool )
    {
        match keycode
        {
            P1_UP => { self.player1.set_vel_y( -PLAYER_SPEED ) }
            P1_DOWN => { self.player1.set_vel_y( PLAYER_SPEED ) }
            P1_LEFT => { self.player1.set_vel_x( -PLAYER_SPEED ) }
            P1_RIGHT => { self.player1.set_vel_x( PLAYER_SPEED ) }
            P2_UP => { self.player2.set_vel_y( -PLAYER_SPEED ) }
            P2_DOWN => { self.player2.set_vel_y( PLAYER_SPEED ) }
            P2_LEFT => { self.player2.set_vel_x( -PLAYER_SPEED ) }
            P2_RIGHT => { self.player2.set_vel_x( PLAYER_SPEED ) }
            _ => {}
        }
    }

    fn key_up_event( &mut self, ctx: &mut Context, keycode: Keycode, _: Mod, _: bool )
    {
        match keycode
        {
            P1_UP => { self.player1.set_vel_y( 0.0 ) }
            P1_DOWN => { self.player1.set_vel_y( 0.0 ) }
            P1_LEFT => { self.player1.set_vel_x( 0.0 ) }
            P1_RIGHT => { self.player1.set_vel_x( 0.0 ) }
            P2_UP => { self.player2.set_vel_y( 0.0 ) }
            P2_DOWN => { self.player2.set_vel_y( 0.0 ) }
            P2_LEFT => { self.player2.set_vel_x( 0.0 ) }
            P2_RIGHT => { self.player2.set_vel_x( 0.0 ) }
            _ => {}
        }
    }
}