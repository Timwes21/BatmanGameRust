use crate::batman::{Batarang, Batman};
use crate::sprites::get_sprites;
use ggez::{Context, GameResult};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Image, Mesh, Rect};
use crate::game_defs::{ GRAVITY, Direction };
use crate::enemies::enemy::{Enemy, Action};







pub struct KnifeGuy {
    pub rough_x: f32,
    pub precise_x: f32,
    pub y: f32,
    pub action: Action,
    pub move_speed: f32,
    pub counter: f32,
    pub health: f32,
    pub death: f32,
    pub dead: bool,
    pub direction: Direction,
    pub sleep_direction: Direction,
    walking_sprites: Vec<Image>,
    standing_sprites: Vec<Image>,
    pub attacking_sprites: Vec<Image>,
    dying_sprites: Vec<Image>,
    sleeping_sprites: Vec<Image>,
    knockout_counter: f32
}

impl KnifeGuy {
    pub fn new(x: f32, y:f32, move_speed: f32, ctx: &mut Context) -> Self{
        let walking_sprites   = get_sprites("Knife Guy/walking", 4, "walking", ctx);
        let standing_sprites  = get_sprites("Knife Guy/standing", 1, "standing", ctx);
        let attacking_sprites = get_sprites("Knife Guy/stabbing", 5, "stab", ctx);
        let dying_sprites     = get_sprites("Knife Guy/die", 4, "die", ctx);
        let sleeping_sprites  = get_sprites("Knife Guy/sleep", 1, "sleep", ctx);


        Self{ 
            rough_x: x, 
            precise_x: x, 
            y, 
            walking_sprites, 
            standing_sprites, 
            attacking_sprites, 
            dying_sprites, 
            sleeping_sprites, 
            knockout_counter: 0.0,
            action: Action::Standing, 
            move_speed, 
            counter: 0.0, 
            health: 100.0, 
            death: 100.0, 
            dead: false, 
            direction: Direction::Right, 
            sleep_direction: Direction::Right 
        }
    }    
}

impl Enemy for KnifeGuy{

    fn movement(&mut self, batman:&mut Batman){
        let player_x = batman.get_x();
        let player_width = batman.get_current_sprite().width() as f32 * 3.2;


        let detection_range = 600.0;


        let enemys_right_side = self.rough_x + self.get_current_sprite().width() as f32 * 3.2;
        let enemys_left_side = self.rough_x;

        let player_left = player_x + 50.0;
        let player_right = player_x + player_width - 50.0;


        let enemy_in_detection_range_left = enemys_right_side < player_left && enemys_right_side > player_left - detection_range;
        let enemy_in_detection_range_right = enemys_left_side > player_right && enemys_left_side < player_right + detection_range;

        let out_of_range_left = enemys_right_side < player_left - detection_range;
        let out_of_range_right = enemys_left_side > player_right + detection_range;

        
        

        if self.health <= 0.0 {
            if self.action != Action::Dying {
                self.reset_counter();
            }
            self.action = Action::Dying;
        }
        else if self.action == Action::Knockout || self.action == Action::Sleep {
            self.knockout(20.0);
        }
        else if enemy_in_detection_range_left || enemy_in_detection_range_right {
            self.rough_x += if enemy_in_detection_range_left { self.move_speed } else { -self.move_speed };
            self.action = Action::Walking;
        }
        else if out_of_range_left || out_of_range_right {
            self.action = Action::Standing;
        }
        else if batman.is_grounded() {
            self.action = Action::Attacking;
            if self.rough_x < batman.get_mid_point() && batman.direction() == Direction::Left{
                batman.attack(&mut self.health);
            }
            else if self.rough_x > batman.get_mid_point() && batman.direction() == Direction::Right{
                batman.attack(&mut self.health);
            } 
        }
        else {
            self.action = Action::Standing;
        }
        
        if self.action == Action::Attacking && self.counter.round() as usize == 2{
            batman.take_damage(0.1);
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
            _                 => &self.standing_sprites
        }
    }

    fn end_conditions(&mut self) {
        match self.action {
            Action::Dying => self.dead = true,
            Action::Knockout  => { self.set_action(Action::Sleep); self.sleep_direction = self.direction; },
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
        self.get_y()
    }

    fn get_knockout_counter(&self)-> f32 {
        self.knockout_counter
    }

    fn set_knockout_counter(&mut self, new_counter: f32) {
        self.knockout_counter = new_counter;
    }


}








