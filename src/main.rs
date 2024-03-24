use ggez::{conf::{self}, glam::{self, *}, input::{keyboard::{KeyCode, KeyInput, KeyMods}}, timer, Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Image};
use ggez::event::{self, EventHandler};
use std::{env, path};
use crate::f32::Vec2;
use rand::Rng;

//SETUP

//PLAYER DATA INITIALIZATION
const PLAYER_INITIAL_X: f64 = 45.0;
const PLAYER_INITIAL_Y: f64 = 0.0;
const PLAYER_INITIAL_DIRECTION: f64 = 1.0;
const PLAYER_INITIAL_STANDING: bool = false;
const PLAYER_INITIAL_VERTICAL_SPEED: f64 = 0.0;
const PLAYER_INITIAL_HORIZONTAL_SPEED: f64 = 0.0;
const PLAYER_INITIAL_IMAGE: &str = PLAYER_IMAGE_RIGHT;
const PLAYER_IMAGE_RIGHT: &str = "/player_right.png";
const PLAYER_IMAGE_LEFT: &str = "/player_left.png";
const PLAYER_INIITAL_HORIZONTAL_MOVEMENT_SPEED: f64 = 160.0;
const PLAYER_INITIAL_RUNNING_RATE_CONSTANT: f64 = 2.5;
const PLAYER_INITIAL_JUMP_SPEED: f64 = -280.0;
const PLAYER_INITIAL_COLLIDES_LEFT: bool = false;
const PLAYER_INITIAL_COLLIDES_RIGHT: bool = false;
//const HORIZONTAL_DECELERATION_RATE: f32 = 1.0; <-- sliding mechanique
const INITIAL_CUMULATIVE_HORIZONTAL_MOVEMENT: f64 = 0.0;

//MAX SPEED
//const MAXIMAL_HORIZONTAL_SPEED_LIMIT_CONSTANT: f64 = 500.0;
const MAX_SPEED_VERTICAL_CONSTANT: f64 = 250.0;

//WORLD DATA INITIALIZATION
const INITIAL_WORLD_SPEED_MULTIPLIER: f64 = 1.0;
const GRAVITY_CONSTANT: f64 = 420.0;

//KEYSTROKES
const LEFT_KEY: KeyCode = KeyCode::Left;
const RIGHT_KEY: KeyCode = KeyCode::Right;
const RUNNING_KEY: KeyMods = KeyMods::SHIFT;

//BLOCK
const INITIAL_BLOCK_IMAGE: &str = "/block_one.png";
const INITIAL_BLOCK_COUNTER: f64 = 128.0;
const INITIAL_BLOCK_ID: f64 = 9.0;

//WALL
const WALL_SPEED_CONSTANT: f64 = 40.0;
const WALL_SPEED_ACCELERATED_CONSTANT: f64 = 1800.0;
const INITIAL_WALL_PLACEMENT: f64 = -1200.0;
const WALL_IMAGE_CONSTANT: &str = "/wall2.png";

//WORLD GENERATION
const BLOCK_COUNTER_CONSTANT: f64 = 128.0;

//BULLET
const BULLET_IMAGE_CONSTANT: &str = "/bullet.png";

//THE STATE OF PLAYER
#[derive(Copy, Clone, Debug)]
struct Player {
    pos_x: f64,
    pos_y: f64,
    direction: f64,
    standing: bool,
    vertical_speed: f64,
    horizontal_speed: f64,
    collides_right: bool,
    collides_left: bool
}

// Implement methods for the Player struct
impl Player {
    // Constructor method to create a new Player instance
    fn new(pos_x: f64, pos_y: f64, direction: f64, standing: bool, vertical_speed: f64, horizontal_speed: f64, collides_right: bool, collides_left: bool) -> Self {
        Player {
            pos_x,
            pos_y,
            direction,
            standing,
            vertical_speed,
            horizontal_speed,
            collides_right, 
            collides_left
        }
    }

    // Method to display player information
    fn _describe(&self) {
        println!("pos_x: {}", self.pos_x);
        println!("pos_y: {}", self.pos_y);
    }

    pub fn move_horizontally(&mut self, speed:f64) {
        self.pos_x += speed;
    }
    
