use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};
use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

// Color for food
const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];   // RGB(A)

const MOVING_PERIOD: f64 = 0.1;

pub struct Game {
    snake: Snake,
    // Flag for food existing and some food position vars
    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    waiting_time: f64,
}

// Implement functionality for the Game struct
impl Game {
    // Constructor
    pub fn new(width: i32, height: i32) -> Game {
    Game {
        snake: Snake::new(2, 2),
        food_exists: false,
        food_x: 0,
        food_y: 0,
        width,
        height,
        waiting_time: 0.0,
        }
    }

    // Make snake move with some input
    pub fn key_pressed(&mut self, key: Key) {
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction())
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // Draw the snake
        self.snake.draw(con, g);
        // If food exists, draw it on board
        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        // Top, bottom, left, right
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);
    }

    pub fn update(&mut self, delta_time: f64){
        // How long to wait for the snake to move to the new position
        self.waiting_time += delta_time;

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    // Check if the snake has reached the apple
    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();

        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn add_food(&mut self) {
        // parameters for food position on board
        let mut rnd = thread_rng();
        let mut new_x = rnd.gen_range(1, self.width - 1);
        let mut new_y = rnd.gen_range(1, self.height - 1);

        self.food_x = new_x;
        self.food_y = new_y;

        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        self.snake.move_forward(dir);
        self.check_eating();
        self.waiting_time = 0.0;
    }
}