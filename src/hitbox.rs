
pub struct Hitbox
{
    pub top_x: f32,
    pub top_y: f32,
    pub width: f32,
    pub height: f32,
}

impl Hitbox
{
    pub fn new( top_x: f32, top_y: f32, width: f32, height: f32 ) -> Hitbox
    {
        Hitbox{
            top_x,
            top_y,
            width,
            height,
        }
    }

    fn get_top(&self) -> f32
    {
        self.top_y
    }

    fn get_bottom(&self) -> f32
    {
        self.top_y + self.height
    }

    fn get_left(&self) -> f32
    {
        self.top_x
    }

    fn get_right(&self) -> f32
    {
        self.top_x + self.width
    }

    pub fn check_collision( &self, other : &Hitbox ) -> bool
    {
        if other.get_bottom() < self.get_top()
        {
            return false;
        }
        if other.get_right() < self.get_left()
        {
            return false;
        }

        other.get_top() < self.get_bottom() && other.get_left() < self.get_right()
    }
}

