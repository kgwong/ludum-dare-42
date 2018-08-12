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
    tile: Option<Tile>,
    tile_image_id: usize,
}

fn player_debug_color() -> graphics::Color
{
    graphics::Color::new( 1.0, 0.0, 0.0, 1.0 )
}

fn tile_test_color() -> graphics::Color
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
            tile: None,
            tile_image_id: 1,
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
            rotation: self.get_facing_radians(),
            offset: graphics::Point2::new(0.5, 0.5),
            ..Default::default()
        };
        graphics::draw_ex(ctx, &self.sprite, param );

        let test_sprite = graphics::Image::solid( ctx, 10, player_debug_color() ).unwrap();
        let test_pos = graphics::Point2::new(self.pos_x, self.pos_y );
        graphics::draw(ctx, &test_sprite, test_pos, 0.0 );

        let tile_draw_pos : graphics::Point2 = graphics::Point2::new
        (
            self.pos_x + self.get_tile_offset_x(),
            self.pos_y + self.get_tile_offset_y() 
        );
        match self.tile
        {
            Some( ref mut tile ) =>
            {
                tile.draw_at_pos(ctx, &tile_draw_pos );
            }
            None => {}
        }
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

    fn get_tile_offset_x( &self ) -> f32
    {
        match self.dir
        {
            Direction::LEFT => { -32.0 }
            Direction::RIGHT => { 32.0 }
            _ => { 0.0 }
        }
    }

    fn get_tile_offset_y( &self ) -> f32
    {
        match self.dir
        {
            Direction::UP => { -32.0 }
            Direction::DOWN => { 32.0 }
            _ => { 0.0 }
        }
    }

    pub fn on_action( &mut self, ctx: &mut Context, tile_map: &mut TileMap )
    {
        if self.tile.is_none()
        {
            self.pickup_tile(ctx, tile_map);
        }
        else
        {
            self.throw_tile(ctx, tile_map);
        }
    }

    fn throw_tile( &mut self, ctx: &mut Context, tile_map: &mut TileMap )
    {
        self.tile = None;
    }

    pub fn pickup_tile( &mut self, ctx: &mut Context, tile_map: &mut TileMap )
    {
        if self.tile.is_some() 
        {
            return;
        }

        let tile_distance : usize = TILE_SPACE;
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
                let image = graphics::Image::solid( ctx, 32, tile_test_color() ).unwrap();
                self.tile_image_id = tile.image_id;
                self.tile = Some( Tile::new( ctx, 0, 0, self.tile_image_id ) );
            }
            _ => {}
        }
         
    }
}