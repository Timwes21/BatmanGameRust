use ggez::{Context, GameResult};
use ggez::graphics::{Canvas, Color};
use ggez::event::{EventHandler};
use ggez::input::keyboard::{self, KeyCode};
use crate::batman::{Batman, Batarang};
use crate::game_defs::{HEIGHT, WIDTH};
use crate::projectiles::Projectiles;
use crate::enemies::{AxeGuy, Enemies, FireMime, GunGuy, KnifeGuy, Mime, RocketGuy};
use rand::prelude::*;
use crate::ui_draw::UIDraw;



struct Rand {
    rng: ThreadRng, 
    nums: Vec<i32> 
}

impl Rand {
    fn new(min: i32, max: i32)-> Self{
        let rng = rand::rng();
        let nums: Vec<i32> = (min..max).collect();
        Self { rng, nums }
    }

    fn shuffle (&mut self)-> f32{        
            self.nums.shuffle(&mut self.rng);
            let enemy_x_res = self.nums.choose(&mut self.rng);
            let enemy_x = match enemy_x_res {
                Some(x) => x,
                None => &100
            };
            
            enemy_x.to_owned() as f32
    }

    fn normalize(&mut self)-> f32{
        self.shuffle()/ 10.0
    }

}




pub struct MyGame {
    background: UIDraw,
    buildings: UIDraw,
    ground: UIDraw,
    batman: Batman,
    ground_height: f32,
    enemies_on_screen: Vec<Enemies>,
    pub game_over: bool,
    batarangs: Vec<Batarang>,
    batarang_xs: Vec<f32>,
    projectiles: Vec<Projectiles>,
    paused: bool,
    wave: usize,
    takeaway: usize,
    enemies_spawned: usize,
    enemies_to_spawn: Vec<Enemies>,
    sum_enemies_killed: usize

}

impl MyGame {
    pub fn new(_ctx: &mut Context, background_path: &str, buildings_path: &str, ground_path: &str) -> GameResult<Self> {
        let background = UIDraw::make_image(_ctx, background_path);
        let buildings = UIDraw::make_image(_ctx, buildings_path);
        let mut ground = UIDraw::make_image(_ctx, ground_path);
        // let enemy = Enemies::AxeGuy(AxeGuy::new(WIDTH/2.0, 10.0, _ctx));
        let enemies_on_screen: Vec<Enemies> = Vec::new();
        // enemies.push(enemy);

        let batman = Batman::new( 0.0, 0.0, 10.0, _ctx, 100.0);
        let ground_height = ((HEIGHT as u32) - ground.height()) as f32;

        Ok(
            Self { 
                background, 
                buildings, 
                ground, 
                batman, 
                ground_height, 
                enemies_on_screen, 
                enemies_to_spawn: Vec::new(), 
                game_over: false, 
                batarangs: Vec::new(), 
                batarang_xs: Vec::new(), 
                projectiles: Vec::new(), 
                paused: false, 
                wave: 1, 
                takeaway: 0, 
                enemies_spawned: 0,
                sum_enemies_killed: 0
            }
        )
    }



    fn spawn_enemies(&mut self, _ctx: &mut Context){
        let enemy_limit = self.wave * 2;
        let enemy_screen_limit = 6;
        
        let target = enemy_limit.clamp(0, 40);
        let enemies_num = target - self.enemies_spawned;
        let enemies_to_spawn = enemies_num.clamp(0, enemy_screen_limit - self.enemies_on_screen.len());
        
        if enemies_to_spawn == 0 {return};


        for _ in 0..enemies_to_spawn{
            if self.enemies_to_spawn.len() == 0{ self.refill_enemies_to_spawn(_ctx);}

            let enemy = match self.enemies_to_spawn.pop(){
                Some(e) => e,
                None => {println!(); return}
            };
            self.enemies_on_screen.push(enemy);
            self.enemies_spawned += 1;
            
        }
    }

    fn restart_game(&mut self, ctx: &mut Context){
        self.batman = Batman::new( 0.0, 0.0, 10.0, ctx, 100.0);
        self.wave = 1;
        self.enemies_on_screen.clear();
        self.takeaway = 0;
        self.enemies_spawned = 0;
        self.sum_enemies_killed = 0;
    }

