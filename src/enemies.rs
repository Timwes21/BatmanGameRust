use crate::enemy::Enemy;
use crate::batman::Batman;
use crate::knife_guy::KnifeGuy;
use crate::gun_guy::{Bullet, GunGuy};
use ggez::{Context};
use ggez::graphics::{Canvas};


pub enum Enemies{
    Knife(KnifeGuy),
    Gun(GunGuy)
}


impl Enemies {
    pub fn draw(& mut self, canvas:&mut  Canvas, ctx: &Context, player_x: f32, player_width: f32){

        match self {
            Enemies::Gun(e) => e.draw(canvas, ctx, player_x, player_width),
            Enemies::Knife(e) => e.draw(canvas, ctx, player_x, player_width), 
        }
    }

    pub fn update(& mut self, ground_height: f32, batman:&mut Batman, batarang_xs: &Vec<f32>, bullets: &mut Vec<Bullet>){
        match self {
            Enemies::Gun(e) => {
                e.update(ground_height, batman, batarang_xs);
                e.add_bullets(bullets);
            },
            Enemies::Knife(e) => e.update(ground_height, batman, batarang_xs),
        }
    }

    pub fn is_alive(&self)-> bool {
        match self {
            Enemies::Gun(e) => e.is_alive(),
            Enemies::Knife(e) => e.is_alive(),
        }

    }

}