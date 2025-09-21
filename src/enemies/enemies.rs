use crate::batman::Batman;
use crate::enemies::{Axe, AxeGuy, Bullet, Enemy, FireMime, GunGuy, KnifeGuy, Mime, Rocket, RocketGuy};
use crate::projectiles::{self, Projectile};
use ggez::{Context};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Image, Mesh, Rect};
use crate::game_defs::{ GRAVITY, Direction };



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

pub enum Action {
    Walking,
    Standing,
    Attacking,
    Dying,
    Knockout,
    Sleep,
    Shooting,
    Knockback,
    Backflip,
    Throwing
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
    // pub fn draw(& mut self, canvas:&mut  Canvas, ctx: &Context, player_x: f32, player_width: f32){
    //     delegate!(self, draw(canvas, ctx, player_x, player_width));
    // }

    pub fn update(& mut self, ground_height: f32, batman:&mut Batman, batarang_xs: &Vec<f32>, projectiles: &mut Vec<Projectile>){
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

    fn draw(& mut self, canvas:&mut  Canvas, ctx: &Context, player_x: f32, player_width: f32){
        let enemy = match self {
            Enemies::Gun(e)        => e,
            Enemies::Knife(e)    => e,
            Enemies::Mime(e)         => e,
            Enemies::Rocket(e)  => e,
            Enemies::FireMime(e) => e,
            Enemies::AxeGuy(e)     => e
        };
        let enemy_width = enemy.get_width();
        let rough_y = enemy.y;
        let y = if enemy.action == Action::Sleep { 
            let res = enemy.get_drawn_y();
            res
        } else { 
            rough_y
        };


        
        let player_on_right = player_x + player_width/2.0 > enemy.rough_x + enemy_width/2.0;


        
        let direction = {
            if enemy.action == Action::Sleep {  
                let sleep_direction: f32 = if enemy.sleep_direction == Direction::Left { 3.2 } else { -3.2 };
                sleep_direction
            }
            else if player_on_right {
                let new_precise_x = enemy.get_precise_x_right();

                enemy.precise_x = new_precise_x;
                enemy.direction = Direction::Right;
                -3.2
            } 
            else {
                let new_precise_x = enemy.get_precise_x_left();
                enemy.precise_x = new_precise_x;
                if enemy.action == Action::Attacking && enemy.counter.round() >= 2.0 {
                    let diff = (enemy.attacking_sprites[2].width() - enemy.attacking_sprites()[0].width()) as f32;
                    let new_precise_x = enemy.precise_x - diff * 3.2;
                    enemy.precise_x = new_precise_x;
                }
                enemy.direction = Direction::Left;
                3.2
            }
            
        };
        

        let x = enemy.precise_x;
        
        canvas.draw(enemy.get_current_sprite(), DrawParam::default()
            .dest([x, y])
            .scale([direction, 3.2]));


        let health = if enemy.health > 0.0 { enemy.health * 2.0 } else { 0.0 };

        
        let health_bar_x = enemy.get_healthbar_x();

        let health_bar = Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(health_bar_x, y -60.0, health, 20.0), Color::from_rgb(0, 255, 0));
        let death_bar = Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(health_bar_x, y -60.0, enemy.death * 2.0, 20.0), Color::from_rgb(255, 0, 0));

        // let x_display = Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(self.get_rough_x(), y -60.0, self.get_death() * 2.0, 20.0), Color::from_rgb(255, 255, 255));



        let health_bar_res = match health_bar {
            Ok(m) => m,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        let death_bar_res = match death_bar {
            Ok(m) => m,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        // let x_display_res = match x_display {
        //     Ok(m) => m,
        //     Err(e) => {
        //         println!("{}", e);
        //         return;
        //     }
        // };


        canvas.draw(&death_bar_res, DrawParam::default()
            .dest([0.0, 10.0])
        );

        canvas.draw(&health_bar_res, DrawParam::default()
            .dest([0.0, 10.0])
        );

        // canvas.draw(&x_display_res, DrawParam::default()
        //     .dest([0.0, 10.0])
        // );


        
    }

    

    


}