use crate::batman::Batman;
use crate::enemies::{AxeGuy, Enemy, FireMime, GunGuy, KnifeGuy, Mime, RocketGuy};
use crate::projectiles::Projectiles;
use ggez::{Context};
use ggez::graphics::Canvas;



macro_rules! delegate {
    ($self:expr, $method:ident($($arg:expr), *)) => {
        match $self {
            Enemies::Gun(e)      => e.$method($($arg), *),
            Enemies::Knife(e)    => e.$method($($arg), *),
            Enemies::Mime(e)     => e.$method($($arg), *),
            Enemies::Rocket(e)   => e.$method($($arg), *),
            Enemies::FireMime(e) => e.$method($($arg), *),
            Enemies::AxeGuy(e)   => e.$method($($arg), *)
        }
    };
}

pub enum Enemies{
    Knife(KnifeGuy),
    Gun(GunGuy),
    Mime(Mime),
    Rocket(RocketGuy),
    FireMime(FireMime),
    AxeGuy(AxeGuy)
}


impl Enemies {
    pub fn draw(& mut self, canvas:&mut  Canvas, ctx: &Context, player_x: f32, player_width: f32){
        delegate!(self, draw(canvas, ctx, player_x, player_width));
    }

    pub fn update(& mut self, ground_height: f32, batman:&mut Batman, batarang_xs: &Vec<f32>, projectiles: &mut Vec<Projectiles>){
        delegate!(self, update(ground_height, batman, batarang_xs));
        match self {
            Enemies::Gun (e)       => e.add_bullets(projectiles),
            Enemies::AxeGuy(e)     => e.add_axes(projectiles),
            Enemies::Rocket(e)  => e.add_rockets(projectiles),
            _ => {}
        }
    }
    

    pub fn is_alive(&self)-> bool {
        delegate!(self, is_alive())
    }   

    


}