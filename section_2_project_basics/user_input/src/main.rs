// Import libraries
use std::io;

/// This is a documentation comment, which is used for doc generation with cargo

fn main() {
    // Create a mutable object through the mut directive
    let mut input = String::new();

    // println! is a macro (coming up later)
    println!("Say something");

    // Receive input into the mutable variable input (a reference to it)
    match io::stdin().read_line(&mut input){
        // Check if match gives back an Ok or an Err
        Ok(_) => {
            println!("You said: {}", input);
        },
        Err(e) => {
            println!("Something went wrong {}", e);
        }
    }
}
