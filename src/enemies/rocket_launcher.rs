use crate::batman::{Batarang, Batman};
use crate::projectiles::{Projectiles, Rocket};
use crate::sprites::get_sprites;
use ggez::{Context, GameResult};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Image, Mesh, Rect};
use crate::game_defs::{ GRAVITY, Direction, WIDTH };
use crate::enemies::enemy::{Enemy, Action};








pub struct RocketGuy {
    pub rough_x: f32,
    pub precise_x: f32,
    pub y: f32,
    pub action: Action,
    pub counter: f32,
    pub health: f32,
    pub death: f32,
    pub dead: bool,
    pub direction: Direction,
    pub sleep_direction: Direction,
    pub attack_counter: f32,
    pub can_spawn_rocket: bool,
    pub attacking_sprites: Vec<Image>,
    knockout_counter: f32,
    dying_sprites: Vec<Image>,
    standing_sprites: Vec<Image>,
}

impl RocketGuy {
    pub fn new(x: f32, y:f32, ctx: &mut Context) -> Self{
        let dying_sprites = get_sprites("Rocket Launcher/die", 4, "die", ctx);
        let attacking_sprites = get_sprites("Rocket Launcher/shoot", 10, "shoot", ctx);
        let standing_sprites = get_sprites("Rocket Launcher/sit", 1, "sit", ctx);


        Self{ 
            rough_x: x,
            precise_x: x,
            y,
            dying_sprites,
            attacking_sprites,
            standing_sprites,
            action: Action::Standing,
            knockout_counter: 0.0,
            counter: 0.0,
            health: 100.0,
            death: 100.0,
            dead: false,
            direction: Direction::Right,
            sleep_direction: Direction::Right,
            attack_counter: 0.0,
            can_spawn_rocket: true 
        }
    }

    pub fn add_rockets(&mut self, projectiles: &mut Vec<Projectiles>){
        if self.action == Action::Attacking && self.counter.round() == 1.0 && self.can_spawn_rocket{
            let x = if self.direction == Direction::Left{ self.precise_x + self.get_current_sprite().width() as f32} else {self.precise_x};
            let rocket = Rocket::new(x, self.y, self.direction);
            projectiles.push(Projectiles::Rocket(rocket));
            self.can_spawn_rocket = false;


        }
    }
}

impl Enemy for RocketGuy{

    fn movement(&mut self, batman:&mut Batman){
        let fixed_mid_point = self.get_mid_point() + 30.0;
        let enemy_on_left = fixed_mid_point < batman.get_mid_point() && batman.get_mid_point() - 150.0 < fixed_mid_point;
        let enemy_on_right = batman.get_mid_point() < fixed_mid_point && fixed_mid_point < batman.get_mid_point() + 150.0;

        

        if batman.direction() == Direction::Left && enemy_on_left || batman.direction() == Direction::Right && enemy_on_right {
            batman.attack(&mut self.health);
        }


        if self.health <= 0.0 {
            if self.action != Action::Dying {
                self.reset_counter();
            }
            self.action = Action::Dying;
        }
        else if self.attack_counter >= 10.0  {
            self.action = Action::Attacking;
        }
        else {
            self.action = Action::Standing;
            self.attack_counter += 0.05;
        }

        self.counter += 0.1;
    }


    fn get_sprites(&self) -> &Vec<Image>{
        match self.action {
            Action::Attacking => &self.attacking_sprites,
            Action::Standing  => &self.standing_sprites,
            Action::Dying     => &self.dying_sprites,
            _                 => &self.attacking_sprites
        }
    }

    fn end_conditions(&mut self) {
        match self.action {
            Action::Dying     => self.dead = true,
            Action::Attacking => {self.attack_counter = 0.0; self.can_spawn_rocket = true},
            _                 => {}
        }
    }


    fn get_healthbar_x(&self)->f32{
        let rough_x = self.get_rough_x();
        rough_x + 105.0

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

    fn get_precise_x_right(&mut self) -> f32 {
        let rough_x = self.get_rough_x();
        let width = self.get_width();
        rough_x + width + 50.0
    }

    fn get_knockout_counter(&self)-> f32 {
        self.knockout_counter
    }

    fn set_knockout_counter(&mut self, new_counter: f32) {
        self.knockout_counter = new_counter;
    }

}
