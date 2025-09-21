use ggez::conf::{WindowMode, WindowSetup};
use ggez::{ContextBuilder, GameResult};
use ggez::event::{self};
use std::env;
use std::path::PathBuf;
mod game_defs;
mod batman;
mod sprites;
mod game;
mod enemies;
mod projectiles;


use crate::game_defs::{WIDTH, HEIGHT};



fn main() -> GameResult {
    
    let resources_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    
    
    let (mut ctx, event_loop) = ContextBuilder::new("The Batman: Escape from Arkham", "Timothy Wesley")
    .window_setup(WindowSetup::default().title("The Batman"))
    .window_mode(WindowMode::default().dimensions(WIDTH, HEIGHT))
    .add_resource_path(resources_dir)
    .build()
    .expect("aieee, could not create ggez context!");

    let my_game  = game::MyGame::new(&mut ctx, "/bg-game.png", "/buildings.png", "/ground.png")?;
    
    event::run(ctx, event_loop, my_game);

}


