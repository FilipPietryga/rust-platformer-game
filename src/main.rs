use ggez::{conf::{self, NumSamples}, glam::{self, *}, input::{keyboard::{KeyCode, KeyInput, KeyMods}, mouse::delta}, timer, Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Image};
use ggez::event::{self, EventHandler};
use num_traits::abs;
use std::{env, path, slice::Windows};
use crate::f32::Vec2;

//SETUP

//PLAYER DATA INITIALIZATION
const INITIAL_X: f32 = 0.0;
const INITIAL_Y: f32 = 0.0;
const INITIAL_DIRECTION: f32 = 1.0;
const INITIAL_PLAYER_IMAGE: &str = PLAYER_IMAGE_RIGHT;
const PLAYER_IMAGE_RIGHT: &str = "/player_right.png";
const PLAYER_IMAGE_LEFT: &str = "/player_left.png";
const PLAYER_HORIZONTAL_MOVEMENT_SPEED: f32 = 160.0;
const RUNNING_CONSTANT: f32 = 2.5;
//const HORIZONTAL_DECELERATION_RATE: f32 = 1.0; <-- sliding mechanique

//MAX SPEED
const MAXIMAL_HORIZONTAL_SPEED_LIMIT_CONSTANT: f32 = 500.0;
const MAXIMAL_VERTICAL_SPEED_LIMIT_CONSTANT: f32 = 200.0;

//WORLD DATA INITIALIZATION
const INITIAL_WORLD_SPEED_MULTIPLIER: f32 = 1.0;
const GRAVITY_CONSTANT: f32 = 30.0;

//KEYSTROKES
const LEFT_KEY: KeyCode = KeyCode::Left;
const RIGHT_KEY: KeyCode = KeyCode::Right;
const RUNNING_KEY: KeyMods = KeyMods::SHIFT;

//THE STATE OF PLAYER
struct Player {
    pos_x: f32,
    pos_y: f32,
    direction: f32,
    image: Image,
    standing: bool,
    vertical_speed: f32,
    horizontal_speed: f32,
}

// Implement methods for the Player struct
impl Player {
    // Constructor method to create a new Player instance
    fn new(pos_x: f32, pos_y: f32, direction: f32, image: Image, standing: bool, vertical_speed: f32, horizontal_speed: f32) -> Self {
        Player {
            pos_x,
            pos_y,
            direction,
            image,
            standing,
            vertical_speed,
            horizontal_speed
        }
    }

    // Method to display player information
    fn _describe(&self) {
        println!("pos_x: {}", self.pos_x);
        println!("pos_y: {}", self.pos_y);
    }

    pub fn move_horizontally(&mut self, speed:f32) {
        self.pos_x += speed;
    }
}

// Game State
struct Timeless {
    player: Player,
    speed: f32,
}

impl Timeless {
    // Initial State of the world
    fn new(ctx: &mut Context) -> Timeless {
        let player = Player::new(
            INITIAL_X, INITIAL_Y, INITIAL_DIRECTION, Image::from_path(ctx, INITIAL_PLAYER_IMAGE).unwrap(), false, 0.0, 0.0
        );
        let speed = INITIAL_WORLD_SPEED_MULTIPLIER;
    
        Timeless { player, speed }
    }
}

//event handler for the game
impl EventHandler for Timeless {
    // Update loop
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        //self.player.describe();

        let k_ctx = &_ctx.keyboard;
        // Increase or decrease `position_x` by 0.5, or by 5.0 if Shift is held.

