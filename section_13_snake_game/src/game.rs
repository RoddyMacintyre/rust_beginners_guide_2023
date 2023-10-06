use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};
use crate::draw::{draw_block, draw_rectangle};

// Color for food
const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];   // RGB(A)

pub struct Game {
    // Flag for food existing and some food position vars
    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32
}

// Implement functionality for the Game struct
impl Game {
    // Constructor
    pub fn new(width: i32, height: i32) -> Game {
    Game {
        food_exists: false,
        food_x: 0,
        food_y: 0,
        width,
        height
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
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
        if !self.food_exists {
            self.add_food();
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
}