use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

use std::f32::{self, consts};

use tile::*;
use projectile::*;
use hitbox::*;

const THROW_SPEED : f32 = 1.0;

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
    shadow_sprite: graphics::Image,
    tile: Option<Tile>,
    tile_image_id: usize,
    hitbox: Hitbox,
    is_dead: bool,
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
            shadow_sprite: graphics::Image::new( ctx, "/robo_shadow.png").unwrap(),
            tile: None,
            tile_image_id: 1,
            hitbox: Hitbox::new( pos_x as f32, pos_y as f32, 32.0, 32.0 ),
            is_dead: false,
        }
    }

    fn get_adj_vel_x( &mut self ) -> f32
    {
        if self.vel_x != 0.0 && self.vel_y != 0.0
        {
            self.vel_x * 0.7
        }
        else
        {
            self.vel_x
        }
    }

    fn get_adj_vel_y( &mut self ) -> f32
    {
        if self.vel_x != 0.0 && self.vel_y != 0.0
        {
            self.vel_y * 0.7
        }
        else
        {
            self.vel_y
        }
    }

    fn change_pos_from_vel( &mut self )
    {
        self.pos_x += self.get_adj_vel_x();
        self.pos_y += self.get_adj_vel_y();
    }

    pub fn update( &mut self, projectiles: &mut Vec<Projectile>, tile_map: &TileMap )
    {
        if self.is_dead
        {
            return;
        }
        self.change_pos_from_vel();
        self.hitbox.top_x = self.pos_x - self.width as f32 / 2.0;
        self.hitbox.top_y = self.pos_y - self.height as f32 / 2.0;

        //check if we are standing on top of a tile
        //let center = self.get_center();
        let tile_distance : usize = TILE_SPACE;
        let tile_index_x : usize = self.pos_x as usize / tile_distance;
        let tile_index_y : usize = self.pos_y as usize / tile_distance;

        let tile = &tile_map.map[tile_index_y][tile_index_x];
        match tile.get_state()
        {
            TileState::EMPTY => {
                self.is_dead = true;
            }
            _ => {}
        }

        for projectile in projectiles
        {
            if self.id != projectile.get_owner() && self.hitbox.check_collision( projectile.get_hitbox() )
            {
                projectile.kill();
                self.is_dead = true;
            }
        }
    }

    fn get_center( &self ) -> graphics::Point2
    {
        graphics::Point2::new( 
            self.pos_x + self.width as f32 / 2.0, 
            self.pos_y + self.height as f32 / 2.0,
            )
    }

    pub fn get_id( &self ) -> u32
    {
        self.id
    }

    pub fn is_dead( &self ) -> bool
    {
        self.is_dead
    }

    pub fn draw( &mut self, ctx: &mut Context ) -> GameResult<()>
    {       
        if self.is_dead
        {
            return Ok(());
        }

        //draw player shadow
        let top_right = graphics::Point2::new(self.pos_x + 5.0, self.pos_y + 2.0);
        let param = graphics::DrawParam {
            dest: top_right,
            rotation: self.get_facing_radians(),
            offset: graphics::Point2::new(0.5, 0.5),
            ..Default::default()
        };
        graphics::draw_ex(ctx, &self.shadow_sprite, param );
        //draw player   
        let top_right = graphics::Point2::new(self.pos_x, self.pos_y );
        let param = graphics::DrawParam {
            dest: top_right,
            rotation: self.get_facing_radians(),
            offset: graphics::Point2::new(0.5, 0.5),
            ..Default::default()
        };
        graphics::draw_ex(ctx, &self.sprite, param );

        /*let test_sprite = graphics::Image::solid( ctx, 32, player_debug_color() ).unwrap();
        let test_pos = graphics::Point2::new(self.pos_x - self.width as f32 / 2.0, self.pos_y - self.height as f32 / 2.0);
        graphics::draw(ctx, &test_sprite, test_pos, 0.0 );
        */

        let tile_draw_pos : graphics::Point2 = graphics::Point2::new
        (
            self.pos_x + self.get_tile_offset_x(),
            self.pos_y + self.get_tile_offset_y()
        );
        let shadow_draw_pos : graphics::Point2 = graphics::Point2::new
        (
            self.pos_x + self.get_tile_offset_x() + 2.0,
            self.pos_y + self.get_tile_offset_y() + 5.0
        );
        match self.tile
        {
            Some( ref mut tile ) =>
            {
                //draw the tile shadow
                let color = graphics::Color::new( 0.0, 0.0, 0.0, 0.7);
                let shadow = graphics::Image::solid( ctx, 32, color).unwrap();
                graphics::draw(ctx, &shadow, shadow_draw_pos, 0.0);
                //then draw the tile
                tile.draw_at_pos(ctx, &tile_draw_pos );
            }
            None => {}
        }
        Ok(())
    }

    pub fn set_vel_x( &mut self, vel_x: f32 )
    {
        self.vel_x = vel_x;
        if self.vel_x > 0.0 
        {
            self.dir = Direction::RIGHT;
        }
        else if self.vel_x < 0.0
        {
            self.dir = Direction::LEFT;
        }
    }

    pub fn set_vel_y( &mut self, vel_y: f32 )
    {
        self.vel_y = vel_y;
        if self.vel_y > 0.0
        {
            self.dir = Direction::DOWN;
        }
        else if self.vel_y < 0.0
        {
            self.dir = Direction::UP;
        }
    }

    pub fn on_dir_released( &mut self, dir: Direction )
    {
        match dir 
        {
            Direction::UP => 
            { 
                if self.vel_y < 0.0
                {
                    self.vel_y = 0.0;
                }
            }
            Direction::DOWN => 
            {
                if self.vel_y > 0.0
                {
                    self.vel_y = 0.0;
                }
            }
            Direction::LEFT =>
            {
                 if self.vel_x < 0.0
                 {
                     self.vel_x = 0.0;
                 }
            }
            Direction::RIGHT => 
            { 
                if self.vel_x > 0.0
                {
                    self.vel_x = 0.0;
                } 
            }         
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
            Direction::LEFT => { -48.0 }
            Direction::RIGHT => { 16.0 }
            _ => { -16.0 }
        }
    }

    fn get_tile_offset_y( &self ) -> f32
    {
        match self.dir
        {
            Direction::UP => { -48.0 }
            Direction::DOWN => { 16.0 }
            _ => { -16.0 }
        }
    }

    pub fn on_action( &mut self, ctx: &mut Context, tile_map: &mut TileMap, projectiles: &mut Vec<Projectile> )
    {
        if self.is_dead
        {
            return;
        }
        if self.tile.is_none()
        {
            self.pickup_tile(ctx, tile_map);
        }
        else
        {
            self.throw_tile(ctx, projectiles);
        }
    }

    fn throw_tile( &mut self, ctx: &mut Context, projectiles: &mut Vec<Projectile> )
    {
        self.tile = None;
        projectiles.push( Projectile::new( 
            ctx, 
            self.id, 
            self.pos_x + self.get_tile_offset_x(), 
            self.pos_y + self.get_tile_offset_y(), 
            self.get_adj_vel_x() + self.get_throw_vel_x(),
            self.get_adj_vel_y() + self.get_throw_vel_y(),
            self.tile_image_id ));
    }

    fn get_throw_vel_x( &self ) -> f32
    {
        match self.dir
        {
            Direction::LEFT => { -THROW_SPEED }
            Direction::RIGHT => { THROW_SPEED }
            _ => { 0.0 }
        }
    }

    fn get_throw_vel_y( &self ) -> f32
    {
        match self.dir
        {
            Direction::UP => { -THROW_SPEED }
            Direction::DOWN => { THROW_SPEED }
            _ => { 0.0 }
        }
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
        match self.dir
        {
            Direction::UP => { tile_index_y -= 1; }
            Direction::DOWN => { tile_index_y += 1; }
            Direction::LEFT => { tile_index_x -= 1; }
            Direction::RIGHT => { tile_index_x += 1; }
        }       
        let tile = &mut tile_map.map[tile_index_y][tile_index_x];
        match &tile.get_state()
        {
            TileState::FULL =>
            {
                tile.change_state( TileState::EMPTY );
                self.tile_image_id = tile.image_id;
                self.tile = Some( Tile::new( ctx, 0, 0, self.tile_image_id ) );
            }
            _ => {}
        }
         
    }
}