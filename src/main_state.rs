use ggez::*;
use ggez::graphics::{DrawMode, Point2};

use player::Player;
use tile::TileMap;
use tile::Tile;

const NUM_TILES_X : usize = 30;
const NUM_TILES_Y : usize = 15;

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
}