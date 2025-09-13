use ggez::graphics::Image;
use ggez::{Context, GameResult};


pub fn get_sprites(folder_path: &str, num_of_sprites: i32, sprite_name: &str, _ctx: &mut Context ) -> GameResult<Vec<Image>>{
    let mut sprites: Vec<Image> = Vec::new();
    for i in 1..num_of_sprites+1{
        let path = format!("/{}/{}{}.png", folder_path, sprite_name, i);
        let sprite = Image::from_path(_ctx, path)?;
        sprites.push(sprite);
    }
    Ok(sprites)
}