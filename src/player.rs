use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

use std::f32::{self, consts};

use tile::*;

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
    graphics::Color::new( 1.0, 0.0, 0.0, 1.0 )
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

    fn get_center( &self ) -> graphics::Point2
    {
        graphics::Point2::new( 
            ( self.pos_x + self.width as f32 / 2.0 ), 
            ( self.pos_y + self.height as f32 / 2.0 ),
            )
    }

    pub fn draw( &mut self, ctx: &mut Context ) -> GameResult<()>
    {       
        //let center = self.get_center();
        let top_right = graphics::Point2::new(self.pos_x, self.pos_y );
        let param = graphics::DrawParam {
            dest: top_right,
            scale: graphics::Point2::new( 6.0, 6.0,),
            rotation: self.get_facing_radians(),
            offset: graphics::Point2::new(0.5, 0.5),
            ..Default::default()
        };
        graphics::draw_ex(ctx, &self.sprite, param );

        //let test_sprite = graphics::Image::solid( ctx, 10, player_debug_color() ).unwrap();
        //let test_pos = graphics::Point2::new(self.pos_x, self.pos_y );
        //graphics::draw(ctx, &test_sprite, test_pos, 0.0 );
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

    pub fn pickup_tile( &mut self, tile_map: &mut TileMap )
    {
        let tile_distance : usize = 33;
        let mut tile_index_x : usize = self.pos_x as usize / tile_distance;
        let mut tile_index_y : usize = self.pos_y as usize / tile_distance;
        println!("{}, {}", tile_index_x, tile_index_y);
        match self.dir
        {
            Direction::UP => { tile_index_y -= 1; }
            Direction::DOWN => { tile_index_y += 1; }
            Direction::LEFT => { tile_index_x -= 1; }
            Direction::RIGHT => { tile_index_x += 1; }
        }       
        let tile : &mut Tile = &mut tile_map.map[tile_index_y][tile_index_x];
        match &tile.get_state()
        {
            TileState::FULL =>
            {
                tile.change_state( TileState::EMPTY );
            }
            _ => {}
        }
         
    }
}