use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawParam, Drawable, Image};
use ggez::event::{EventHandler};
use ggez::input::keyboard::{self, KeyCode};
use crate::batman::{Batman, Batarang};
use crate::game_defs::{HEIGHT, WIDTH};
use crate::knife_guy::{self, KnifeGuy};
use crate::enemy::{self, Enemy};
use crate::gun_guy::{GunGuy, Bullet};
use crate::enemies::Enemies;





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
    bullets: Vec<Bullet>
}

impl MyGame {
    pub fn new(_ctx: &mut Context, background_path: &str, buildings_path: &str, ground_path: &str) -> GameResult<Self> {
        let background = Image::from_path(_ctx, background_path)?;
        let buildings = Image::from_path(_ctx, buildings_path)?;
        let ground = Image::from_path(_ctx, ground_path)?;
        let enemy = Enemies::Gun(GunGuy::new(10.0, 10.0, _ctx)?);
        let mut enemies: Vec<Enemies> = Vec::new();
        enemies.push(enemy);

        let batman = Batman::new( 0.0, 0.0, 10.0, _ctx, 100.0)?;
        let ground_height = ((HEIGHT as u32) - ground.height()) as f32;

        Ok(Self { background, buildings, ground, batman, ground_height, enemies, game_over: false, batarangs: Vec::new(), batarang_xs: Vec::new(), bullets: Vec::new() })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.batman.is_alive(){
            self.batman.update(_ctx, self.ground_height, & mut self.batarangs);
            for batarang in self.batarangs.iter_mut(){
                batarang.update();
                let batarang_x: f32 = batarang.get_x();
                self.batarang_xs.push(batarang_x);
            }

            for bullet in self.bullets.iter_mut(){
                bullet.update();
            }

            self.bullets.retain(|b| !b.is_offscreen() && !b.hit_batman(&mut self.batman));

            self.batarangs.retain(|b| !b.is_offscreen());
            
            for (i,  enemy) in self.enemies.iter_mut().enumerate(){
                enemy.update(self.ground_height, &mut self.batman, &self.batarang_xs, &mut self.bullets);
            }
            self.enemies.retain(|b| b.is_alive());
            self.batarang_xs.clear();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);

        canvas.draw(&self.background, DrawParam::default()
            .scale([10.0, 10.0]));
        canvas.draw(&self.buildings, DrawParam::default()
            .dest([0.0,150.0])
            .scale([2.4, 5.5]));


        canvas.draw(&self.ground, DrawParam::default()
            .dest([0.0, self.ground_height])
            .scale([2.2, 1.4]));


        println!("{}", self.bullets.len());
        let mut text = graphics::Text::new("The Batman".to_string());
        canvas.draw(&text, DrawParam::default()
            .dest([(WIDTH/2.0 -100.0) as f32, 0.0])
            .scale([2.0, 2.0]));
                
        for enemy in self.enemies.iter_mut(){
            enemy.draw(&mut canvas, ctx, self.batman.get_x(), self.batman.get_width());
        }

        for bullet in self.bullets.iter_mut(){
            bullet.draw(&mut canvas, ctx);
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
                let enemy = GunGuy::new(10.0, 10.0, ctx)?;
                self.enemies.push(Enemies::Gun(enemy));
                
            }
        }

        for batarang in self.batarangs.iter_mut(){
                batarang.draw(&mut canvas);
            }

        canvas.finish(ctx)
    }

}