    pub fn reset(&mut self) {
        self.pos_x = PLAYER_INITIAL_X;
        self.pos_y = PLAYER_INITIAL_Y;
        self.direction = PLAYER_INITIAL_DIRECTION;
        self.standing = PLAYER_INITIAL_STANDING;
        self.horizontal_speed = PLAYER_INITIAL_HORIZONTAL_SPEED;
        self.vertical_speed = PLAYER_INITIAL_VERTICAL_SPEED;
        self.collides_left = PLAYER_INITIAL_COLLIDES_LEFT;
        self.collides_right = PLAYER_INITIAL_COLLIDES_RIGHT;
    }
}

struct Block {
    rect: Rectangle,
    image: Image
}

// THE BASIC BUILDING BLOCK OF THE WORLD
// Implement methods for the Block struct
impl Block {
    // Constructor method to create a new Block instance
    fn new(rect: Rectangle, image: Image) -> Self {
        Block {
            rect,
            image
        }
    }

    // Method to display player information
    fn _describe(&self) {
        println!("pos_x: {}", self.rect.x);
        println!("pos_y: {}", self.rect.y);
    }
}

//FOR COLLISION DETECTION
#[derive(Debug, Copy, Clone)]
struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    collision_direction: Option<CollisionDirection>,
}

//FOR COLLISION DETECTION (HORIZONTAL)
#[derive(Debug, Copy, Clone)]
enum CollisionDirection {
    Left,
    Right,
}

//RECTANGLE CLASS, USED FOR COLLISION DETECTION
impl Rectangle {
    // Check if two rectangles intersect (collide)
    fn intersects(&self, other: &Rectangle) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }

    // Resolve the collision by adjusting player's position
    fn resolve_collision(&mut self, other: &Rectangle) {
        let dx = (self.x + self.width / 8.0) - (other.x + other.width / 8.0);
        let dy = (self.y + self.height / 8.0) - (other.y + other.height / 8.0);

        let (mut new_x, mut new_y) = (self.x, self.y);

        if dx.abs() > dy.abs() {
            if dx > 0.0 {
                new_x = other.x + other.width;
            } else {
                new_x = other.x - self.width;
            }
        } else {
            if dy > 0.0 {
                new_y = other.y + other.height;
            } else {
                new_y = other.y - self.height;
            }
        }

        self.x = new_x;
        self.y = new_y;
    }

    fn intersects_horizontally(&self, other: &Rectangle) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }

    // Resolve the collision by adjusting player's position horizontally
    fn resolve_horizontal_collision(&mut self, other: &Rectangle) {
        let overlap_left = (self.x + self.width) - other.x;
        let overlap_right = (other.x + other.width) - self.x;

        if overlap_left.abs() < overlap_right.abs() {
            // Resolve collision from the left
            self.x -= overlap_left;
            self.collision_direction = Some(CollisionDirection::Left);
        } else {
            // Resolve collision from the right
            self.x += overlap_right;
            self.collision_direction = Some(CollisionDirection::Right);
        }
    }

}

struct Bullet {
    speed: f64,
    x: f64,
    y: f64,
}

// Game State
struct Timeless {
    player: Player,
    speed: f64,
    player_image: Image,
    blocks: Vec<Block>,
    cumulative_horizontal_movement: f64,
    wall_x: f64,
    wall_image: Image,
    block_counter: f64,
    block_id: f64,
    wall_speed: f64,
    bullet_image: Image
}

