use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

use tilesheet::SheetMap;

pub const TILE_SEPARATOR : usize = 2;
pub const TILE_SIZE : usize = 32;
pub const TILE_SPACE : usize = TILE_SEPARATOR + TILE_SIZE;

type TileRow = Vec<Tile>;

pub struct TileMap
{
    pub map: Vec<TileRow>,
    num_tiles_x: usize,
    num_tiles_y: usize,
    sheetmap: SheetMap, 
}

impl TileMap
{
    pub fn new( _ctx: &mut Context, num_tiles_x: usize, num_tiles_y: usize ) -> TileMap
    {
        let mut res = TileMap
        {
            map: Vec::new(),
            num_tiles_x,
            num_tiles_y,
            sheetmap: SheetMap::new( _ctx )
        };

        for y_index in 0..num_tiles_y
        {
            let mut tile_row = Vec::new();
            for x_index in 0..num_tiles_x
            {
                //edge tiles
                if ( y_index == 0 || y_index == num_tiles_y -1 || x_index == 0 || x_index == num_tiles_x -1 )
                {
                    tile_row.push( Tile::new_empty( _ctx, x_index, y_index ) );
                }
                else
                {
                    //plus one on indices because we have a border
                    tile_row.push( Tile::new( _ctx, x_index, y_index, res.sheetmap.map[y_index-1][x_index-1] ) );
                }
            }

            res.map.push( tile_row );
        }
        res
    }
    
    pub fn draw( &mut self, ctx: &mut Context ) -> GameResult<()>
    {
        for y_index in 0..self.num_tiles_y
        {
            for x_index in 0..self.num_tiles_x
            {
                self.map[y_index][x_index].draw( ctx );
            }
        }
        Ok(())
    }
}

#[derive(PartialEq)]
pub enum TileState 
{
    FULL,
    EMPTY,
}

pub struct Tile
{
    pos_x: usize,
    pos_y: usize,
    state: TileState,
    sprite: graphics::Image,
    pub image_id: usize,
}

fn tile_test_color() -> graphics::Color
{
    graphics::Color::new(0.7, 0.8, 0.9, 1.0)
}

fn tile_missing_color() -> graphics::Color
{
    graphics::Color::new(0.3, 0.3, 0.3, 1.0)
}

fn get_image( ctx: &mut Context, id: usize ) -> graphics::Image
{
    let path = format!( "/tiles/sunflower_{:02}.png", id );
    println!( "{}", path );
    graphics::Image::new( ctx, path ).unwrap()
}

impl Tile
{
    pub fn new_empty( ctx: &mut Context, index_x: usize, index_y: usize ) -> Tile
    { 
        Tile
        {
            pos_x: index_x * ( TILE_SIZE + TILE_SEPARATOR ),
            pos_y: index_y * ( TILE_SIZE + TILE_SEPARATOR ),
            state: TileState::EMPTY,
            sprite: graphics::Image::solid( ctx, 32, tile_missing_color() ).unwrap(),
            image_id: 0,
        }
    }
    pub fn new( ctx: &mut Context, index_x: usize, index_y: usize, image_id: usize ) -> Tile
    { 
        Tile
        {
            pos_x: index_x * ( TILE_SIZE + TILE_SEPARATOR ),
            pos_y: index_y * ( TILE_SIZE + TILE_SEPARATOR ),
            state: TileState::FULL,
            sprite: get_image( ctx, image_id),
            image_id: image_id
        }
    }

    pub fn draw_at_pos( &mut self, ctx: &mut Context, pos: &graphics::Point2 )
    {
        match self.state
        {
            TileState::FULL =>
            {
                //self.sprite = graphics::Image::solid( ctx, 32, tile_test_color() ).unwrap();
            }
            TileState::EMPTY =>
            {
                self.sprite = graphics::Image::solid( ctx, 32, tile_missing_color() ).unwrap();
            }
        }
        graphics::draw(ctx, &self.sprite, *pos, 0.0 );
    } 

    pub fn draw( &mut self, ctx: &mut Context )
    {
        let dest_point = graphics::Point2::new( self.pos_x as f32, self.pos_y as f32 );
        self.draw_at_pos(ctx, &dest_point);
    }

    pub fn get_state( &self ) -> &TileState
    {
        &self.state
    }

    pub fn change_state( &mut self, tile_state: TileState ) 
    {
        self.state = tile_state;
    }
}
