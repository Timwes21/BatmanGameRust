use ggez::Context;
use ggez::graphics::{self, Canvas, Color, DrawMode, DrawParam, Image, Mesh, Rect};
use ggez::input::keyboard::{self, KeyCode};

use crate::sprites::get_sprites;
use crate::game_defs::{ WIDTH, HEIGHT, GRAVITY, SCALE, Direction, EnemyReaction};
use crate::ui_draw::UIDraw;




#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Action {
    Running,
    Jumping,
    Standing,
    Punching1,
    Punching2,
    Punching3,
    Kicking1,
    Kicking2,
    Throwing,
    SuperPunch,
    Stun
}



pub struct Batman {
    x: f32,
    y: f32,
    move_speed: f32,
    jumping: bool,
    running_sprites: Vec<Image>,
    standing_sprites: Vec<Image>,
    jumping_sprites: Vec<Image>,
    punching1_sprites: Vec<Image>,
    punching2_sprites: Vec<Image>,
    punching3_sprites: Vec<Image>,
    kicking1_sprites: Vec<Image>,
    kicking2_sprites: Vec<Image>,
    throwing_sprites: Vec<Image>,
    super_punch_sprites: Vec<Image>,
    stun_sprites: Vec<Image>,
    counter: f32,
    action: Action,
    direction: Direction,
    falling: bool,
    incr: f32,
    jump_speed: f32,
    punching: bool,
    health: f32,
    death: f32,
    can_spawn_batarang: bool,
    stun_cooldown: f32
}


impl Batman {
    pub fn new(x: f32, y: f32, move_speed: f32, ctx: &mut Context, health: f32) -> Self{

        let running_sprites:      Vec<Image> = get_sprites("Batman running", 22, "running", ctx);
        let standing_sprites:     Vec<Image> = get_sprites("Batman standing", 16, "standing", ctx);
        let jumping_sprites:      Vec<Image> = get_sprites("Batman jumping", 7, "jump", ctx);
        let punching1_sprites:    Vec<Image> = get_sprites("Batman punching1", 11, "punch", ctx);
        let punching2_sprites:    Vec<Image> = get_sprites("Batman punching2", 10, "punch", ctx);
        let punching3_sprites:    Vec<Image> = get_sprites("Batman punching3", 10, "punch", ctx);
        let kicking1_sprites:     Vec<Image> = get_sprites("Batman kicking1", 9, "kick", ctx);
        let kicking2_sprites:     Vec<Image> = get_sprites("Batman kicking2", 10, "kick", ctx);
        let throwing_sprites:     Vec<Image> = get_sprites("Batman Throwing", 19, "throw", ctx);
        let stun_sprites:         Vec<Image> = get_sprites("Batman stun", 13, "stun", ctx);
        let super_punch_sprites:  Vec<Image> = get_sprites("Batman super punch", 17, "super_punch", ctx);
        
        let batman = Self{ 
            x, 
            y, 
            move_speed, 
            jumping: false, 
            running_sprites, 
            standing_sprites, 
            jumping_sprites, 
            punching1_sprites,
            punching2_sprites,
            punching3_sprites,
            kicking1_sprites,
            kicking2_sprites,
            throwing_sprites,
            super_punch_sprites,
            stun_sprites,
            counter: 0.0, 
            action: Action::Standing, 
            direction: Direction::Left, 
            falling: false, 
            incr: 0.3, 
            jump_speed: 50.0,
            punching: false,
            health,
            death: health,
            can_spawn_batarang: true,
            stun_cooldown: 0.0
        }; 
        batman 
    }