    fn refill_enemies_to_spawn(&mut self, _ctx: &mut Context){
        let enemy_y = 700.0;
        let min_x = 0;
        let max_x = (WIDTH - 100.0) as i32;
        let min_walking_speed = 10;
        let max_walking_speed = 30;
        
        let mut rand_x = Rand::new(min_x, max_x);
        let mut rand_walking_speed = Rand::new(min_walking_speed, max_walking_speed);

        let enemies_to_spawn = match self.wave {
                w if w >= 30 => vec![
                            Enemies::Mime(Mime::new(rand_x.shuffle(), enemy_y, rand_walking_speed.normalize(), _ctx)),
                            Enemies::Rocket(RocketGuy::new(rand_x.shuffle(), enemy_y, _ctx)),
                            Enemies::FireMime(FireMime::new(rand_x.shuffle(), enemy_y, rand_walking_speed.normalize(), _ctx)),
                            Enemies::AxeGuy(AxeGuy::new(rand_x.shuffle(), enemy_y, rand_walking_speed.normalize(), _ctx))],
                
                w if w >= 20 => vec![
                            Enemies::Mime(Mime::new(rand_x.shuffle(), enemy_y, rand_walking_speed.normalize(), _ctx)),
                            Enemies::Rocket(RocketGuy::new(rand_x.shuffle(), enemy_y, _ctx)),
                            Enemies::FireMime(FireMime::new(rand_x.shuffle(), enemy_y, rand_walking_speed.normalize(), _ctx))],
                
                w if w >= 10  => vec![
                            Enemies::Mime(Mime::new(rand_x.shuffle(), enemy_y, rand_walking_speed.normalize(), _ctx)),
                            Enemies::Gun(GunGuy::new(rand_x.shuffle(), enemy_y, rand_walking_speed.normalize(), _ctx)),
                            Enemies::Knife(KnifeGuy::new(rand_x.shuffle(), enemy_y, rand_walking_speed.normalize(), _ctx))],
                
                w if w > 5  => vec![
                            Enemies::Gun(GunGuy::new(rand_x.shuffle(), enemy_y, rand_walking_speed.normalize(), _ctx)), 
                            Enemies::Knife(KnifeGuy::new(rand_x.shuffle(), enemy_y, rand_walking_speed.normalize(), _ctx))],
                
                _ => vec![Enemies::Knife(KnifeGuy::new(rand_x.shuffle(), enemy_y, rand_walking_speed.normalize(), _ctx))]
            };

        self.enemies_to_spawn = enemies_to_spawn;
    }

}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        
        self.spawn_enemies(_ctx);
        if self.enemies_on_screen.len() == 0 {

            self.enemies_spawned = 0;
            self.wave += 1;
            self.refill_enemies_to_spawn(_ctx);
            self.batman.reset_health();
            
        }

		
		
        if keyboard::is_key_pressed(_ctx, KeyCode::P){
            self.paused = true;
        }
        if keyboard::is_key_pressed(_ctx, KeyCode::R){
            self.paused = false;
        }

        if self.batman.is_alive() && !self.paused{
            self.batman.update(_ctx, self.ground_height, & mut self.batarangs);


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

            
            for (i,  enemy) in self.enemies_on_screen.iter_mut().enumerate(){
                enemy.update(self.ground_height, &mut self.batman, &self.batarang_xs, &mut self.projectiles);
            }
            self.enemies_on_screen.retain(|b| b.is_alive());
            self.batarang_xs.clear();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!("{}", self.projectiles.len());
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);

        

        self.background.draw(&mut canvas, [0.0, 0.0], [10.0, 10.0]);
        self.buildings.draw(&mut canvas, [0.0,150.0], [2.4, 5.5]);
        self.ground.draw(&mut canvas, [0.0, self.ground_height], [2.2, 1.4]);



        let mut wave = UIDraw::make_text(format!("Wave: {}", self.wave).to_string());
        wave.draw(&mut canvas, [(WIDTH/2.0 -100.0) as f32, 50.0], [3.0, 3.0]);
        
                
        for enemy in self.enemies_on_screen.iter_mut(){
            enemy.draw(&mut canvas, ctx, self.batman.get_x(), self.batman.get_width());
        }

        for projectile in self.projectiles.iter_mut(){
            projectile.draw(&mut canvas, ctx);
        }

        self.batman.draw(&mut canvas, ctx);
        if !self.batman.is_alive(){
            self.game_over = true;
            
            let mut game_over_text = UIDraw::make_text("Game Over".to_string());
            game_over_text.set_scale(100.0);
            game_over_text.draw(&mut canvas, [(WIDTH/2.0-220.0) as f32, HEIGHT/2.0 - 100.0], [1.0, 1.0]);
            
            let mut press_center = UIDraw::make_text("Press Space To Start Over".to_string());
            press_center.set_scale(40.0);
            press_center.draw(&mut canvas, [(WIDTH/2.0 - 240.0) as f32, HEIGHT/2.0], [1.0, 1.0]);

            if keyboard::is_key_pressed(ctx, KeyCode::Space){
                self.restart_game(ctx);
            }
        }

        for batarang in self.batarangs.iter_mut(){
            batarang.draw(&mut canvas);
        }

        if self.paused{
            let mut press_r = UIDraw::make_text("Press R to Resume".to_string());
            press_r.set_scale(40.0);
            press_r.draw(&mut canvas, [WIDTH - 400.0, 0.0], [1.0, 1.0]);

        } else {
            let mut press_p = UIDraw::make_text("Press P to Pause".to_string());
            press_p.set_scale(40.0);
            press_p.draw(&mut canvas, [WIDTH - 400.0, 0.0], [1.0, 1.0]);
        }

        let mut enemies_killed_text = UIDraw::make_text(format!("Enemies Killed: {}", self.sum_enemies_killed));
        enemies_killed_text.set_scale(25.0);
        enemies_killed_text.draw(&mut canvas, [WIDTH/ 3.0, 0.0], [1.0, 1.0]);


        canvas.finish(ctx)
    }





}


