use ggez::*;
use ggez::graphics::{DrawMode, Point2};

use player::Player;

pub struct MainState {
    player1 : Player,
    player2 : Player, 
    pos_x: f32,
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState 
        { 
            player1 : Player::new( _ctx, 1, 100, 100 ),
            player2 : Player::new( _ctx, 2, 200, 200 ),
            pos_x: 0.0 
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        self.player1.draw( ctx );
        self.player2.draw( ctx );
        graphics::present(ctx);
        Ok(())
    }
}