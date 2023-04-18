use ggez::{Context, graphics::{Canvas, DrawParam, Text, Rect, Quad, Color}, GameResult, glam::Vec2};

use crate::api::GameObject;

pub struct GameOverScreen;

impl GameOverScreen {
    pub fn new() -> Self {
        Self
    }
}
 
impl GameObject for GameOverScreen{
    
    fn draw(&self, _ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        canvas.draw(&Quad, DrawParam::default().color(Color::BLACK).dest_rect(Rect::new(0., 0., 1920., 1080.)));
        canvas.draw(&Text::new("Game Over"), DrawParam::new().scale(Vec2::new(5., 5.)));
        Ok(())
    }
}
