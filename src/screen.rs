use std::process;

use ggez::{
    event::EventHandler,
    glam::Vec2,
    graphics::{Canvas, DrawParam, Image, Rect, Sampler, Quad, Color},
    input::keyboard::KeyInput,
    winit::event::VirtualKeyCode,
    Context, GameResult,
};

use crate::{api::{FixableGameObject, GameObject}, pause::{GameOverScreen, GameOverCause}};

pub struct GameState {
    pub broken_lifetime: i32
}

impl GameState {
    pub fn chance_of_breaking(&self) -> f32 {
        self.broken_lifetime as f32 * 0.00001
    }
}

static GAME_STATE: GameState = GameState {
    broken_lifetime: 120
};

pub struct Window {
    frame: usize,
    components: Vec<FixableGameObject>,
    background: Image,
    menu: Option<Box<dyn GameObject>>
}

impl Window {

    pub fn pause(&mut self, menu: Box<dyn GameObject>) {
        self.menu = Some(menu);
    }

    /// Creates a new window.
    pub fn new(ctx: &Context) -> Window {
        let mut window = Window {
            frame: 0,
            components: Vec::new(),
            background: Image::from_path(ctx, "/background.png").unwrap(),
            menu: None
        };

        for component in create_objects(ctx) {
            window.add_component(component);
        }

        window
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
    pub fn add_component(&mut self, child: FixableGameObject) {
        self.components.push(child);
    }

    pub fn is_paused(&self) -> bool {
        self.menu.is_some()
    }
}

impl EventHandler for Window {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.frame += 1;

        if self.is_paused() {
            return Ok(())
        }

        // Update components
        let mut game_over_key: Option<&VirtualKeyCode> = None;
        for component in &mut self.components {
            component.update(&GAME_STATE)?;
            if component.is_broken() && component.key_object.as_ref().unwrap().frames_existed > 120 {
                game_over_key = Some(component.fix_key);
                break;
            }
        }

        if game_over_key.is_some() {
            self.pause(Box::new(GameOverScreen::new(GameOverCause::NotInTime(game_over_key.unwrap()))));
        }

        std::thread::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        if input.keycode.is_some() {
            if input.keycode.unwrap() == VirtualKeyCode::Escape {
                process::exit(0);
            }

            let mut fixed_something = false;
            for component in &mut self.components {
                if component.on_key_pressed(input.keycode.as_ref().unwrap()) {
                    fixed_something = true
                }
            }

            if !fixed_something {
                self.pause(Box::new(GameOverScreen::new(GameOverCause::WrongKey(input.keycode.as_ref().unwrap()))));
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
        canvas.set_sampler(Sampler::nearest_clamp());

        if self.menu.is_some() {
            self.menu.as_ref().unwrap().draw(ctx, &mut canvas)?;
        } else {

            canvas.draw(&Quad, DrawParam::default().color(Color::BLACK).dest_rect(Rect::new(0., 0., 5000., 5000.)));
            canvas.draw(&self.background, DrawParam::default().dest_rect(Rect::new(0., 0., 6.4, 6.4)));

            for child in &self.components {
                child.draw(ctx, &mut canvas)?;
            }

            for child in &self.components {
                if child.key_object.is_some() {
                    child.key_object.as_ref().unwrap().draw(ctx, &mut canvas)?;
                }
            }
        }
        
        canvas.finish(ctx)?;

        Ok(())
    }
}

/// Returns an array of the game objects boxed that needed to be added to the window.
pub fn create_objects(ctx: &Context) -> [FixableGameObject; 7] {
    let window = FixableGameObject::new("/window", Vec2::new(1165., 51.), &VirtualKeyCode::W, ctx);

    let milk = FixableGameObject::new("/milk", Vec2::new(1220., 384.), &VirtualKeyCode::M, ctx);
    let lamp = FixableGameObject::new("/lamp", Vec2::new(1478., 384.), &VirtualKeyCode::L, ctx);
    let drawer_1 = FixableGameObject::new("/drawer", Vec2::new(1188., 767.), &VirtualKeyCode::D, ctx);
    let drawer_2 = FixableGameObject::new("/drawer", Vec2::new(1188., 645.), &VirtualKeyCode::D, ctx);
    let drawer_3 = FixableGameObject::new("/drawer", Vec2::new(1188., 525.), &VirtualKeyCode::D, ctx);

    let rug = FixableGameObject::new("/rug", Vec2::new(1343., 935.), &VirtualKeyCode::R, ctx);

    [
        window, 
        milk,
        rug,
        lamp,
        drawer_1, 
        drawer_2, 
        drawer_3
    ]
}
