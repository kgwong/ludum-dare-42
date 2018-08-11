use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

const TILE_SEPARATOR : usize = 1;

type TileRow = Vec<Tile>;

pub struct TileMap
{
    map: Vec<TileRow>,
    num_tiles_x: usize,
    num_tiles_y: usize,
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
        };

        for y_index in 0..num_tiles_y
        {
            let mut tile_row = Vec::new();
            for x_index in 0..num_tiles_x
            {
                tile_row.push( Tile::new( _ctx, x_index, y_index ) );
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
}

fn tile_test_color() -> graphics::Color
{
    graphics::Color::new(0.7, 0.8, 0.9, 1.0)
}

impl Tile
{
    pub fn new( ctx: &mut Context, index_x: usize, index_y: usize ) -> Tile
    { 
        Tile
        {
            pos_x: index_x * ( 32 + TILE_SEPARATOR ),
            pos_y: index_y * ( 32 + TILE_SEPARATOR ),
            state: TileState::FULL,
            sprite: graphics::Image::solid( ctx, 32, tile_test_color() ).unwrap(),
        }
    }

    pub fn draw( &mut self, ctx: &mut Context ) -> GameResult<()>
    {
        let dest_point = graphics::Point2::new( self.pos_x as f32, self.pos_y as f32 );
        graphics::draw(ctx, &self.sprite, dest_point, 0.0 );
        Ok(())
    }
}