    pub fn draw(& mut self, canvas:&mut  Canvas, ctx: &mut Context){
        let x = self.x;
        let y = self.y;

        let width = self.get_current_sprite().width() as f32 * 3.2;
        let move_left = x;
        let move_right = x + width;


        let (direction, x) = if matches!(self.action, Action::Running | 
                                                                Action::Throwing | 
                                                                Action::Stun | 
                                                                Action::SuperPunch) {
            match self.direction {
                Direction::Left  => (3.4, move_left),
                Direction::Right => (-3.4, move_right),
                _       => (3.4, x)
            }
        }
        else {
            match self.direction {
                Direction::Right => (3.4, move_left),
                Direction::Left  => (-3.4, move_right),
                _       => (3.4, x)
            }
        };
        canvas.draw(self.get_current_sprite(), DrawParam::default()
            .dest([x, y])
            .scale([direction, 3.4]));

        let health = &self.health * 3.0;
        let death = &self.death * 3.0;

        let mut health_bar = UIDraw::make_mesh(ctx, 0.0, 0.0, health, 60.0, Color::from_rgb(0, 255, 0));
        health_bar.draw(canvas, [0.0, 10.0], [0.0, 10.0]);
        
        let death_bar_res = Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(0.0, 0.0, death, 60.0), Color::from_rgb(255, 0, 0));


        let death_bar = match death_bar_res {
            Ok(m) => m,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };



        canvas.draw(&death_bar, DrawParam::default()
            .dest([0.0, 10.0])
        );
        // canvas.draw(&health_bar, DrawParam::default()
        //     .dest([0.0, 10.0])
        // );

