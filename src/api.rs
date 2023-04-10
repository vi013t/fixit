use rand::Rng;
use std::path::PathBuf;

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
    fn update(&mut self) -> GameResult {
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
    fn on_key_pressed(&mut self, _code: &VirtualKeyCode) -> GameResult {
        Ok(())
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
    fix_key: &'static VirtualKeyCode,
    frames_since_broken: Option<i32>,
    key_object: Option<KeyPopup>,
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
    pub fn mess_up(&mut self) {
        self.frames_since_broken = Some(0);
        let center = self.position + self.dimensions() / 2.;
        self.key_object = Some(KeyPopup::new(center - KeyPopup::dimensions() / 2. + Vec2::new(0., 100.), self.fix_key));
    }

    /// Returns the dimensions to display the image at. To display the texture properly
    /// without scaling or stretching issues, this should match the aspect ratio of the image
    /// and be no larger than the image.
    fn dimensions(&self) -> Vec2 {
        Vec2::new(300., 300.)
    }
}

impl GameObject for FixableGameObject {

    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {

        // Get drawing arguments
        let scale_factor: f32 = self.dimensions().x / self.texture.width() as f32;
        let texture = if self.is_broken() { &self.broken_texture } else { &self.texture };

        // Draw the image
        canvas.draw(texture, DrawParam::new().dest_rect(Rect::new(self.position.x, self.position.y, scale_factor, scale_factor)));

        // Draw the key icon if this is broken
        if self.key_object.is_some() {
            self.key_object.as_ref().unwrap().draw(ctx, canvas)?;
        }

        // Exit with no errors
        Ok(())
    }

    fn on_key_pressed(&mut self, code: &VirtualKeyCode) -> GameResult {
        if code == self.fix_key {
            self.frames_since_broken = None;
            self.key_object = None;
        }

        Ok(())
    }

    fn update(&mut self) -> GameResult {
        if self.is_broken() {
            self.frames_since_broken = Some(self.frames_since_broken.as_ref().unwrap() + 1);
        } else {
            let chance_to_break_per_frame = 0.005;
            if rand::thread_rng().gen_range(0. ..1.) < chance_to_break_per_frame {
                self.mess_up();
            }
        }
        Ok(())
    }
}

struct KeyPopup {
    position: Vec2,
    text: Text
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
    pub fn new(position: Vec2, key: &'static VirtualKeyCode) -> Self {

        // Create the text object
        let mut text = Text::new(format!("{:?}", key));
        text.set_font("PixeloidSans");

        // Return the key game object
        Self {
            position,
            text
        }
    }

    /// The size to render the key icon at
    pub fn dimensions() -> Vec2 {
        Vec2::new(100., 100.)
    }

    pub fn texture(ctx: &Context) -> Image {
        Image::from_path(ctx, PathBuf::from("/key.png")).unwrap()
    }
}

impl GameObject for KeyPopup {

    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {

        // Draw the key
        let scale_factor: f32 = KeyPopup::dimensions().x / KeyPopup::texture(ctx).width() as f32;
        canvas.draw(&KeyPopup::texture(ctx), DrawParam::new().dest_rect(Rect::new(self.position.x, self.position.y, scale_factor, scale_factor)));

        // Draw the text
        let text_position = self.position + KeyPopup::dimensions() / 2. - Vec2::new(11., 22.);
        canvas.draw(&self.text, DrawParam::new().dest_rect(Rect::new(text_position.x, text_position.y, 3., 3.)));

        // Exit with no errors
        Ok(())
    }
}
