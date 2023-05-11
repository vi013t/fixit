use ggez::{conf::FullscreenType, event, graphics::FontData, ContextBuilder, GameResult};
use screen::Window;
use util::get_resource_dir;

mod api;
mod pause;
mod screen;
mod util;

/// The main function; Creates the game loop.
pub fn main() -> GameResult {
    
    // Create the context and event loop
    let (mut ctx, event_loop) = ContextBuilder::new("fixit", "Neph Iapalucci").add_resource_path(get_resource_dir()).build()?;
    ctx.gfx.set_window_title("Fixit");
    ctx.gfx.set_fullscreen(FullscreenType::Desktop)?;
    ctx.gfx.add_font("PixeloidSans", FontData::from_path(&ctx, "/fonts/PixeloidSans.ttf")?);

    // Create the main window
    let window = Window::new(&ctx);

    // Run the event loop
    event::run(ctx, event_loop, window);
}

