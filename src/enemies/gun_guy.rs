use crate::batman::{Batarang, Batman};
use crate::sprites::get_sprites;
use ggez::{Context, GameResult};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Image, Mesh, MeshBuilder, Rect};
use crate::game_defs::{ GRAVITY, Direction };
use crate::enemies::enemy::{Enemy, Action};
use crate::game_defs::{WIDTH, EnemyReaction};
use crate::projectiles::{self, Projectiles, Bullet};



pub struct GunGuy {
    pub rough_x: f32,
    pub precise_x: f32,
    pub y: f32,
    pub action: Action,
    pub move_speed: f32,
    pub counter: f32,
    pub health: f32,
    pub death: f32,
    pub dead: bool,
    pub shooting_interval: f32,
    pub direction: Direction,
    pub sleep_direction: Direction,
    pub knockout_counter: f32,
    walking_sprites: Vec<Image>,
    standing_sprites: Vec<Image>,
    pub attacking_sprites: Vec<Image>,
    dying_sprites: Vec<Image>,
    sleeping_sprites: Vec<Image>,
    knockback_sprites: Vec<Image>,
}

impl GunGuy {
    pub fn new(x: f32, y:f32, move_speed: f32, ctx: &mut Context) -> Self{
        let walking_sprites   = get_sprites("Gun Guy/walking", 4, "walking", ctx);
        let standing_sprites  = get_sprites("Gun Guy/standing", 1, "standing", ctx);
        let attacking_sprites = get_sprites("Gun Guy/shooting", 3, "shooting", ctx);
        let dying_sprites     = get_sprites("Gun Guy/die", 4, "die", ctx);
        let sleeping_sprites  = get_sprites("Gun Guy/sleep", 1, "sleep", ctx);
        let knockback_sprites = get_sprites("Gun Guy/knockback", 3, "knockback", ctx);


        Self{ 
            rough_x: x, 
            precise_x: x,
            y,
            walking_sprites,
            standing_sprites,
            attacking_sprites,
            dying_sprites,
            sleeping_sprites,
            knockback_sprites,
            action: Action::Standing,
            move_speed,
            counter: 0.0,
            health: 100.0,
            death: 100.0,
            dead: false,
            shooting_interval: 0.0,
            direction: Direction::Left,
            knockout_counter: 0.0,
            sleep_direction: Direction::Left
        }
    }    

    pub fn add_bullets(&mut self, projectiles: &mut Vec<Projectiles>){
        if self.action == Action::Attacking && self.counter.round() == 1.0{
            let x = if self.direction == Direction::Left{ self.precise_x + self.get_current_sprite().width() as f32} else {self.precise_x};
            let bullet = Bullet::new(x, self.y, self.direction);
            
            projectiles.push(Projectiles::Bullet(bullet));


        }
    }
}


