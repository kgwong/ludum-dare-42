use hitbox::Hitbox;

use ggez::graphics;
use ggez::Context;

use tile::*;

use player::*;

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
    is_dead: bool,
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
            hitbox: Hitbox::new( pos_x, pos_y, TILE_SIZE as f32, TILE_SIZE as f32 ),
            sprite_id,
            sprite: get_image(ctx, sprite_id),
            is_dead: false,
        }
    }

    pub fn update( &mut self, factor: f32 )
    {
        self.pos_x += self.vel_x * factor;
        self.pos_y += self.vel_y * factor;
        self.hitbox.top_x = self.pos_x;
        self.hitbox.top_y = self.pos_y;
    }

    pub fn draw( &mut self, ctx: &mut Context )
    {
        //draaw shadow
        let shadow_draw_pos : graphics::Point2 = graphics::Point2::new
        (
            self.pos_x + 2.0,
            self.pos_y + 5.0
        );
        let color = graphics::Color::new( 0.0, 0.0, 0.0, 0.7);
        let shadow = graphics::Image::solid( ctx, 32, color).unwrap();
        graphics::draw(ctx, &shadow, shadow_draw_pos, 0.0);

        //draw self
        let dest_point = graphics::Point2::new( self.pos_x, self.pos_y);
        graphics::draw(ctx, &self.sprite, dest_point, 0.0 );
    }

    pub fn is_dead( &self ) -> bool
    {
        //we also need to check out of bounds
        self.is_dead
    }

    pub fn kill( &mut self ) 
    {
        self.is_dead = true;
    }

    pub fn get_hitbox( &self ) -> &Hitbox
    {
        &self.hitbox
    }

    pub fn get_owner( &self ) -> u32
    {
        self.owner
    }

}