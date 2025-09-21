use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawParam, Drawable, Image, DrawMode, Mesh, Rect};
use ggez::event::{EventHandler};
use ggez::input::keyboard::{self, KeyCode};
use crate::batman::{Batman, Batarang};
use crate::game_defs::{HEIGHT, WIDTH};
use crate::projectiles::Projectile;
use crate::enemies::{Axe, AxeGuy, Bullet, Enemies, FireMime, GunGuy, KnifeGuy, Mime, Rocket, RocketGuy};
use rand::prelude::*;







pub struct MyGame {
    background: Image,
    buildings: Image,
    ground: Image,
    batman: Batman,
    ground_height: f32,
    enemies: Vec<Enemies>,
    pub game_over: bool,
    batarangs: Vec<Batarang>,
    batarang_xs: Vec<f32>,
    projectiles: Vec<Projectile>,
    paused: bool,
    wave: usize,
    takeaway: usize

}

impl MyGame {
    pub fn new(_ctx: &mut Context, background_path: &str, buildings_path: &str, ground_path: &str) -> GameResult<Self> {
        let background = Image::from_path(_ctx, background_path)?;
        let buildings = Image::from_path(_ctx, buildings_path)?;
        let ground = Image::from_path(_ctx, ground_path)?;
        let enemy = Enemies::AxeGuy(AxeGuy::new(WIDTH/2.0, 10.0, _ctx));
        let mut enemies: Vec<Enemies> = Vec::new();
        enemies.push(enemy);

        let batman = Batman::new( 0.0, 0.0, 10.0, _ctx, 100.0)?;
        let ground_height = ((HEIGHT as u32) - ground.height()) as f32;

        Ok(Self { background, buildings, ground, batman, ground_height, enemies, game_over: false, batarangs: Vec::new(), batarang_xs: Vec::new(), projectiles: Vec::new(), paused: false, wave: 1, takeaway: 0 })
    }

    fn get_random_x(start: i32, end: i32)-> f32{
        let mut rng = rand::rng();


        let mut nums: Vec<i32> = (start..end).collect();

        nums.shuffle(&mut rng);
        let enemy_x_res = nums.choose(&mut rng);
        let enemy_x = match enemy_x_res {
            Some(x) => x,
            None => &100
        };

        enemy_x.to_owned() as f32
    }

