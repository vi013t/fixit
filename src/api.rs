use rand::Rng;
use std::path::PathBuf;
use crate::screen::GameState;
use ggez::{
    glam::Vec2,
    graphics::{Canvas, DrawParam, Image, Rect, Text},
    winit::event::VirtualKeyCode,
    Context, GameResult,
};

/// An object that exist in the game and can be drawn to the screen.
pub trait GameObject {
    /// Draws this component to the screen.
    ///
    /// ## Parameters
    /// ```rust
    /// ctx: &mut Context
    /// ```
    /// - The drawing context
    /// ```rusts
    /// canvas: &mut Canvas
    /// ```
    /// - The canvas to draw on
    ///
    /// ## Returns
    ///
    /// `Ok(())` if drawn successfully, or `GameError` if an error occurred.
    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult;

    /// Updates this component every frame.
    fn update(&mut self, _state: &GameState) -> GameResult {
        Ok(())
    }

    /// Called when a key is pressed.
    ///
    /// ## Parameters
    /// ```rust
    /// &mut self
    /// ```
    /// The component that is listening for a key press
    /// ```rust
    /// code: &VirtualKeyCode
    /// ```
    /// The key pressed.
    ///
    fn on_key_pressed(&mut self, _code: &VirtualKeyCode) -> bool {
        false
    }

    /// Called when a key is released.
    ///
    /// ## Parameters
    /// ```rust
    /// &mut self
    /// ```
    /// The component that is listening for a key press
    /// ```rust
    /// code: &VirtualKeyCode
    /// ```
    /// The key released.
    ///
    fn on_key_released(&mut self, _code: &VirtualKeyCode) -> GameResult {
        Ok(())
    }
}

pub struct FixableGameObject {
    position: Vec2,
    texture: Image,
    broken_texture: Image,
    pub fix_key: &'static VirtualKeyCode,
    frames_since_broken: Option<i32>,
    pub key_object: Option<KeyPopup>,
}

impl FixableGameObject {
    /// Creates a new instance of a "fixable" / "breakable" game object.
    /// ## Parameters
    /// ```rust
    /// texture: &str
    /// ```
    /// - The texture for the object, represented as a path to the image file relative to `resources`.
    /// ```rust
    /// position: Vec2
    /// ```
    /// - The coordinates of the upper-left corner of this object
    /// ```rust
    /// fix_key: &'static VirtualKeyCode
    /// ```
    /// - The key that will fix this object when pressed
    /// ```rust
    /// ctx: &Context
    /// ```
    /// - The drawing context, used to fetch the image texture.
    ///
    /// ### Returns
    /// The newly created `FixableGameObject`.
    ///
    pub fn new(texture: &str, position: Vec2, fix_key: &'static VirtualKeyCode, ctx: &Context) -> Self {
        Self {
            position,
            fix_key,
            frames_since_broken: None,
            texture: Image::from_path(&ctx.gfx, PathBuf::from(texture.to_owned() + ".png")).unwrap(),
            broken_texture: Image::from_path(&ctx.gfx, PathBuf::from(texture.to_owned() + "_broken.png")).unwrap(),
            key_object: None,
        }
    }

    /// Returns whether or not this object is currently "broken" and is awaiting keyboard input
    /// to be fixed.
    ///
    /// ### Parameters
    /// ```rust
    /// &self
    /// ```
    /// - A reference to the object to check
    ///
    /// ### Returns
    /// ```rust
    /// bool
    /// ```
    /// - Whether or not the object is broken
    pub fn is_broken(&self) -> bool {
        self.frames_since_broken.is_some()
    } 

    /// "Breaks" this object. The texture is updated to the broken version and the timer will be changed.
    pub fn mess_up(&mut self, state: &GameState) {
        self.frames_since_broken = Some(0);
        let dimensions = Vec2::new(
            self.broken_texture.width() as f32,
            self.broken_texture.height() as f32,
        ) * 6.4;
        let center = self.position + dimensions / 2.;
        let bottom = self.position.y + dimensions.y;

        self.key_object = Some(KeyPopup::new(
            Vec2::new(center.x, bottom),
            self.fix_key,
            state.broken_lifetime 
        ));
    }
}

impl GameObject for FixableGameObject {
    fn draw(&self, _ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        // Get drawing arguments
        let texture = if self.is_broken() {
            &self.broken_texture
        } else {
            &self.texture
        };

        // Draw the image
        canvas.draw(
            texture,
            DrawParam::new().dest_rect(Rect::new(self.position.x, self.position.y, 6.4, 6.4)),
        );

        // Exit with no errors
        Ok(())
    }

    fn on_key_pressed(&mut self, code: &VirtualKeyCode) -> bool {
        if code == self.fix_key && self.is_broken() {
            self.frames_since_broken = None;
            self.key_object = None;
            return true;
        }

        false
    }

    fn update(&mut self, state: &GameState) -> GameResult {
        if self.is_broken() {
            self.frames_since_broken = Some(self.frames_since_broken.as_ref().unwrap() + 1);
            self.key_object.as_mut().unwrap().update(state)?;
        } else {
            if rand::thread_rng().gen_range(0. ..1.) < state.chance_of_breaking() {
                self.mess_up(state);
            }
        }
        Ok(())
    }
}

pub struct KeyPopup {
    center: Vec2,
    text: Text,
    pub frames_existed: i32,
    pub lifetime: i32,
}

impl KeyPopup {
    /// Creates a new key icon popup.
    /// ### Parameters
    /// ```rust
    /// position: Vec2
    /// ```
    /// - The position for the upper-left corner of the key icon
    /// ```rust
    /// key: &'static VirtualKeyCode
    /// ```
    /// - A reference to the key that is being displayed
    /// ```rust
    /// ctx: &Context
    /// ```
    /// - The drawing context; Used to fetch the image for the texture.
    ///
    /// ### Returns
    /// The newly created key icon object.
    pub fn new(center: Vec2, key: &'static VirtualKeyCode, lifetime: i32) -> Self {
        // Create the text object
        let mut text = Text::new(format!("{:?}", key));
        text.set_font("PixeloidSans");
        text.set_scale(55.);

        // Return the key game object
        Self {
            center,
            text,
            frames_existed: 0,
            lifetime,
        }
    }

    pub fn texture(ctx: &Context) -> Image {
        Image::from_path(ctx, PathBuf::from("/key.png")).unwrap()
    }
}

impl GameObject for KeyPopup {
    fn update(&mut self, _state: &GameState) -> GameResult {
        self.frames_existed += 1;

        return Ok(());
    }

    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {

        // Draw the key
        let percent_of_lifetime_used = self.frames_existed as f32 / self.lifetime as f32;
        let scale = 6.4 + (percent_of_lifetime_used * 10.).sin();

        canvas.draw(
            &KeyPopup::texture(ctx),
            DrawParam::new().dest_rect(Rect::new(
                self.center.x - KeyPopup::texture(ctx).width() as f32 * scale / 2. - 20.,
                self.center.y - KeyPopup::texture(ctx).width() as f32 * scale / 2.,
                scale, scale
            )),
        );

        // Draw the text
        let text_scale = scale / 6.4;
        let text_dimensions = {
            let dims = self.text.measure(&ctx.gfx)?;
            Vec2::new(dims.x * text_scale, dims.y * text_scale)
        };
        canvas.draw(
            &self.text,
            DrawParam::new().dest_rect(Rect::new(
                self.center.x - text_dimensions.x / 2. - 17.,
                self.center.y - text_dimensions.y / 2.,
                text_scale,
                text_scale,
            ))
        );

        // Exit with no errors
        Ok(())
    }
}
