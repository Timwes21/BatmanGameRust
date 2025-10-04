use ggez::{graphics::Canvas, Context};
use crate::projectiles::{Rocket, Bullet, Axe};
use crate::projectiles::Projectile;

use crate::batman::Batman;



pub enum Projectiles {
    Bullet(Bullet),
    Axe(Axe),
    Rocket(Rocket)
}

impl Projectiles {
    pub fn update(&mut self){
        match self {
            Projectiles::Axe(e)       => e.update(),
            Projectiles::Bullet(e) => e.update(),
            Projectiles::Rocket(e) => e.update(),
        }
    }


    pub fn getting_rid_of(&self, batman: &mut Batman)->bool{
        match self {
            Projectiles::Axe(e)       => e.is_offscreen() || e.hit_batman(batman, 15.0),
            Projectiles::Bullet(e) => e.is_offscreen() || e.hit_batman(batman, 1.0),
            Projectiles::Rocket(e) => e.is_offscreen() || e.hit_batman(batman, 15.0),
        }
    }
    
    pub fn draw(&mut self, canvas: &mut Canvas, ctx: &mut Context){
        match self {
            Projectiles::Axe(e)       => e.draw(canvas, ctx),
            Projectiles::Bullet(e) => e.draw(canvas, ctx),
            Projectiles::Rocket(e) => e.draw(canvas, ctx),
        };
    }
}