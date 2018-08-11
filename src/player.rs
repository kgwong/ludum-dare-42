use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

pub struct Player
{
    id: u32,
    pos_x: u32,
    pos_y: u32,
    width: u32,
    height: u32,
    sprite: graphics::Image,
}

fn player_debug_color() -> graphics::Color
{
    graphics::Color::new( 0.5, 0.5, 0.5, 1.0 )
}

impl Player
{
    pub fn new( ctx: &mut Context, id: u32, pos_x: u32, pos_y: u32 ) -> Player
    {
        Player
        { 
            id, 
            pos_x,
            pos_y,
            width: 32,
            height: 32,
            sprite: graphics::Image::solid( ctx, 32, player_debug_color() ).unwrap(),
        }
    }

    pub fn draw( &mut self, ctx: &mut Context ) -> GameResult<()>
    {
        let dest_point = graphics::Point2::new( self.pos_x as f32, self.pos_y as f32 );
        graphics::draw(ctx, &self.sprite, dest_point, 0.0 );
        Ok(())
    }
}