    fn spawn_enemies(&mut self, _ctx: &mut Context){
        let enemy_y = 500.0;
        if self.enemies.len() <  (self.wave * 2).checked_sub(self.takeaway).unwrap_or(0) && self.wave < 6 {
            let enemy_x = Self::get_random_x(200, 900);

			let enemy = KnifeGuy::new(enemy_x, enemy_y, _ctx);
			self.enemies.push(Enemies::Knife(enemy));
			self.takeaway+= 1;
		}
		if self.enemies.len() < (self.wave * 2).checked_sub(self.takeaway).unwrap_or(0) && self.wave < 6 && self.wave < 20 {
            let enemy_x = Self::get_random_x(200, 900);
			let enemy = GunGuy::new(enemy_x, enemy_y, _ctx);
			self.enemies.push(Enemies::Gun(enemy));
			self.takeaway+= 1;
		}
		if self.enemies.len() < (self.wave/2).checked_sub(self.takeaway).unwrap_or(0) && self.wave > 9 {
            let enemy_x = Self::get_random_x(200, 900);
			let enemy = Mime::new(enemy_x, enemy_y, _ctx);
			self.enemies.push(Enemies::Mime(enemy));
			self.takeaway+= 1;
		}
		if self.enemies.len() < (self.wave/2).checked_sub(self.takeaway).unwrap_or(0) && self.wave > 19 {
            let enemy_x = Self::get_random_x(200, 900);
			let enemy = AxeGuy::new(enemy_x, enemy_y, _ctx);
			self.enemies.push(Enemies::AxeGuy(enemy));
			self.takeaway+= 1;
		}
		if self.enemies.len() < (self.wave/2).checked_sub(self.takeaway).unwrap_or(0) && self.wave > 24 {
            let enemy_x = Self::get_random_x(200, 900);
			let enemy = FireMime::new(enemy_x, enemy_y, _ctx);
			self.enemies.push(Enemies::FireMime(enemy));
			self.takeaway+= 1;
		}
		if self.enemies.len() < (self.wave/2).checked_sub(self.takeaway).unwrap_or(0) && self.wave > 30 {
            let enemy_x = Self::get_random_x(200, 900);
			let enemy = RocketGuy::new(enemy_x, enemy_y, _ctx);
			self.enemies.push(Enemies::Rocket(enemy));
			self.takeaway+= 1;
		} 
    }

}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        
        self.spawn_enemies(_ctx);
		
		
        if keyboard::is_key_pressed(_ctx, KeyCode::P){
            self.paused = true;
        }
        if keyboard::is_key_pressed(_ctx, KeyCode::R){
            self.paused = false;
        }

        if self.batman.is_alive() && !self.paused{
            self.batman.update(_ctx, self.ground_height, & mut self.batarangs);

            if self.wave < 10{

            }

            for batarang in self.batarangs.iter_mut(){
                batarang.update();
                let batarang_x: f32 = batarang.get_x();
                self.batarang_xs.push(batarang_x);
            }

            for (i, projectile) in self.projectiles.iter_mut().enumerate(){
                projectile.update();
            }


            self.projectiles.retain(|b| !b.getting_rid_of(&mut self.batman));
            self.batarangs.retain(|b| !b.is_offscreen());

            
            for (i,  enemy) in self.enemies.iter_mut().enumerate(){
                enemy.update(self.ground_height, &mut self.batman, &self.batarang_xs, &mut self.projectiles);
            }
            self.enemies.retain(|b| b.is_alive());
            self.batarang_xs.clear();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!("{}", self.projectiles.len());
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);

        


        canvas.draw(&self.background, DrawParam::default()
            .scale([10.0, 10.0]));
        canvas.draw(&self.buildings, DrawParam::default()
            .dest([0.0,150.0])
            .scale([2.4, 5.5]));


        canvas.draw(&self.ground, DrawParam::default()
            .dest([0.0, self.ground_height])
            .scale([2.2, 1.4]));


        let mut text = graphics::Text::new("The Batman".to_string());
        canvas.draw(&text, DrawParam::default()
            .dest([(WIDTH/2.0 -100.0) as f32, 0.0])
            .scale([2.0, 2.0]));
                
        for enemy in self.enemies.iter_mut(){
            enemy.draw(&mut canvas, ctx, self.batman.get_x(), self.batman.get_width());
        }

        for projectile in self.projectiles.iter_mut(){
            projectile.draw(&mut canvas, ctx);
        }

        self.batman.draw(&mut canvas, &ctx);
        if !self.batman.is_alive(){
            self.game_over = true;
            let mut game_over = graphics::Text::new("Game Over".to_string());
            let mut press_center = graphics::Text::new("Press Enter To Start Over".to_string());
            game_over.set_scale(100.0);
            press_center.set_scale(40.0);

            canvas.draw(&game_over, DrawParam::default()
                .dest([(WIDTH/2.0-220.0) as f32, HEIGHT/2.0 - 100.0])
                .scale([1.0, 1.0]));

            canvas.draw(&press_center, DrawParam::default()
                .dest([(WIDTH/2.0 - 240.0) as f32, HEIGHT/2.0])
                .scale([1.0, 1.0]));
            if keyboard::is_key_pressed(ctx, KeyCode::Space){
                self.batman = Batman::new( 0.0, 0.0, 10.0, ctx, 100.0)?;
                self.enemies.clear();
                let enemy = KnifeGuy::new(10.0, 10.0, ctx);
                self.enemies.push(Enemies::Knife(enemy));
                
            }
        }

        for batarang in self.batarangs.iter_mut(){
                batarang.draw(&mut canvas);
            }

        if self.paused{
            println!("Should be printing");
            let backdrop = Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(WIDTH/3.0, HEIGHT/3.0, WIDTH/2.0, HEIGHT/2.0), Color::from_rgb(255, 0, 0))?;

            let mut press_r = graphics::Text::new("Press R to Resume".to_string());
            press_r.set_scale(40.0);
            canvas.draw(&press_r, DrawParam::default()
                .dest([WIDTH - 400.0, 0.0])
                .scale([1.0, 1.0]));


            canvas.draw(&backdrop, DrawParam::default()
                .dest([0.0,150.0])
                .scale([3.4, 3.5]));
        }
        else {
            let mut press_p = graphics::Text::new("Press P to Pause".to_string());
            press_p.set_scale(40.0);
            canvas.draw(&press_p, DrawParam::default()
                .dest([WIDTH - 400.0, 0.0])
                .scale([1.0, 1.0]));
        }


        canvas.finish(ctx)
    }





}


