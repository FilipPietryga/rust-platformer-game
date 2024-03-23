use ggez::{glam::{self, *}, Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Image};
use ggez::event::{self, EventHandler};
use std::{env, path};
use crate::f32::Vec2;

struct Player {
    pos_x: f32,
    pos_y: f32,
    speed: f32,
    image: Image,
}

// Implement methods for the Player struct
impl Player {
    // Constructor method to create a new Player instance
    fn new(pos_x: f32, pos_y: f32, speed: f32, image: Image) -> Self {
        Player {
            pos_x,
            pos_y,
            speed,
            image
        }
    }

    // Method to display player information
    fn describe(&self) {
        println!("pos_x: {}", self.pos_x);
        println!("pos_y: {}", self.pos_y);
        println!("speed: {}", self.speed);
    }
}

// Game State
struct Timeless {
    player: Player,
}

impl Timeless {
    // Initial State of the world
    fn new(ctx: &mut Context) -> Timeless {
        let mut player = Player::new(
            0.0, 0.0, 5.0, Image::from_path(ctx, "/player.png").unwrap()
        );

        Timeless { player }
    }
}

impl EventHandler for Timeless {
    // Update loop
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        //self.player.describe();
        Ok(())
    }

    // Draw loop
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        // Draw an image.
        let dst = glam::Vec2::new(self.player.pos_x, self.player.pos_y);
        canvas.draw(&self.player.image, graphics::DrawParam::new().dest(dst));

        // Draw code here...
        let _ = canvas.finish(ctx);
        
        Ok(())
    }
}

// Main function
fn main() {

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("drawing", "ggez").add_resource_path(resource_dir);

    let (mut ctx, event_loop) = cb.build()
        .expect("Could not create the context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let game = Timeless::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, game);
}