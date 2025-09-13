use crate::batman::{Batarang, Batman};
use crate::knife_guy;
use crate::sprites::get_sprites;
use ggez::{Context, GameResult};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Image, Mesh, Rect};
use crate::game_defs::{ GRAVITY, Direction };


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Action {
    Walking,
    Standing,
    Attacking,
    Dying,
    Knockout,
    Sleep
}




pub struct Enemy {
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
    dead: bool
}

impl Enemy {
    pub fn new(x: f32, y:f32, ctx: &mut Context) -> GameResult<Self>{
        let walking_sprites = get_sprites("Knife Guy/walking", 4, "walking", ctx)?;
        let standing_sprites = get_sprites("Knife Guy/standing", 1, "standing", ctx)?;
        let attacking_sprites = get_sprites("Knife Guy/stabbing", 5, "stab", ctx)?;
        let dying_sprites = get_sprites("Knife Guy/die", 6, "die", ctx)?;
        let sleeping_sprites = get_sprites("Knife Guy/sleep", 1, "sleep", ctx)?;


        Ok(Self{ rough_x: x, precise_x: x, y, walking_sprites, standing_sprites, attacking_sprites, dying_sprites, sleeping_sprites, action: Action::Standing, move_speed: 2.0, counter: 0.0, health: 100.0, death: 100.0, dead: false })
    }

    pub fn draw(& mut self, canvas:&mut  Canvas, ctx: &Context, player_x: f32, player_width: f32){
        let enemy_width = self.get_current_sprite().width() as f32 * 3.2;
        let y = self.y;

        
        let player_on_right = player_x + player_width/2.0 > self.rough_x + enemy_width/2.0;
        
        
        let direction = {
            if player_on_right { 
                self.precise_x = self.rough_x + enemy_width;
                -3.2
            } else {        
                self.precise_x = self.rough_x;
                if self.action == Action::Attacking && self.counter.round() >= 2.0 {
                    let diff = (self.attacking_sprites[2].width() - self.attacking_sprites[0].width()) as f32;
                    self.precise_x = self.precise_x - diff * 3.2;
                }
                3.2
            }
            
        };
        

        let x = self.precise_x;
        
        canvas.draw(self.get_current_sprite(), DrawParam::default()
            .dest([x, y])
            .scale([direction, 3.2]));


        let health = if self.health > 0.0 { self.health * 2.0 } else { 0.0 };
        let health_bar = Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(self.rough_x - 20.0, y -60.0, health, 20.0), Color::from_rgb(0, 255, 0));
        let death_bar = Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(self.rough_x - 20.0, y -60.0, self.death * 2.0, 20.0), Color::from_rgb(255, 0, 0));

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

        canvas.draw(&death_bar_res, DrawParam::default()
            .dest([0.0, 10.0])
        );

        canvas.draw(&health_bar_res, DrawParam::default()
            .dest([0.0, 10.0])
        );


    }

    pub fn update(& mut self, ground_height: f32, batman:&mut Batman, batarang_xs: &Vec<f32>){
        self.gravity(ground_height);
        self.check_for_batarang_damage(batarang_xs);
        self.movement(batman);
    }
        
    fn get_current_sprite(&mut self)-> &Image {
        let a = self.counter.round() as usize;
        let length = self.get_sprites().len() -1;
        if a >= length { 
            if self.action == Action::Dying{
                self.dead = true;
            }
            if self.action == Action::Knockout{
                self.action = Action::Sleep
            }
            if self.action == Action::Sleep {
                self.action = Action::Standing;
            }
            self.reset_counter();
        }
        &self.get_sprites()[self.counter.round() as usize]
    }

    fn get_sprites(&self) -> &Vec<Image>{
        match self.action {
            Action::Walking   => &self.walking_sprites,
            Action::Standing  => &self.standing_sprites,
            Action::Attacking => &self.attacking_sprites,
            Action::Dying     => &self.dying_sprites,
            Action::Knockout  => &self.dying_sprites,
            Action::Sleep     => &self.sleeping_sprites
        }
    }

    pub fn is_alive(&self)-> bool{
        !self.dead
    }

    fn reset_counter(&mut self){
        self.counter = 0.0;
    }

    fn gravity(&mut self, ground_height: f32){
        if (self.y + ((self.get_current_sprite().height() * 3)) as f32) < ground_height {
            self.y += GRAVITY;
        }
        else {
            self.y = ground_height - ((self.get_current_sprite().height() * 3)-35) as f32;
        }
    }

    fn check_for_batarang_damage(&mut self, batarang_xs: &Vec<f32>){
        for i in batarang_xs{
            let b_x = i.round();
            let x = self.rough_x.round();
            println!("batarang: {}, enemy: {}", b_x, x);
            if i.round() >= self.rough_x.round()-10.0 && i.round() <= self.rough_x.round()+10.0 {
                
                self.action = Action::Knockout;
                self.health -= 1.0;
                self.reset_counter();
            }
        }
    }

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

        
        
        println!("{:?}", self.action);



        if self.health <= 0.0{
            if self.action != Action::Dying {
                self.reset_counter();
            }
            self.action = Action::Dying;
        }
        else if self.action == Action::Knockout || self.action == Action::Sleep {
            if self.counter.round() as usize >= self.get_sprites().len()-1 {
                self.action = Action::Standing;
            }
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
    
}








