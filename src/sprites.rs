use ggez::graphics::Image;
use ggez::{Context, GameResult};


pub fn get_sprites(folder_path: &str, num_of_sprites: i32, sprite_name: &str, _ctx: &mut Context ) -> Vec<Image>{
    let mut sprites: Vec<Image> = Vec::new();
    for i in 1..num_of_sprites+1{
        let path = format!("/{}/{}{}.png", folder_path, sprite_name, i);
        let sprite = Image::from_path(_ctx, path);
        let sprite = match sprite {
            Ok(e)=> e,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        sprites.push(sprite);
    }
    sprites
}