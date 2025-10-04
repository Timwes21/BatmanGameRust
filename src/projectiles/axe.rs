use crate::game_defs::Direction;
use crate::projectiles::Projectile;

pub struct Axe{
    pub x: f32,
    pub y: f32,
    pub direction: Direction,
    pub move_speed: f32
}


impl Axe {
    pub fn new(x:f32, y:f32, direction: Direction)-> Self{
        let adjusted_y = y + 30.0;
        Self { x, y: adjusted_y, direction, move_speed: 10.0 }
    }

}

impl Projectile for Axe{
    fn get_x(&self)-> f32{
        self.x
    }

    fn set_x(&mut self, new_x: f32){
        self.x = new_x;
    }

    fn get_y(&self)-> f32{
        self.y
    }

    fn get_direction(&self)-> Direction{
        self.direction
    }

    fn get_move_speed(&self) -> f32{
        self.move_speed
    }
}