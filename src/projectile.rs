use hitbox::Hitbox;

use ggez::graphics;
use ggez::Context;

use tile::*;

pub struct Projectile
{
    owner: u32,
    pos_x: f32,
    pos_y: f32,
    vel_x: f32,
    vel_y: f32,
    hitbox: Hitbox,
    sprite_id: usize,
    sprite: graphics::Image,
}

impl Projectile
{
    pub fn new( ctx: &mut Context, owner_id: u32, pos_x: f32, pos_y: f32, vel_x: f32, vel_y: f32, sprite_id: usize ) -> Projectile
    {
        Projectile
        { 
            owner: owner_id,
            pos_x,
            pos_y,
            vel_x,
            vel_y,
            hitbox: Hitbox::new(),
            sprite_id,
            sprite: get_image(ctx, sprite_id)
        }
    }

    pub fn update( &mut self )
    {
        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;
    }

    pub fn draw( &mut self, ctx: &mut Context )
    {
        let dest_point = graphics::Point2::new( self.pos_x as f32, self.pos_y as f32 );
        graphics::draw(ctx, &self.sprite, dest_point, 0.0 );
    }


}