impl Timeless {
    // Initial State of the world
    fn new(ctx: &mut Context) -> Timeless {
        let player = Player::new(
            PLAYER_INITIAL_X, PLAYER_INITIAL_Y, PLAYER_INITIAL_DIRECTION, PLAYER_INITIAL_STANDING, PLAYER_INITIAL_VERTICAL_SPEED, PLAYER_INITIAL_HORIZONTAL_SPEED, false, false
        );
        let speed = INITIAL_WORLD_SPEED_MULTIPLIER;
        let player_image = Image::from_path(ctx, PLAYER_INITIAL_IMAGE).unwrap();

        let blocks: Vec<Block> = vec![
            Block { rect: Rectangle{x: 0.0, y: 320.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(ctx, INITIAL_BLOCK_IMAGE).unwrap()},
            Block { rect: Rectangle{x: 128.0, y: 320.0 + 128.0 / 2.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(ctx, INITIAL_BLOCK_IMAGE).unwrap()},
            Block { rect: Rectangle{x: 128.0*2.0, y: 320.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(ctx, INITIAL_BLOCK_IMAGE).unwrap()},
            Block { rect: Rectangle{x: 128.0*3.0, y: 320.0 + 128.0 / 2.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(ctx, INITIAL_BLOCK_IMAGE).unwrap()},
            Block { rect: Rectangle{x: 128.0*4.0, y: 320.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(ctx, INITIAL_BLOCK_IMAGE).unwrap()},
            Block { rect: Rectangle{x: 128.0*5.0, y: 320.0 + 128.0 / 2.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(ctx, INITIAL_BLOCK_IMAGE).unwrap()},
            Block { rect: Rectangle{x: 128.0*6.0, y: 320.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(ctx, INITIAL_BLOCK_IMAGE).unwrap()},
            Block { rect: Rectangle{x: 128.0*7.0, y: 320.0 + 128.0 / 2.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(ctx, INITIAL_BLOCK_IMAGE).unwrap()},
            Block { rect: Rectangle{x: 128.0*8.0, y: 320.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(ctx, INITIAL_BLOCK_IMAGE).unwrap()},
        ];

        let wall_image = Image::from_path(ctx, WALL_IMAGE_CONSTANT).unwrap();
        let bullet_image = Image::from_path(ctx, BULLET_IMAGE_CONSTANT).unwrap();

        Timeless { player, speed, player_image, blocks, cumulative_horizontal_movement: INITIAL_CUMULATIVE_HORIZONTAL_MOVEMENT, wall_x: INITIAL_WALL_PLACEMENT, wall_image: wall_image, block_counter: INITIAL_BLOCK_COUNTER, block_id: INITIAL_BLOCK_ID, wall_speed: WALL_SPEED_CONSTANT, bullet_image }
    }

    pub fn reset(&mut self, ctx: &mut Context) {
        let player = Player::new(
            PLAYER_INITIAL_X, PLAYER_INITIAL_Y, PLAYER_INITIAL_DIRECTION, PLAYER_INITIAL_STANDING, PLAYER_INITIAL_VERTICAL_SPEED, PLAYER_INITIAL_HORIZONTAL_SPEED, PLAYER_INITIAL_COLLIDES_RIGHT, PLAYER_INITIAL_COLLIDES_LEFT
        );
        let speed = INITIAL_WORLD_SPEED_MULTIPLIER;
        let player_image = Image::from_path(ctx, PLAYER_INITIAL_IMAGE).unwrap();

        let blocks: Vec<Block> = vec![
            Block { rect: Rectangle{x: 0.0, y: 320.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(ctx, INITIAL_BLOCK_IMAGE).unwrap()},
            Block { rect: Rectangle{x: 128.0, y: 320.0 + 128.0 / 2.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(ctx, INITIAL_BLOCK_IMAGE).unwrap()},
            Block { rect: Rectangle{x: 128.0*2.0, y: 320.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(ctx, INITIAL_BLOCK_IMAGE).unwrap()},
            Block { rect: Rectangle{x: 128.0*3.0, y: 320.0 + 128.0 / 2.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(ctx, INITIAL_BLOCK_IMAGE).unwrap()},
            Block { rect: Rectangle{x: 128.0*4.0, y: 320.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(ctx, INITIAL_BLOCK_IMAGE).unwrap()},
            Block { rect: Rectangle{x: 128.0*5.0, y: 320.0 + 128.0 / 2.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(ctx, INITIAL_BLOCK_IMAGE).unwrap()},
            Block { rect: Rectangle{x: 128.0*6.0, y: 320.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(ctx, INITIAL_BLOCK_IMAGE).unwrap()},
            Block { rect: Rectangle{x: 128.0*7.0, y: 320.0 + 128.0 / 2.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(ctx, INITIAL_BLOCK_IMAGE).unwrap()},
            Block { rect: Rectangle{x: 128.0*8.0, y: 320.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(ctx, INITIAL_BLOCK_IMAGE).unwrap()},
        ];

        let wall_image = Image::from_path(ctx, WALL_IMAGE_CONSTANT).unwrap();
        let bullet_image = Image::from_path(ctx, BULLET_IMAGE_CONSTANT).unwrap();
        
        self.player = player;
        self.player_image = player_image;
        self.blocks = blocks;
        self.wall_image = wall_image;
        self.bullet_image = bullet_image;
        self.speed = speed;
        self.wall_x = INITIAL_WALL_PLACEMENT;
        self.wall_speed = WALL_SPEED_CONSTANT;
        self.cumulative_horizontal_movement = INITIAL_CUMULATIVE_HORIZONTAL_MOVEMENT;
        self.block_id = INITIAL_BLOCK_ID;
        self.block_counter = INITIAL_BLOCK_COUNTER;
    }
}

//event handler for the game
impl EventHandler for Timeless {
    // Update loop
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        //self.player.describe();
        //self.camera_x = self.player.pos_x - (SCREEN_WIDTH / 2.0);

        let k_ctx = &_ctx.keyboard;
        // Increase or decrease `position_x` by 0.5, or by 5.0 if Shift is held.

        //<KEYSTROKES IN EVENT HANDLER>
        //MOVEMENT LEFT-RIGHT WITH SPRINT
        if k_ctx.is_key_pressed(RIGHT_KEY) && !self.player.collides_right {
            self.player.direction = 1.0;
            self.player_image = Image::from_path(_ctx, PLAYER_IMAGE_RIGHT).unwrap();
            if k_ctx.is_mod_active(RUNNING_KEY) {
                //self.player.horizontal_speed += self.speed * HORIZONTAL_SPEED_CONSTANT * self.player.direction * RUNNING_CONSTANT;
                self.player.horizontal_speed = self.speed * PLAYER_INIITAL_HORIZONTAL_MOVEMENT_SPEED * self.player.direction * PLAYER_INITIAL_RUNNING_RATE_CONSTANT;
            } else {
                //self.player.horizontal_speed += self.speed * HORIZONTAL_SPEED_CONSTANT * self.player.direction;
                self.player.horizontal_speed = self.speed * PLAYER_INIITAL_HORIZONTAL_MOVEMENT_SPEED * self.player.direction;
            }
        } else if k_ctx.is_key_pressed(LEFT_KEY) && !self.player.collides_left {
            self.player.direction = -1.0;
            self.player_image = Image::from_path(_ctx, PLAYER_IMAGE_LEFT).unwrap();
            if k_ctx.is_mod_active(RUNNING_KEY) {
                //self.player.horizontal_speed += self.speed * HORIZONTAL_SPEED_CONSTANT * self.player.direction * RUNNING_CONSTANT;
                self.player.horizontal_speed = self.speed * PLAYER_INIITAL_HORIZONTAL_MOVEMENT_SPEED * self.player.direction * PLAYER_INITIAL_RUNNING_RATE_CONSTANT;
            } else {
                //self.player.horizontal_speed += self.speed * HORIZONTAL_SPEED_CONSTANT * self.player.direction;
                self.player.horizontal_speed = self.speed * PLAYER_INIITAL_HORIZONTAL_MOVEMENT_SPEED * self.player.direction;
            }
        }

        //JUMP TRIGGER
        if k_ctx.is_key_pressed(KeyCode::Z) {
            println!("Tried to Jump!");
            if self.player.standing {
                println!("Jumped!");
                self.player.vertical_speed = PLAYER_INITIAL_JUMP_SPEED;
                self.player.standing = false;
            }
        }

        //JUMP TRIGGER
        if k_ctx.is_key_just_pressed(KeyCode::X) {
            println!("SHOT!");
        }

        //</KEYSTROKES IN EVENT HANDLER>

        //PREVIOUS SPEED LIMITS MECHANIQUE
        /*if self.player.horizontal_speed > MAXIMAL_HORIZONTAL_SPEED_LIMIT_CONSTANT {
            self.player.horizontal_speed = MAXIMAL_HORIZONTAL_SPEED_LIMIT_CONSTANT;
        }

        if self.player.horizontal_speed < -MAXIMAL_HORIZONTAL_SPEED_LIMIT_CONSTANT {
            self.player.horizontal_speed = -MAXIMAL_HORIZONTAL_SPEED_LIMIT_CONSTANT;
        }*/

        //HORIZONTAL MOVEMENT UPDATE

        //<CALCULATIONS FOR THE HORIZONTAL COLLISION>
        let mut player_collider = Rectangle { x: self.player.pos_x, y: self.player.pos_y, width: 64.0, height: 56.0, collision_direction: Some(CollisionDirection::Right) };

        for item in self.blocks.iter() {

            let mut x_2 = item.rect.x;
            if self.player.pos_x > 320.0 {
                x_2 = item.rect.x - self.player.pos_x + 320.0 - self.cumulative_horizontal_movement;
            }
            let rect = Rectangle{x: x_2, y: item.rect.y, width: item.rect.width, height: item.rect.height, collision_direction: Some(CollisionDirection::Right)};

            if player_collider.intersects_horizontally(&rect) {
                println!("Horizontal collision detected!");
                player_collider.resolve_horizontal_collision(&rect); // Adjust player's position
                
                if let Some(direction) = player_collider.collision_direction {
                    match direction {
                        CollisionDirection::Left => {
                            println!("Collision is on the left side of the obstacle");
                            if self.player.horizontal_speed > 0.0 {
                                self.player.horizontal_speed = 0.0;
                            }
                            self.player.pos_x=player_collider.x - 1.0 * timer::delta(_ctx).as_secs_f64();
                            self.player.collides_right = true;
                        },
                        CollisionDirection::Right => {
                            println!("Collision is on the right side of the obstacle");
                            if self.player.horizontal_speed < 0.0 {
                                self.player.horizontal_speed = 0.0;
                            }
                            self.player.pos_x = player_collider.x + 1.0 * timer::delta(_ctx).as_secs_f64();
                            self.player.collides_left = true;
                        },
                    }
                }
            }
        }

        //</CALCULATIONS FOR THE HORIZONTAL COLLISION>

        
        //<REFRESH FOR THE RIGHT/LEFT COLLISION CHECKS>
        if self.player.collides_left || self.player.collides_right {
            let player_collider = Rectangle { x: self.player.pos_x, y: self.player.pos_y, width: 64.0, height: 56.0, collision_direction: Some(CollisionDirection::Right) };
            let mut collision = false;
            for item in self.blocks.iter() {

                let mut x_2 = item.rect.x;
                if self.player.pos_x >= 320.0 {
                    x_2 = item.rect.x-self.player.pos_x + 320.0 - self.cumulative_horizontal_movement;
                }
                let rect = Rectangle{x: x_2, y: item.rect.y, width: item.rect.width, height: item.rect.height, collision_direction: Some(CollisionDirection::Right)};

                if player_collider.intersects_horizontally(&rect) {
                    collision = true;
                    println!("Horizontal collision detected!");
                }
            }
            if !collision {
                self.player.collides_left = false;
                self.player.collides_right = false;
            }
        }
        //</REFRESH FOR THE RIGHT/LEFT COLLISION CHECKS>


        //<APPLY THE MOVEMENT AND CLIP IT WHEN EXCEEDS 320>
        if self.player.pos_x < 320.0 {
            self.player.move_horizontally(self.player.horizontal_speed * timer::delta(_ctx).as_secs_f64());
            self.block_counter -= self.player.horizontal_speed * timer::delta(_ctx).as_secs_f64();
        } else {
            self.cumulative_horizontal_movement += self.player.horizontal_speed * timer::delta(_ctx).as_secs_f64();
            self.block_counter -= self.player.horizontal_speed * timer::delta(_ctx).as_secs_f64();
        }
        self.player.horizontal_speed = 0.0;
        //</APPLY THE MOVEMENT AND CLIP IT WHEN EXCEEDS 320>


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
        
        //<GRAVITY>
        if !self.player.standing {
            self.player.vertical_speed += GRAVITY_CONSTANT * timer::delta(_ctx).as_secs_f64();
        }
        //</GRAVITY>

        //<LIMIT THE FALLING SPEED>
        if self.player.vertical_speed > MAX_SPEED_VERTICAL_CONSTANT {
            self.player.vertical_speed = MAX_SPEED_VERTICAL_CONSTANT;
        }
        //</LIMIT THE FALLING SPEED>


        //println!("{}", self.player.pos_y);
        //collision


        //<CHECK IF THE CHARACTER FELL ON THE PLATORM>
        if !self.player.standing {

            let mut player_collider = Rectangle { x: self.player.pos_x+11.0, y: self.player.pos_y, width: 42.0, height: 64.0, collision_direction: Some(CollisionDirection::Right) };

            for item in self.blocks.iter() {
                let platform_collider = item.rect;

                let mut x_2 = item.rect.x;
                if self.player.pos_x >= 320.0 {
                    x_2 = item.rect.x-self.player.pos_x+320.0 - self.cumulative_horizontal_movement;
                }
                let rect = Rectangle{x: x_2, y: item.rect.y, width: item.rect.width, height: item.rect.height, collision_direction: Some(CollisionDirection::Right)};

                if player_collider.intersects(&rect) {
                    //println!("Player collided with platform!");
                    
                    if self.player.vertical_speed > 0.0 {
                        self.player.vertical_speed = 0.0;
                        self.player.standing = true;
                    }
                    player_collider.resolve_collision(&rect); 
                } 
            }
        }
        //</CHECK IF THE CHARACTER FELL ON THE PLATORM>


        //<CHECK IF THE CHARACTER IS ON AIR>
        if self.player.standing {
            let mut player_collider = Rectangle { x: self.player.pos_x, y: self.player.pos_y, width: 64.0, height: 64.0, collision_direction: Some(CollisionDirection::Right) };
            let mut anyblock: bool = false;
            for item in self.blocks.iter() {

                let mut x_2 = item.rect.x;
                if self.player.pos_x >= 320.0 {
                    x_2 = item.rect.x-self.player.pos_x+320.0 - self.cumulative_horizontal_movement;
                }
                let rect = Rectangle{x: x_2, y: item.rect.y, width: item.rect.width, height: item.rect.height, collision_direction: Some(CollisionDirection::Right)};

                if player_collider.intersects(&rect) {
                    //println!("Player collided with platform!");
                    anyblock = true;
                    player_collider.resolve_collision(&rect); 
                } 
            }
            if !anyblock {
                self.player.standing = false;
            }
        }
        //</CHECK IF THE CHARACTER IS ON AIR>


        //FALL CALCULATION
        self.player.pos_y += self.speed * self.player.vertical_speed * timer::delta(_ctx).as_secs_f64();


        //THE WALL MOVEMENT
        println!("{}", -(self.wall_x - self.player.pos_x + 1000.0));
        if self.wall_x - self.player.pos_x + 1000.0 > 0.0 { 
            self.wall_speed = WALL_SPEED_ACCELERATED_CONSTANT;
        } else {
            self.wall_speed = WALL_SPEED_CONSTANT;
        }

        //THE WALL "EATING" MECHANIQUE
        if(-(self.wall_x - self.player.pos_x + 1000.0) < -3000.0) {
            self.wall_speed = 0.0;
        } else {
            self.wall_x += self.wall_speed * timer::delta(_ctx).as_secs_f64() * self.speed;
        }


        //WORLD GENERATION
        if self.block_counter <= 0.0 {
            let num = rand::thread_rng().gen_range(0..100);
            if num < 33 {
                self.blocks.push(Block { rect: Rectangle{x: 128.0*self.block_id, y: 320.0 + 128.0 / 2.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(_ctx, INITIAL_BLOCK_IMAGE).unwrap()});
            }
            else if num < 66 {
                self.blocks.push(Block { rect: Rectangle{x: 128.0*self.block_id, y: 320.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(_ctx, INITIAL_BLOCK_IMAGE).unwrap()});
            }
            else if num < 100 {
                self.blocks.push(Block { rect: Rectangle{x: 128.0*self.block_id, y: 320.0 + 128.0 / 3.0, width: 128.0, height: 128.0, collision_direction: Some(CollisionDirection::Right)}, image: Image::from_path(_ctx, INITIAL_BLOCK_IMAGE).unwrap()});
            }
            self.block_id += 1.0;
            self.block_counter = BLOCK_COUNTER_CONSTANT;
        }

        Ok(())
    }

    // Draw loop
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //INITIALIZE THE CANVAS
        let mut canvas: graphics::Canvas = graphics::Canvas::from_frame(ctx,Color::BLACK);

        // DRAW THE PLAYER
        let dst = glam::Vec2::new(self.player.pos_x as f32, self.player.pos_y as f32);
        canvas.draw(&self.player_image, graphics::DrawParam::new().dest(dst));

        // DRAW EACH BLOCK
        for item in self.blocks.iter() {
            let mut x_2: f64 = item.rect.x;
            if self.player.pos_x >= 320.0 {
                x_2 = item.rect.x-self.player.pos_x+320.0 -self.cumulative_horizontal_movement;
            }
            let dst = glam::Vec2::new((x_2) as f32, (item.rect.y) as f32);
            canvas.draw(&item.image, graphics::DrawParam::new().dest(dst));
        }

        //DRAW THE WALL
        let dst: Vec2 = glam::Vec2::new(self.wall_x as f32, 0.0);
        canvas.draw(&self.wall_image, graphics::DrawParam::new().dest(dst));

        //DRAW THE BLACK SCREEN IF IT HAS EATEN THE PLAYER
        if self.wall_x - self.player.pos_x + 600.0 > 0.0 {
            let rect = graphics::Rect::new(0.0, 0.0, 640.0, 480.0);
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest(rect.point())
                    .scale(rect.size())
                    .color(Color::BLACK),
            );
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::Q) => {
                println!("Terminating!");
                ctx.request_quit();
            },
            Some(KeyCode::R) => {
                println!("Reseting!");
                //todo!("re-initialize the game when pressed R");
                self.player.reset();
                self.reset(ctx);
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