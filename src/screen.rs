use std::process;

use ggez::{event::EventHandler, Context, GameResult, graphics::{Canvas, Quad, DrawParam, Color, Rect}, input::keyboard::KeyInput, winit::event::VirtualKeyCode, glam::Vec2};

use crate::api::{GameObject, FixableGameObject};

pub struct Window {
    frame: usize,
    components: Vec<Box<dyn GameObject>>,
    width: u32,
    height: u32
}

impl Window {
    /// Creates a new window.
    pub fn new(width: u32, height: u32) -> Window {
        Window {
            frame: 0,
            components: Vec::new(),
            width,
            height
        }
    }

    /// Adds a component to be drawn on the screen.
    ///
    /// **Parameters**
    /// ```rust
    /// &mut self
    /// ```
    /// - The window to add the component to
    /// ```rust
    /// child: impl GameObject + 'static
    /// ```
    /// - The `GameObject` component to add
    pub fn add_component(&mut self, child: Box<dyn GameObject>) {
        self.components.push(child);
    }
}

impl EventHandler for Window {

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.frame += 1;

        // Update components
        for component in &mut self.components {
            component.update()?;
        }

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        if input.keycode.is_some() {
            if input.keycode.unwrap() == VirtualKeyCode::Escape {
                process::exit(0);
            }
            for component in &mut self.components {
                component.on_key_pressed(&input.keycode.unwrap())?;
            }
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
        if input.keycode.is_some() {
            for component in &mut self.components {
                component.on_key_released(&input.keycode.unwrap())?;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, None);

        canvas.draw(
            &Quad,
            DrawParam::default()
                .color(Color::WHITE)
                .dest_rect(Rect::new(0., 0., self.width as f32, self.height as f32)),
        );

        for child in &self.components {
            child.draw(ctx, &mut canvas)?;
        }

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn create_objects(ctx: &Context) -> [Box<dyn GameObject>; 1] {

    let painting = FixableGameObject::new("/painting", Vec2::new(1200., 300.), &VirtualKeyCode::P, ctx);
    
    let objects: [Box<dyn GameObject>; 1] = [Box::new(painting)];
    objects
}