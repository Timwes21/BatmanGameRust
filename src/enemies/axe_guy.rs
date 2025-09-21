use crate::batman::{Batarang, Batman};
use crate::enemies::Enemies;
use crate::projectiles::{self, Projectile};
use crate::sprites::get_sprites;
use ggez::{Context, GameResult};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Image, Mesh, Rect};
use crate::game_defs::{ GRAVITY, WIDTH, Direction };
use crate::enemies::enemy::{Enemy, Action, EnemyBase};







pub struct AxeGuy {
    pub enemy_base: EnemyBase,
    pub throwing_counter: f32,
    pub can_spawn_axe: bool,
    pub attacking_sprites: Vec<Image>,
    walking_sprites: Vec<Image>,
    standing_sprites: Vec<Image>,
    throwing_sprites: Vec<Image>,
    dying_sprites: Vec<Image>,
    sleeping_sprites: Vec<Image>,
    knockback_sprites: Vec<Image>,
}

impl AxeGuy {
    pub fn new(x: f32, y:f32, ctx: &mut Context) -> Self{
        let walking_sprites   = get_sprites("Axe Guy/walking", 6, "walking", ctx);
        let standing_sprites  = get_sprites("Axe Guy/standing", 1, "standing", ctx);
        let attacking_sprites = get_sprites("Axe Guy/swing", 4, "swing", ctx);
        let throwing_sprites  = get_sprites("Axe Guy/throw", 5, "throw", ctx);
        let dying_sprites     = get_sprites("Axe Guy/die", 3, "die", ctx);
        let sleeping_sprites  = get_sprites("Axe Guy/sleep", 1, "sleep", ctx);
        let knockback_sprites = get_sprites("Axe Guy/knockback", 1, "knockback", ctx);


        Self{ 
            rough_x: x, 
            precise_x: x, 
            y, 
            action: Action::Standing, 
            move_speed: 2.0, 
            counter: 0.0, 
            health: 100.0, 
            death: 100.0, 
            dead: false, 
            direction: Direction::Right, 
            sleep_direction: Direction::Right, 
            knockout_counter: 0.0, 
            throwing_counter: 0.0, 
            can_spawn_axe: false, 
            walking_sprites, 
            standing_sprites, 
            attacking_sprites, 
            throwing_sprites, 
            dying_sprites, 
            sleeping_sprites, 
            knockback_sprites, 
        }
    }    

    pub fn add_axes(&mut self, projectiles: &mut Vec<Projectile>){
        if self.action == Action::Throwing && self.counter.round() == 1.0 && self.can_spawn_axe{
            let x = if self.direction == Direction::Left{ self.precise_x + self.get_current_sprite().width() as f32} else {self.precise_x};
            let axe_res = Axe::new(x, self.y, self.direction);
            
            let axe = match axe_res {
                Ok(m)=> m,
                Err(e)=>{println!("{}", e); return;}
            };
            
            projectiles.push(Projectile::Axe(axe));
            self.can_spawn_axe = false;


        }
    }
}

impl Enemy for AxeGuy{
    fn movement(&mut self, batman:&mut Batman){
        let player_x = batman.get_x();
        let player_width = batman.get_current_sprite().width() as f32 * 3.2;
        

        let detection_range = 600.0;


        let enemys_right_side = self.rough_x + self.get_width();
        let enemys_left_side = self.rough_x;

        let player_left = player_x + 50.0;
        let player_right = player_x + player_width - 50.0;


        let enemy_in_detection_range_left = enemys_right_side < player_left && enemys_right_side > player_left - detection_range;
        let enemy_in_detection_range_right = enemys_left_side > player_right && enemys_left_side < player_right + detection_range;

        let out_of_range_left = enemys_right_side < player_left - detection_range;
        let out_of_range_right = enemys_left_side > player_right + detection_range;

        let enemy_on_left = self.get_mid_point() < batman.get_mid_point() && batman.get_mid_point() - 150.0 < self.get_mid_point();
        let enemy_on_right = batman.get_mid_point() < self.get_mid_point() && self.get_mid_point() < batman.get_mid_point() + 150.0;

        if batman.direction() == Direction::Left && enemy_on_left || batman.direction() == Direction::Right && enemy_on_right {
            batman.attack(&mut self.health);
        }
        

        if self.health <= 0.0 {
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
        else if enemy_in_detection_range_left || enemy_in_detection_range_right {
            self.rough_x += if enemy_in_detection_range_left { self.move_speed } else { -self.move_speed };
            self.action = Action::Walking;
        }
        else if out_of_range_left || out_of_range_right {
            self.throwing_counter += 0.2;
            if self.throwing_counter.round() >= 20.0 {
                self.action = Action::Throwing;
            }
            else {
                self.action = Action::Standing;
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
            Action::Attacking => &self.attacking_sprites,
            Action::Dying | 
            Action::Knockout  => &self.dying_sprites,
            Action::Sleep     => &self.sleeping_sprites,
            Action::Throwing  => &self.throwing_sprites,
            _                 => &self.standing_sprites
        }
    }

    fn end_conditions(&mut self) {
        match self.action {
            Action::Dying    => self.dead = true,
            Action::Knockout => { self.set_action(Action::Sleep); self.sleep_direction = self.direction; },
            Action::Throwing => { self.throwing_counter = 0.0; self.can_spawn_axe = true },
            _                => {}
        }
    }

    fn get_enemy_base(&mut self) -> EnemyBase {
        
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


}


pub struct Axe{
    x: f32,
    y: f32,
    direction: Direction,
    move_speed: f32
}


impl Axe {
    fn new(x:f32, y:f32, direction: Direction)-> GameResult<Self>{
        let adjusted_y = y + 30.0;
        Ok(Self { x, y: adjusted_y, direction, move_speed: 10.0 })
    }

    pub fn update(&mut self){
        self.x += if self.direction == Direction::Left { -self.move_speed } else { self.move_speed };
    }

    pub fn draw(& mut self, canvas:&mut  Canvas, ctx: &Context)-> GameResult{
        let bullet = Mesh::new_circle(ctx, DrawMode::fill(), [self.x, self.y], 15.0, 0.1, Color::from_rgb(0, 0, 0))?;

        canvas.draw(&bullet, DrawParam::default()
            .dest([0.0, 10.0])
        );

        Ok(())
    }

    pub fn is_offscreen(&self) -> bool{
        if self.x > WIDTH || self.x < 0.0{ true } else { false }
    }

    pub fn hit_batman(&self, batman: &mut Batman)-> bool{
        let batman_mid_point = batman.get_mid_point().round();
        let bullet_x = self.x.round();
        if bullet_x >= batman_mid_point - 20.0 && batman_mid_point + 20.0 >= bullet_x && batman.is_grounded(){
            batman.take_damage(0.8);
            true
        }
        else {
            false
        }
    }
}





