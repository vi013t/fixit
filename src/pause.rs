use ggez::{Context, graphics::{Canvas, DrawParam, Text, Rect, Quad, Color}, GameResult, glam::Vec2, winit::event::VirtualKeyCode};

use crate::api::GameObject;

pub struct GameOverScreen {
    title: Text,
    description: Text
}

pub enum GameOverCause<'a> {
    WrongKey(&'a VirtualKeyCode),
    NotInTime(&'a VirtualKeyCode)
}

impl GameOverScreen {
    pub fn new(cause: GameOverCause) -> Self {
        let mut title = Text::new("Game Over");
        title.set_font("PixeloidSans");
        title.set_scale(100.);

        let mut description = Text::new(
            match cause {
                GameOverCause::WrongKey(key) => format!("You pressed {:?} when nothing needed it.", key),
                GameOverCause::NotInTime(key) => format!("You waited too long to press {:?}.", key)
            }
        );
        description.set_font("PixeloidSans");
        description.set_scale(100.);

        Self {
            title,
            description
        }
    }
}
 
impl GameObject for GameOverScreen{
    
    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        static TEXT_SCALE: f32 = 2.;
        let text_dimensions = {
            let dims = self.title.measure(&ctx.gfx)?;
            Vec2::new(dims.x * TEXT_SCALE, dims.y * TEXT_SCALE)
        };
        
        canvas.draw(&Quad, DrawParam::default().color(Color::BLACK).dest_rect(Rect::new(0., 0., 1920., 1080.)));
        canvas.draw(&self.title, DrawParam::new().scale(Vec2::new(TEXT_SCALE, TEXT_SCALE)).dest(Vec2::new(1920. / 2. - text_dimensions.x / 2., 20.)));
        
        let desc_scale = 1.;
        let desc_dimensions = {
            let dims = self.description.measure(&ctx.gfx)?;
            Vec2::new(dims.x * desc_scale, dims.y * desc_scale)
        };
        canvas.draw(&self.description, DrawParam::new().dest(Vec2::new(1920. / 2. - desc_dimensions.x / 2., 300.)));

        Ok(())
    }
}

trait Button {
    fn draw(&self);
    fn on_click(&self);
}
