use ggez::{conf::FullscreenType, event, graphics::FontData, ContextBuilder, GameResult};
use screen::{create_objects, Window};
use util::get_resource_dir;

mod api;
mod screen;
mod util;

pub fn main() -> GameResult {
    // Create the context and event loop
    let (mut ctx, event_loop) = ContextBuilder::new("fixit", "Neph Iapalucci")
        .add_resource_path(get_resource_dir())
        .build()?;
    ctx.gfx.set_window_title("Fixit");
    ctx.gfx.set_fullscreen(FullscreenType::Desktop)?;
    ctx.gfx.add_font(
        "PixeloidSans",
        FontData::from_path(&ctx, "/fonts/PixeloidSans.ttf")?,
    );

    // Create the main window
    let mut window = Window::new(9999, 9999);

    // Add objects to the screen
    for object in create_objects(&ctx) {
        window.add_component(object);
    }

    // Run the event loop
    event::run(ctx, event_loop, window);
}