        //MOVEMENT LEFT-RIGHT WITH SPRINT
        if k_ctx.is_key_pressed(RIGHT_KEY) {
            self.player.direction = 1.0;
            self.player.image = Image::from_path(_ctx, PLAYER_IMAGE_RIGHT).unwrap();
            if k_ctx.is_mod_active(RUNNING_KEY) {
                //self.player.horizontal_speed += self.speed * HORIZONTAL_SPEED_CONSTANT * self.player.direction * RUNNING_CONSTANT;
                self.player.horizontal_speed = self.speed * PLAYER_HORIZONTAL_MOVEMENT_SPEED * self.player.direction * RUNNING_CONSTANT;
            } else {
                //self.player.horizontal_speed += self.speed * HORIZONTAL_SPEED_CONSTANT * self.player.direction;
                self.player.horizontal_speed = self.speed * PLAYER_HORIZONTAL_MOVEMENT_SPEED * self.player.direction;
            }
        } else if k_ctx.is_key_pressed(LEFT_KEY) {
            self.player.direction = -1.0;
            self.player.image = Image::from_path(_ctx, PLAYER_IMAGE_LEFT).unwrap();
            if k_ctx.is_mod_active(RUNNING_KEY) {
                //self.player.horizontal_speed += self.speed * HORIZONTAL_SPEED_CONSTANT * self.player.direction * RUNNING_CONSTANT;
                self.player.horizontal_speed = self.speed * PLAYER_HORIZONTAL_MOVEMENT_SPEED * self.player.direction * RUNNING_CONSTANT;
            } else {
                //self.player.horizontal_speed += self.speed * HORIZONTAL_SPEED_CONSTANT * self.player.direction;
                self.player.horizontal_speed = self.speed * PLAYER_HORIZONTAL_MOVEMENT_SPEED * self.player.direction;
            }
        }

        //PREVIOUS SPEED LIMITS MECHANIQUE
        /*if self.player.horizontal_speed > MAXIMAL_HORIZONTAL_SPEED_LIMIT_CONSTANT {
            self.player.horizontal_speed = MAXIMAL_HORIZONTAL_SPEED_LIMIT_CONSTANT;
        }

        if self.player.horizontal_speed < -MAXIMAL_HORIZONTAL_SPEED_LIMIT_CONSTANT {
            self.player.horizontal_speed = -MAXIMAL_HORIZONTAL_SPEED_LIMIT_CONSTANT;
        }*/

        //HORIZONTAL MOVEMENT UPDATE
        self.player.move_horizontally(self.player.horizontal_speed * timer::delta(_ctx).as_secs_f32());
        self.player.horizontal_speed = 0.0;

        //DECELERATION MECHANIQUE
        /*if self.player.horizontal_speed > 0.0 {
            self.player.horizontal_speed -= HORIZONTAL_DECELERATION_RATE;
        }
        if self.player.horizontal_speed < 0.0 {
            self.player.horizontal_speed += HORIZONTAL_DECELERATION_RATE;
        }
        
        if abs(self.player.horizontal_speed) < 1.0 {
            self.player.horizontal_speed = 0.0;
        }*/
        
        //GRAVITY
        if !self.player.standing {
            self.player.vertical_speed += GRAVITY_CONSTANT;
        }

        if self.player.vertical_speed < -MAXIMAL_VERTICAL_SPEED_LIMIT_CONSTANT {
            self.player.vertical_speed = -MAXIMAL_HORIZONTAL_SPEED_LIMIT_CONSTANT;
        }
        
        //fall
        self.player.pos_y += self.speed * self.player.vertical_speed * timer::delta(_ctx).as_secs_f32();



        println!("{}", self.player.pos_y);

        Ok(())
    }

    // Draw loop
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas: graphics::Canvas = graphics::Canvas::from_frame(ctx,Color::BLACK);

        // Draw an image.
        let dst = glam::Vec2::new(self.player.pos_x, self.player.pos_y);
        canvas.draw(&self.player.image, graphics::DrawParam::new().dest(dst));
        

        // Draw a stroked rectangle mesh.
        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::Q) => {
                println!("Terminating!");
                ctx.request_quit();
            },
            Some(KeyCode::Z) => {
                println!("Jump!");
            },
            Some(KeyCode::X) => {
                println!("One shot!");
            },
            Some(KeyCode::C) => {
                println!("Time attack!");
            },
            _ => (),
        }
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

    let cb = ContextBuilder::new("Timeless", "Filip Pietryga")
        .window_setup(conf::WindowSetup::default().title("Timeless!"))
        .window_mode(conf::WindowMode::default().dimensions(640.0, 480.0))
        .add_resource_path(resource_dir);

    let (mut ctx, event_loop) = cb.build()
        .expect("Could not create the context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let game = Timeless::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, game);
}