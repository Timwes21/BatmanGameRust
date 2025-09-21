use crate::batman::{Batarang, Batman};
use crate::enemies::{knife_guy, GunGuy, KnifeGuy, Mime, RocketGuy, Enemies};
use crate::sprites::get_sprites;
use ggez::{Context, GameResult};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Image, Mesh, Rect};
use crate::game_defs::{ GRAVITY, Direction };


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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



pub struct EnemyBase {
    pub rough_x: f32,
    pub precise_x: f32,
    pub y: f32,
    pub action: Action,
    pub knockout_counter: f32,
    pub move_speed: f32,
    pub counter: f32,
    pub health: f32,
    pub death: f32,
    pub dead: bool,
    pub direction: Direction,
    pub sleep_direction: Direction,
}



pub trait Enemy {
    fn draw(& mut self, canvas:&mut  Canvas, ctx: &Context, player_x: f32, player_width: f32){
        let enemy_width = self.get_width();
        let rough_y = self.get_y();
        let y = if self.get_action() == Action::Sleep { 
            let res = self.get_drawn_y();
            res
        } else { 
            rough_y
        };


        
        let player_on_right = player_x + player_width/2.0 > self.get_rough_x() + enemy_width/2.0;


        
        let direction = {
            if self.get_action() == Action::Sleep {  
                let sleep_direction: f32 = if self.get_sleep_direction() == Direction::Left { 3.2 } else { -3.2 };
                sleep_direction
            }
            else if player_on_right {
                let new_precise_x = self.get_precise_x_right();

                self.set_precise_x(new_precise_x);
                self.set_direction(Direction::Right);
                -3.2
            } 
            else {
                let new_precise_x = self.get_precise_x_left();
                self.set_precise_x(new_precise_x);
                if self.get_action() == Action::Attacking && self.get_counter().round() >= 2.0 {
                    let diff = (self.get_attacking_sprites()[2].width() - self.get_attacking_sprites()[0].width()) as f32;
                    let new_precise_x = self.get_precise_x() - diff * 3.2;
                    enemy.precise_x = new_precise_x;
                }
                self.set_direction(Direction::Left);
                3.2
            }
            
        };
        

        let x = self.get_precise_x();
        
        canvas.draw(self.get_current_sprite(), DrawParam::default()
            .dest([x, y])
            .scale([direction, 3.2]));


        let health = if self.get_health() > 0.0 { self.get_health() * 2.0 } else { 0.0 };

        
        let health_bar_x = self.get_healthbar_x();

        let health_bar = Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(health_bar_x, y -60.0, health, 20.0), Color::from_rgb(0, 255, 0));
        let death_bar = Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(health_bar_x, y -60.0, self.get_death() * 2.0, 20.0), Color::from_rgb(255, 0, 0));

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

    fn update(& mut self, ground_height: f32, batman:&mut Batman, batarang_xs: &Vec<f32>){
        self.gravity(ground_height);
        self.check_for_batarang_damage(batarang_xs);
        self.movement(batman);
    }
        


    fn gravity(&mut self, ground_height: f32){
        let y = self.get_y();
        if (y + ((self.get_current_sprite().height() * 3)) as f32) < ground_height {
            let new_y = y + GRAVITY;
            self.set_y(new_y);
        }
        else {
            let new_y = ground_height - ((self.get_current_sprite().height() * 3)-35) as f32;
            self.set_y(new_y);
        }
    }

    fn check_for_batarang_damage(&mut self, batarang_xs: &Vec<f32>){
        for i in batarang_xs{
            let x = self.get_rough_x().round();
            if i.round() >= x-10.0 && i.round() <= x+10.0 {    
                let new_action = Action::Knockout;
                self.set_action(new_action);
                let new_health = self.get_health() - 1.0;
                self.set_health(new_health);
                self.reset_counter();
            }
        }
    }

    fn get_mid_point(&mut self)-> f32{
        self.get_rough_x() + (self.get_current_sprite().width() as f32 * 3.4)/2.0
    }    

    fn batman_attack(&mut self, batman: &mut Batman, enemy_on_left: bool, enemy_on_right: bool){
        if batman.direction() == Direction::Left && enemy_on_left || batman.direction() == Direction::Right && enemy_on_right {
            let knockback = batman.attack(&mut self.get_health());
            
            if knockback && self.get_action() != Action::Knockback {
                self.set_action(Action::Knockback);
                self.set_counter(0.0);
            }
        }
    }

    fn get_width(&mut self)->f32{
        self.get_current_sprite().width() as f32 * 3.2
    }
    
    fn get_precise_x_right(&mut self) -> f32{
        let rough_x = self.get_rough_x();
        let enemy_width = self.get_width();
        rough_x + enemy_width
    }

    fn get_precise_x_left(&mut self)->f32{
        self.get_rough_x()
    }

    fn get_healthbar_x(&self)->f32{
        let rough_x = self.get_rough_x();
        rough_x - 20.0
    }

    fn get_current_sprite(&mut self)-> &Image {
        let a = self.get_counter().round() as usize;
        let length = self.get_sprites().len();
        if a >= length { 
            self.end_conditions();
            self.reset_counter();
        }
        &self.get_sprites()[self.get_counter().round() as usize]
    }

    fn get_enemy_base(&mut self) -> EnemyBase;

    fn end_conditions(&mut self);


    fn reset_counter(&mut self);

    fn movement(&mut self, batman:&mut Batman);

    fn get_rough_x(&self)->f32;

    fn get_precise_x(&self)->f32;

    fn set_precise_x(&mut self, precise_x: f32);

    fn get_y(&self)->f32;

    fn set_y(&mut self, new_y: f32);

    fn get_counter(&self)->f32;

    fn set_counter(&mut self, new_counter: f32);

    fn get_action(&self)->Action;

    fn set_action(&mut self, new_action: Action);

    fn set_health(&mut self, new_health: f32);

    fn get_health(&self)->f32;

    fn get_dead(&self)->bool;

    fn set_dead(&mut self, is_dead: bool);

    fn get_attacking_sprites(&self)->&Vec<Image>;

    fn get_death(&self)->f32;

    fn get_sprites(&self) -> &Vec<Image>;

    fn is_alive(&self)-> bool;

    fn set_direction(&mut self, direction: Direction);

    fn get_sleep_direction(&self)-> Direction;

    fn get_drawn_y(&self) ->f32;



}








