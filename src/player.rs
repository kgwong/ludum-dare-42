use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

pub struct Player
{
    id: u32,
    pos_x: f32,
    pos_y: f32,
    width: u32,
    height: u32,
    vel_x: f32,
    vel_y: f32,
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
            pos_x: pos_x as f32,
            pos_y: pos_y as f32,
            width: 32,
            height: 32,
            vel_x: 0.0,
            vel_y: 0.0,
            sprite: graphics::Image::solid( ctx, 32, player_debug_color() ).unwrap(),
        }
    }

    pub fn update( &mut self )
    {
        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;
    }

    pub fn draw( &mut self, ctx: &mut Context ) -> GameResult<()>
    {
        let dest_point = graphics::Point2::new( self.pos_x, self.pos_y );
        graphics::draw(ctx, &self.sprite, dest_point, 0.0 );
        Ok(())
    }

    pub fn set_vel_x( &mut self, vel_x: f32 )
    {
        self.vel_x = vel_x;
    }

    pub fn set_vel_y( &mut self, vel_y: f32 )
    {
        self.vel_y = vel_y;
    }
}