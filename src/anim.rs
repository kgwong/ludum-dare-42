use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

pub struct Anim
{
    pos_x: f32,
    pos_y: f32,
    frame: u32,
    end: u32,
    wait: u32,
    //sprite: graphics::Image,
    is_dead: bool,
    prefix: String,
}

impl Anim
{
    pub fn new( ctx: &mut Context, pos_x: f32, pos_y: f32, prefix: String, end: u32, wait: u32 ) -> Anim
    { 
        //let path = prefix + "1.png";
        Anim
        {
            pos_x,
            pos_y,
            frame: 1,
            end,
            wait,
            //sprite : graphics::Image::new(ctx, path ).unwrap(),
            is_dead : false,
            prefix,
        }
    }

    pub fn update( &mut self )
    {
        if self.frame >= self.end * self.wait
        {
            self.is_dead = true;
            return;
        }
        self.frame += 1;
    }

    pub fn draw( &mut self, ctx: &mut Context )
    {
        let dest_point = graphics::Point2::new( self.pos_x as f32, self.pos_y as f32 );
        let mut f : u32 = self.frame / self.wait + 1;
        if f > self.end
        {
            f = self.end;
        }
        let path = format!("{}{}.png", self.prefix, f );
        let sprite = graphics::Image::new(ctx, path ).unwrap();
        graphics::draw(ctx, &sprite, dest_point, 0.0 );
    }

    pub fn is_dead( &self ) -> bool
    {
        self.is_dead
    }
}