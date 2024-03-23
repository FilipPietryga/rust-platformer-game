use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("Timeless", "Filip Pietryga")
        .build()
        .expect("Could not create the context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = Timeless::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct Timeless {
    // Your state here...
}

impl Timeless {
    pub fn new(_ctx: &mut Context) -> Timeless {
        // Load/create resources such as images here.
        Timeless {
            // ...
        }
    }
}

impl EventHandler for Timeless {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Draw code here...
        canvas.finish(ctx)
    }
}