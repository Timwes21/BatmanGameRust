use ggez::{graphics::Canvas, Context, GameResult};

use crate::{batman::Batman, enemies::{Axe, Bullet, Rocket}};


pub enum Projectile {
    Bullet(Bullet),
    Axe(Axe),
    Rocket(Rocket)
}

impl Projectile {
    pub fn update(&mut self){
        match self {
            Projectile::Axe(e) => e.update(),
            Projectile::Bullet(e) => e.update(),
            Projectile::Rocket (e) => e.update(),
        }
    }


    pub fn getting_rid_of(&self, batman: &mut Batman)->bool{
        match self {
            Projectile::Axe(e) => e.is_offscreen() || e.hit_batman(batman),
            Projectile::Bullet(e) => e.is_offscreen() || e.hit_batman(batman),
            Projectile::Rocket (e) => e.is_offscreen() || e.hit_batman(batman),
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas, ctx: &mut Context){
        match self {
            Projectile::Axe(e) => e.draw(canvas, ctx),
            Projectile::Bullet(e) => e.draw(canvas, ctx),
            Projectile::Rocket (e) => e.draw(canvas, ctx),
        };
    }
}