        if self.stun_cooldown > 0.0{
            self.stun_cooldown -= 0.05;

            let stun_cooldown_bar_height = 100.0;
            let stun_cooldown_bar_x = 20.0;
            let stun_cooldown_bar_y = HEIGHT - stun_cooldown_bar_height;
            
            let stun_colldown_representaion = Mesh::new_rectangle(
                                                    ctx, DrawMode::fill(), 
                                                    Rect::new(stun_cooldown_bar_x, stun_cooldown_bar_y, 
                                                    self.stun_cooldown * 10.0, stun_cooldown_bar_height), 
                                                    Color::from_rgb(255, 0, 0));
            let stun_cooldown_bar = match stun_colldown_representaion {
                Ok(m) => m,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };
            let cooldown_text = graphics::Text::new("Stun Cooldown".to_string());
            
            canvas.draw(&cooldown_text, DrawParam::default()
                .dest([stun_cooldown_bar_x, stun_cooldown_bar_y - 30.0])
                .scale([3.0, 3.0]));
        
            canvas.draw(&stun_cooldown_bar, DrawParam::default()
                .dest([0.0, 10.0])
            );
        }




    }

    pub fn update(& mut self, ctx: &mut Context, ground_height: f32, batarangs: & mut Vec<Batarang>){
        let sprite_height = self.get_current_sprite().height();
        let sprite_width = self.get_current_sprite().width();
        
        if !self.falling && !self.jumping{
            self.y = ground_height -((sprite_height * 3)-25) as f32;
        }
        else if (self.y + ((sprite_height * 3)-5) as f32) < ground_height {
            self.y += GRAVITY;
            self.falling = true;

        }
        else {
            self.falling = false;
            self.jumping = false;
            self.jump_speed = 50.0;
        }



        
        if keyboard::is_key_pressed(ctx, KeyCode::Up) && (self.y + ((sprite_height * 3)-6) as f32) >= ground_height { 
            self.jumping = true;
            if self.action != Action::Jumping {
                self.counter = 0.0;
            }
            self.action = Action::Jumping;
            self.incr = 0.13;
        }
        else if keyboard::is_key_pressed(ctx, KeyCode::Left) && self.x != 0.0 && self.x != WIDTH  { 
            self.x -= self.move_speed;
            if self.is_grounded() {
                self.action = Action::Running;
                self.incr = 0.35;
            }
            if self.action == Action::Running && self.counter.round() as usize == self.get_sprites().len()-1{
                self.counter = 3.0;
            }
            self.direction = Direction::Left;
        }
        else if keyboard::is_key_pressed(ctx, KeyCode::Right) && self.x != WIDTH - ((sprite_width as f32) * 3.0) { 
            self.x += self.move_speed;
            if self.is_grounded(){
                self.action = Action::Running;
                self.incr = 0.3;
            }
            if self.action == Action::Running && self.counter.round() as usize >= self.get_sprites().len()-1{
                self.counter = 3.0;
            }
            self.direction = Direction::Right;
        }
        else if keyboard::is_key_pressed(ctx, KeyCode::A){
            if self.action == Action::Punching1 && self.counter.round() as usize >= self.get_sprites().len()-5 {
                self.action = Action::Punching2;
                self.counter = 0.0;
                self.incr = 0.25;
            }
            else if matches!(self.action, Action::Punching2 | Action:: Kicking2) && self.counter.round() as usize >= self.get_sprites().len()-6 {
                self.action = Action::Punching3;
                self.counter = 0.0;
                self.incr = 0.25;
            }
            else if !matches!(self.action, Action::Punching1 | Action:: Punching2 | Action::Punching3) {
                self.action = Action::Punching1;
                self.counter = 0.0;
                self.incr = 0.25;
            }

            self.punching = true;
        }
        else if keyboard::is_key_pressed(ctx, KeyCode::D){
            if self.action != Action::SuperPunch{
                self.counter = 0.0;
                self.incr = 0.27;
                self.action = Action::SuperPunch;
            }
            self.punching = true;
        }
        else if keyboard::is_key_pressed(ctx, KeyCode::S){
            if self.action == Action::Kicking1 && self.counter.round() as usize >= self.get_sprites().len()-5 {
                self.action = Action::Kicking2;
                self.counter = 0.0;
            }
            else if !matches!(self.action, Action::Kicking1 | Action::Kicking2) {
                self.action = Action::Kicking1;
                self.counter = 0.0;
            }

            self.punching = true;
            self.incr = 0.2;
        }
        else if keyboard::is_key_pressed(ctx, KeyCode::Q){
            if self.action != Action::Throwing {
                self.action = Action::Throwing;
                self.counter = 0.0;
            }
            self.incr = 0.2;
        }
        else if self.action == Action::Throwing {
            if self.counter.round() as usize >= 3 && self.counter.round() as usize <= self.get_sprites().len() -6{
                self.x += if self.direction == Direction::Left { -self.move_speed * 0.13 } else { self.move_speed * 0.13};
            }

            if self.counter.round() as usize == 5 && self.can_spawn_batarang{
                let x = if self.direction == Direction::Left {self.x +30.0} else {self.x + self.get_width() - 30.0};
                let batarang = Batarang::new(x, self.y, self.direction, ctx);

                batarangs.push(batarang);
                self.can_spawn_batarang = false;
            }
        }
        else if keyboard::is_key_pressed(ctx, KeyCode::W) && self.stun_cooldown <= 0.0{
            if self.action != Action::Stun {
                self.counter = 0.0;
            }
            self.action = Action::Stun;
        }
        else if self.action == Action::Stun || self.punching{
            // to catch if still active
        }
        else { 
            if self.is_grounded() {
                self.action = Action::Standing;
                self.incr = 0.2;
            }
        }

        if self.punching && self.counter.round() as usize >= self.get_sprites().len()-1{
            self.punching = false;
        }



        if self.action != Action::Throwing {
            self.can_spawn_batarang = true;
        }

        self.counter += self.incr;
        self.jump();
    }
    

    pub fn get_current_sprite(&mut self)-> &Image {        
        let a = self.counter.round() as usize;
        let length = self.get_sprites().len();
        if a >= length {
            if self.action == Action::Throwing{
                self.action = Action::Standing;
                self.can_spawn_batarang = true;
            }
            if self.action == Action::Stun{
                self.stun_cooldown = 30.0;
                self.action = Action::Standing
            }
            if self.action != Action::Running{
                self.counter = 0.0;
            }
        }

        &self.get_sprites()[self.counter.round() as usize]
    }
    

    fn get_sprites(&self) -> &Vec<Image>{
        match self.action {
            Action::Running    => &self.running_sprites,
            Action::Jumping    => &self.jumping_sprites,
            Action::Standing   => &self.standing_sprites,
            Action::Punching1  => &self.punching1_sprites,
            Action::Punching2  => &self.punching2_sprites,
            Action::Punching3  => &self.punching3_sprites,
            Action::Kicking1   => &self.kicking1_sprites,
            Action::Kicking2   => &self.kicking2_sprites,
            Action::Throwing   => &self.throwing_sprites,
            Action::SuperPunch => &self.super_punch_sprites,
            Action::Stun       => &self.stun_sprites
        }
    }
    
    fn jump(&mut self){
        if self.jumping {
            self.y -= self.jump_speed;
            if self.jump_speed > 1.0 {
                self.jump_speed -= 1.0;
            }
        }
    }

    pub fn get_x(&self)-> f32{
        self.x
    }

    pub fn get_width(&mut self) -> f32{

        self.get_current_sprite().width() as f32 * 3.2
    }

    pub fn take_damage(&mut self, damage: f32){
        self.health -= damage;
    }

    pub fn is_grounded(&self)->bool{
        !self.jumping && !self.falling
    }

    pub fn get_mid_point(&mut self) -> f32{
        let width = self.get_current_sprite().width() as f32;
        let half_width = (width * 3.4)/2.0;
        self.x + half_width

    }

    pub fn attack(&mut self, enemy_health:&mut f32)->EnemyReaction{
        let hit_damage = 30.0;
        let kick_damage = 50.0;
        if self.action == Action::Punching1 && self.counter.round() as usize == 2 {
            *enemy_health -= hit_damage;
            return EnemyReaction::Unfazed;
        }
        if self.action == Action::Punching2 && self.counter.round() as usize == 2 {
            *enemy_health -= hit_damage;
            return EnemyReaction::Unfazed;
        }
        if self.action == Action::Punching3 && self.counter.round() as usize == 2 {
            *enemy_health -= kick_damage;
            return EnemyReaction::Unfazed;
        }
        if self.action == Action::Kicking1 && self.counter.round() as usize == 2{
            *enemy_health -= kick_damage;
            return EnemyReaction::Unfazed;
        }
        if self.action == Action::Kicking2 && self.counter.round() as usize == 2{
            *enemy_health -= kick_damage;
            return EnemyReaction::Unfazed;
        }
        if self.action == Action::SuperPunch && self.counter.round() as usize == 2{
            *enemy_health -= kick_damage;
            return EnemyReaction::Unfazed;
        }
        EnemyReaction::Unfazed

    }

    pub fn stun_grenade(&mut self)-> EnemyReaction{
        if self.action == Action::Stun && self.counter.round() as usize == 4{
            return EnemyReaction::GrenadeKnockout;
        }
        EnemyReaction::Unfazed
    }
    


    pub fn direction(&self)-> Direction{
        self.direction
    }


    pub fn is_alive(&self)->bool{
        self.health > 0.0
    }

    pub fn is_attacking(&self)-> bool{
        matches!(self.action, Action::Kicking1 | Action::Punching1 | Action::Punching2 | Action::Punching3 | Action::Kicking2)
    }

    pub fn reset_health(&mut self){
        self.health = self.death;
    }

}




