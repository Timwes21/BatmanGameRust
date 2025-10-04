use crate::game_defs::{WIDTH, Direction};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh};
use ggez::{Context, GameResult};
use crate::batman::Batman;

pub trait Projectile {
    fn update(&mut self){
        let new_x = self.get_x() + if self.get_direction() == Direction::Left { -self.get_move_speed() } else { self.get_move_speed() };
        self.set_x(new_x);
    }

    fn draw(& mut self, canvas:&mut  Canvas, ctx: &Context)-> GameResult{
    let bullet = Mesh::
                            new_circle(ctx, DrawMode::fill(), 
                            [self.get_x(), self.get_y()], 25.0, 0.1, 
                            Color::from_rgb(0, 0, 0))?;

        canvas.draw(&bullet, DrawParam::default()
            .dest([0.0, 10.0])
        );

        Ok(())
    }

    fn is_offscreen(&self) -> bool{
        if self.get_x() > WIDTH || self.get_x() < 0.0{ true } else { false }
    }

    fn hit_batman(&self, batman: &mut Batman, damage: f32)-> bool{
        let batman_mid_point = batman.get_mid_point().round();
        let bullet_x = self.get_x().round();
        if bullet_x >= batman_mid_point - 20.0 && batman_mid_point + 20.0 >= bullet_x && batman.is_grounded(){
            // batman.take_damage(15.0);
            batman.take_damage(damage);
            true
        }
        else {
            false
        }
    }

    fn get_x(&self)-> f32;

    fn set_x(&mut self, new_x: f32);

    fn get_y(&self)-> f32;


    fn get_direction(&self)-> Direction;

    fn get_move_speed(&self) -> f32;
}

