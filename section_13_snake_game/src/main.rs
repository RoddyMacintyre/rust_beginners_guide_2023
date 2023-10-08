/*
Snake game
    - Algorithms for game mechanics
    - GUI library

    Rules:
        - Snake can change direction, but not into the opposite of the current direction
        - Snake can move within the box (no touching walls)
        - Snake cannot move over its own tail
        - Grow snake when it eats an apple
        - Apple appears randomly on screen after eaten
        - Restart game after 1s when game over
 */

extern crate piston_window;
extern crate rand;

// Add draw.rs and game.rs as modules in this file
mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;
use crate::draw::to_coord_u32;
use crate::game::Game;

// Define a color
const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];     // RGB(A)

fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow = WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        // Add key press event to the game
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
