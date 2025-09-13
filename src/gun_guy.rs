use crate::batman::{Batarang, Batman};
use crate::gun_guy;
use crate::sprites::get_sprites;
use ggez::{Context, GameResult};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Image, Mesh, MeshBuilder, Rect};
use crate::game_defs::{ GRAVITY, Direction };
use crate::enemy::{Enemy, Action};
use crate::game_defs::WIDTH;



pub struct GunGuy {
    rough_x: f32,
    precise_x: f32,
    y: f32,
    walking_sprites: Vec<Image>,
    standing_sprites: Vec<Image>,
    attacking_sprites: Vec<Image>,
    dying_sprites: Vec<Image>,
    sleeping_sprites: Vec<Image>,
    action: Action,
    move_speed: f32,
    counter: f32,
    health: f32,
    death: f32,
    dead: bool,
    shooting_interval: f32,
    direction: Direction,
    knockout_counter: f32,
    sleep_direction: Direction
}

impl GunGuy {
    pub fn new(x: f32, y:f32, ctx: &mut Context) -> GameResult<Self>{
        let walking_sprites = get_sprites("Gun Guy/walking", 4, "walking", ctx)?;
        let standing_sprites = get_sprites("Gun Guy/standing", 1, "standing", ctx)?;
        let attacking_sprites = get_sprites("Gun Guy/shooting", 3, "shooting", ctx)?;
        let dying_sprites = get_sprites("Gun Guy/die", 4, "die", ctx)?;
        let sleeping_sprites = get_sprites("Gun Guy/sleep", 1, "sleep", ctx)?;


        Ok(Self{ rough_x: x, precise_x: x, y, walking_sprites, standing_sprites, attacking_sprites, dying_sprites, sleeping_sprites, action: Action::Standing, move_speed: 2.0, counter: 0.0, health: 100.0, death: 100.0, dead: false, shooting_interval: 0.0, direction: Direction::Left, knockout_counter: 0.0, sleep_direction: Direction::Left })
    }    

    pub fn add_bullets(&mut self, bullets: &mut Vec<Bullet>){
        if self.action == Action::Attacking && self.counter.round() == 1.0{
            let x = if self.direction == Direction::Left{ self.precise_x + self.get_current_sprite().width() as f32} else {self.precise_x};
            let bullet_res = Bullet::new(x, self.y, self.direction);
            
            let bullet = match bullet_res {
                Ok(m)=> m,
                Err(e)=>{println!("{}", e); return;}
            };
            
            bullets.push(bullet);


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
        
        
        if batman.direction() == Direction::Left && enemy_on_left  {
            batman.attack(&mut self.health);
        }
        if batman.direction() == Direction::Right && enemy_on_right{
            batman.attack(&mut self.health);
        }
        



        if self.health <= 0.0{
            if self.action != Action::Dying {
                self.reset_counter();
            }
            self.action = Action::Dying;
        }
        else if self.action == Action::Knockout || self.action == Action::Sleep {
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
            _                 => &self.standing_sprites
        }
    }

    fn get_current_sprite(&mut self)-> &Image {
        let a = self.get_counter().round() as usize;
        let length = self.get_sprites().len() -1;
        if a >= length { 
            let action = self.get_action();
            match action {
                Action::Dying     => { self.set_dead(true); },
                Action::Knockout  => { self.set_action(Action::Sleep); self.sleep_direction = self.direction; },
                Action::Attacking => { self.action = Action::Standing; self.shooting_interval = 0.0; }
                _ => {}
            }
            self.reset_counter();
        }
        &self.get_sprites()[self.get_counter().round() as usize]
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

}




pub struct Bullet{
    x: f32,
    y: f32,
    direction: Direction,
    move_speed: f32
}


impl Bullet {
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
            batman.take_damage(3.0);
            true
        }
        else {
            false
        }
    }
}