pub struct Batarang {
    x: f32,
    y: f32,
    counter: f32,
    move_speed: f32,
    sprites: Vec<Image>,
    direction: Direction,
    scale: f32
}


impl Batarang {
    fn new(x: f32, y: f32, direction: Direction, ctx: &mut Context)->Self{
        let adjusted_y = y + 55.0;
        let sprites: Vec<Image> = get_sprites("Batarang", 4, "batarang", ctx);
        let scale = if direction == Direction::Left { -SCALE } else { SCALE };
        Self { 
            x, 
            y: adjusted_y, 
            counter: 0.0, 
            move_speed: 15.0, 
            direction, 
            sprites, 
            scale 
        }
    }

    pub fn draw(&mut self, canvas:&mut  Canvas){



        canvas.draw(self.get_current_sprite(), DrawParam::default()
            .dest([self.x, self.y])
            .scale([self.scale, 3.4]));
        }

    pub fn update(&mut self){
        self.x += if self.direction == Direction::Left { -self.move_speed } else { self.move_speed };

        if self.counter.round() as usize > self.sprites.len(){
            self.counter = 0.0;
        }
    }

    pub fn is_offscreen(&self) -> bool{
        if self.x > WIDTH || self.x < 0.0{ true } else { false }
    }

    fn get_current_sprite(&self) -> &Image{
        &self.sprites[self.counter.round() as usize]
    }

    pub fn get_x(&self)-> f32{
        self.x
    }

}