use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

const NUM_ACROSS : usize = 8;
const NUM_DOWN : usize = 16; 

type SheetRow = Vec<usize>;

pub struct SheetMap
{
    pub map: Vec<SheetRow>,
    num_tiles_across: usize,
    num_tiles_down: usize,
}


impl SheetMap
{
    pub fn new( _ctx: &mut Context ) -> SheetMap
    {
        let mut res = SheetMap
        {
            map: Vec::new(),
            num_tiles_across : NUM_ACROSS,
            num_tiles_down : NUM_DOWN,
        };

        let mut i : usize = 1;
        for y_index in 0..res.num_tiles_down
        {
            let mut tile_row = Vec::new();
            for x_index in 0..res.num_tiles_across
            {
                tile_row.push( i );
                i += 1;
            }

            res.map.push( tile_row );
        }
        res
    }
}
