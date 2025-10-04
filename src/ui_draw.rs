use ggez::graphics::{self, Canvas, Color, DrawParam, Image, DrawMode, Mesh, Rect, Text};
use ggez::{Context, GameResult};
use crate::game_defs::{HEIGHT, WIDTH};


pub enum UIDraw{
    Text(Text),
    Mesh(Mesh),
    Image(Image)
}

impl UIDraw{
    pub fn make_text(text: String)-> UIDraw{
        let text_display = graphics::Text::new(text);
        UIDraw::Text(text_display)
    }   

    pub fn make_image(_ctx: &mut Context, path: &str)->UIDraw{
        let background_res = Image::from_path(_ctx, path);
        let background = match background_res {
            Ok(e) => e,
            Err(e) => {
                println!("{}", e);
                let error_text = graphics::Text::new("Error loading Image".to_owned());
                return UIDraw::Text(error_text);
            }
        };

        UIDraw::Image(background)


    } 

    pub fn make_mesh(_ctx: &mut Context, x: f32, y: f32, width: f32, height: f32, color: Color)-> UIDraw{
        let mesh_res = Mesh::new_rectangle(_ctx, DrawMode::fill(), Rect::new(x, y, width, height), color);
        let mesh = match mesh_res {
            Ok(e) => e,
            Err(e) => {
                println!("{}", e);
                let error_text = graphics::Text::new("Error loading Mesh".to_owned());
                return UIDraw::Text(error_text);

            }
        };

        UIDraw::Mesh(mesh)
    }


    pub fn draw(&mut self, canvas:&mut Canvas, dest: [f32;2], scale: [f32;2]){
        match self {
            UIDraw::Text(e) => {
                canvas.draw(e, DrawParam::default()
                .dest(dest)
                .scale(scale));
            }
            UIDraw::Image(e) => {
                canvas.draw(e, DrawParam::default()
                .dest(dest)
                .scale(scale));
            }
            UIDraw::Mesh(e) => {
                canvas.draw(e, DrawParam::default()
                .scale(scale));
            }
        };
    }

    pub fn set_scale(&mut self, scale: f32){
        match self {
            UIDraw::Text(e) => {
                e.set_scale(scale);
            }
            _ => {}
        };
    }

    pub fn height(&mut self)-> u32{
        let false_number = 1;
        match self {
            UIDraw::Image(e)=> e.height(),
            _ => false_number
        }
    }



}