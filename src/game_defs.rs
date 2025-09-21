use crate::batman::{ Batman};
use ggez::{Context, GameResult};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh};


pub const WIDTH: f32 = 2200.0;
pub const HEIGHT: f32 = 1200.0;
pub const GRAVITY: f32 = 25.0;
pub const SCALE: f32 = 3.4;



#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right
}


// pub struct Projectile{
//     x: f32,
//     y: f32,
//     direction: Direction,
//     move_speed: f32
// }


// pub trait Projectile {
//     fn new(x:f32, y:f32, direction: Direction) -> Self {
//         let adjusted_y = y + 30.0;
//         Self { x, y: adjusted_y, direction, move_speed: 10.0 }
//     }

//     pub fn update(&mut self){
//         self.x += if self.direction == Direction::Left { -self.move_speed } else { self.move_speed };
//     }

//     pub fn draw(& mut self, canvas:&mut  Canvas, ctx: &Context) -> GameResult {
//         let bullet = Mesh::new_circle(ctx, DrawMode::fill(), [self.x, self.y], 15.0, 0.1, Color::from_rgb(0, 0, 0))?;

//         canvas.draw(&bullet, DrawParam::default()
//             .dest([0.0, 10.0])
//         );

//         Ok(())
//     }

//     pub fn is_offscreen(&self) -> bool{
//         if self.x > WIDTH || self.x < 0.0{ true } else { false }
//     }

//     pub fn hit_batman(&self, batman: &mut Batman)-> bool{
//         let batman_mid_point = batman.get_mid_point().round();
//         let bullet_x = self.x.round();
//         if bullet_x >= batman_mid_point - 20.0 && batman_mid_point + 20.0 >= bullet_x && batman.is_grounded(){
//             batman.take_damage(3.0);
//             true
//         }
//         else {
//             false
//         }
//     }
// }