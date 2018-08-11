use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

use std::f32::{self, consts};

pub enum Direction
{
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Player
{
    id: u32,
    pos_x: f32,
    pos_y: f32,
    width: u32,
    height: u32,
    vel_x: f32,
    vel_y: f32,
    dir: Direction,
    sprite: graphics::Image,
}

fn player_debug_color() -> graphics::Color
{
    graphics::Color::new( 0.5, 0.5, 0.5, 1.0 )
}

impl Player
{
    pub fn new( ctx: &mut Context, id: u32, pos_x: u32, pos_y: u32, init_dir: Direction ) -> Player
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
            dir: init_dir,
            sprite: graphics::Image::new( ctx, "/robo.png" ).unwrap(),
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
        graphics::draw(ctx, &self.sprite, dest_point, self.get_facing_radians() );
        Ok(())
    }

    pub fn set_vel_x( &mut self, vel_x: f32 )
    {
        self.vel_x = vel_x;
        if (self.vel_x > 0.0 )
        {
            self.dir = Direction::RIGHT;
        }
        else if ( self.vel_x < 0.0 )
        {
            self.dir = Direction::LEFT;
        }
    }

    pub fn set_vel_y( &mut self, vel_y: f32 )
    {
        self.vel_y = vel_y;
        if (self.vel_y > 0.0 )
        {
            self.dir = Direction::DOWN;
        }
        else if ( self.vel_y < 0.0 )
        {
            self.dir = Direction::UP;
        }
    }

    fn get_facing_radians( &self ) -> f32
    {
        self.get_facing_degrees().to_radians()
    }

    fn get_facing_degrees( &self ) -> f32
    {
        match self.dir
        {
            Direction::UP => { 180.0 }
            Direction::DOWN => { 0.0 }
            Direction::LEFT => { 90.0 }
            Direction::RIGHT => { 270.0 }
        }
    }
}