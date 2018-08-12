use ggez::*;
use ggez::event::*;
use ggez::timer::*;
use ggez::graphics::{DrawMode, Point2};

use player::*;
use tile::*;
use tilesheet::*;
use projectile::*;

pub const NUM_TILES_X : usize = TILE_SHEET_NUM_ACROSS + 2;
pub const NUM_TILES_Y : usize = TILE_SHEET_NUM_DOWN + 2;

const PLAYER_SPEED : f32 = 2.0;
const EXPECTED_FRAME_RATE : f64 = 60.0;
const EXPECTED_TIME_BETWEEN_FRAMES : f64 = 1.0/EXPECTED_FRAME_RATE;

const P1_UP : event::Keycode = Keycode::W;
const P1_DOWN : event::Keycode = Keycode::S;
const P1_LEFT : event::Keycode = Keycode::A;
const P1_RIGHT : event::Keycode = Keycode::D;
const P1_ACTION : event::Keycode = Keycode::Space;

const P2_UP : event::Keycode = Keycode::Up;
const P2_DOWN : event::Keycode = Keycode::Down;
const P2_LEFT : event::Keycode = Keycode::Left;
const P2_RIGHT : event::Keycode = Keycode::Right;
const P2_ACTION : event::Keycode = Keycode::Return;

pub struct MainState 
{
    player1 : Player,
    player2 : Player, 
    tile_map: TileMap,
    projectiles: Vec<Projectile>,
}

impl MainState 
{
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> 
    {
        let s = MainState 
        { 
            player1 : Player::new( _ctx, 1, ::WINDOW_WIDTH / 2, 160, Direction::DOWN ),
            player2 : Player::new( _ctx, 2, ::WINDOW_WIDTH / 2, ::WINDOW_HEIGHT - 160, Direction::UP ),
            tile_map: TileMap::new( _ctx, NUM_TILES_X, NUM_TILES_Y ),
            projectiles: Vec::new(),
        };
        Ok(s)
    }

    fn reset( &mut self, _ctx: &mut Context)
    {
        self.player1 = Player::new( _ctx, 1, ::WINDOW_WIDTH / 2, 160, Direction::DOWN );
        self.player2 = Player::new( _ctx, 2, ::WINDOW_WIDTH / 2, ::WINDOW_HEIGHT - 160, Direction::UP );
        self.tile_map = TileMap::new( _ctx, NUM_TILES_X, NUM_TILES_Y );
        self.projectiles = Vec::new();
    }
}

impl event::EventHandler for MainState 
{
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> 
    {

        let delta = (timer::duration_to_f64(timer::get_delta(_ctx))) as f32;
        let factor = delta / (EXPECTED_TIME_BETWEEN_FRAMES) as f32;

        if timer::get_ticks(_ctx) % 1000 == 0 {
            println!("Average FPS: {}", timer::get_fps(_ctx));
            println!("Factor is  {}", factor);
        }

        self.player1.update( &mut self.projectiles, &self.tile_map, factor );
        self.player2.update( &mut self.projectiles, &self.tile_map, factor );
        for ref mut projectile in &mut self.projectiles
        {
            projectile.update(factor);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> 
   {
        graphics::clear(ctx);

        self.tile_map.draw( ctx );
        self.player1.draw( ctx );
        self.player2.draw( ctx );
        for ref mut projectile in &mut self.projectiles
        {
            projectile.draw( ctx );
        }

        self.projectiles.retain(|projectile| {
            !projectile.is_dead()
        }
        );
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

            P1_ACTION => { self.player1.on_action( ctx, &mut self.tile_map, &mut self.projectiles ) }
            P2_ACTION => { self.player2.on_action( ctx, &mut self.tile_map, &mut self.projectiles ) }
            
            Keycode::F5 => { self.reset( ctx ); }

            _ => {}
        }
    }

    fn key_up_event( &mut self, ctx: &mut Context, keycode: Keycode, _: Mod, _: bool )
    {
        match keycode
        {
            P1_UP => { self.player1.on_dir_released(Direction::UP) }
            P1_DOWN => { self.player1.on_dir_released(Direction::DOWN) }
            P1_LEFT => { self.player1.on_dir_released(Direction::LEFT)}
            P1_RIGHT => { self.player1.on_dir_released(Direction::RIGHT)}
            P2_UP => { self.player2.on_dir_released(Direction::UP)}
            P2_DOWN => { self.player2.on_dir_released(Direction::DOWN) }
            P2_LEFT => { self.player2.on_dir_released(Direction::LEFT) }
            P2_RIGHT => { self.player2.on_dir_released(Direction::RIGHT)}
            _ => {}
        }
        
    }
}