impl Enemy for GunGuy{
    fn movement(&mut self, batman:&mut Batman){
        let player_x = batman.get_x();
        let player_width = batman.get_current_sprite().width() as f32 * 3.2;

        let detection_range = 600.0;


        let enemys_right_side = self.rough_x + self.get_current_sprite().width() as f32 * 3.2;
        let enemys_left_side = self.rough_x;

        let player_left = player_x + 50.0;
        let player_right = player_x + player_width - 50.0;


        let outside_of_shoooting_range_left = enemys_right_side < player_left - detection_range;
        let outside_of_shoooting_range_right = enemys_left_side > player_right + detection_range;


        let enemy_on_left = self.get_mid_point() < batman.get_mid_point() && batman.get_mid_point() - 150.0 < self.get_mid_point();
        let enemy_on_right = batman.get_mid_point() < self.get_mid_point() && self.get_mid_point() < batman.get_mid_point() + 150.0;
        
        
        if batman.direction() == Direction::Left && enemy_on_left || batman.direction() == Direction::Right && enemy_on_right {
            let hit_reaction = batman.attack(&mut self.health);


            match hit_reaction {
                EnemyReaction::Knockback => {
                    if self.action != Action::Knockback{
                        self.action = Action::Knockback;
                        self.counter = 0.0;
                    }
                },

                _ => {}


            }
        }
        



        if self.health <= 0.0{
            if self.action != Action::Dying {
                self.reset_counter();
            }
            self.action = Action::Dying;
        }
        else if matches!(self.action, Action::Knockout | Action::Sleep | Action::Knockback) {
            if self.action == Action::Sleep{
                self.knockout_counter += 0.2;
                if self.knockout_counter >= 20.0 {
                    self.action = Action::Standing;
                    self.knockout_counter = 0.0;
                }
            }

        }
        else if self.shooting_interval.round() >= 20.0{
            self.action = Action::Attacking;
        }
        else if outside_of_shoooting_range_left {
            self.rough_x += self.move_speed;
            self.action = Action::Walking;
            self.direction = Direction::Right;
        }
        else if outside_of_shoooting_range_right {
            self.rough_x -= self.move_speed;
            self.action = Action::Walking;
            self.direction = Direction::Left;
        }
        else {
            self.action = Action::Standing;
            self.shooting_interval += 0.1;
        }
        self.counter += 0.1;
    
    }

    fn get_sprites(&self) -> &Vec<Image>{
        match self.action {
            Action::Walking   => &self.walking_sprites,
            Action::Standing  => &self.standing_sprites,
            Action::Attacking => &self.attacking_sprites,
            Action::Dying     => &self.dying_sprites,
            Action::Knockout  => &self.dying_sprites,
            Action::Sleep     => &self.sleeping_sprites,
            Action::Knockback => &self.knockback_sprites,
            _                 => &self.standing_sprites
        }
    }

    
    fn end_conditions(&mut self) {
        match self.action {
            Action::Dying     => { self.set_dead(true); },
            Action::Knockout  => { self.set_action(Action::Sleep); self.sleep_direction = self.direction; },
            Action::Attacking => { self.action = Action::Standing; self.shooting_interval = 0.0; },
            Action::Knockback => { self.action = Action::Standing; println!("changing"); },
            _ => {}
        }
    }


    fn reset_counter(&mut self){
        self.counter = 0.0;
    }

    fn get_rough_x(&self)->f32{
        self.rough_x
    }

    fn get_precise_x(&self)->f32{
        self.precise_x
    }

    fn set_precise_x(&mut self, precise_x: f32){
        self.precise_x = precise_x;
    }

    fn get_y(&self)->f32{
        self.y
    }

    fn set_y(&mut self, new_y: f32){
        self.y = new_y;
    }

    fn get_counter(&self)->f32{
        self.counter
    }

    fn set_counter(&mut self, new_counter: f32){
        self.counter = new_counter;
    }

    fn get_action(&self)->Action{
        self.action
    }

    fn set_action(&mut self, new_action: Action){
        self.action = new_action;
    }

    fn set_health(&mut self, new_health: f32){
        self.health = new_health;
    }

    fn get_health(&self)->f32{
        self.health
    }

    fn get_dead(&self)->bool{
        self.dead
    }

    fn set_dead(&mut self, is_dead: bool){
        self.dead = is_dead;
    }

    fn get_attacking_sprites(&self)->&Vec<Image>{
        &self.attacking_sprites
    }

    fn get_death(&self)->f32{
        self.death
    }

    fn is_alive(&self)-> bool{
        !self.dead
    }

    fn set_direction(&mut self, direction: Direction){
        self.direction = direction;
    }

    fn get_sleep_direction(&self)-> Direction{
        self.sleep_direction
    }

    fn get_drawn_y(&self) ->f32{
        self.get_y() + 25.0
    }

    fn get_knockout_counter(&self)-> f32 {
        self.knockout_counter
    }

    fn set_knockout_counter(&mut self, new_counter: f32) {
        self.knockout_counter = new_counter;
    }


}


