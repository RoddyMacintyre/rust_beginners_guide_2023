// Import libraries
use std::io;

/// This is a documentation comment, which is used for doc generation with cargo

fn main() {
    //! This is a function comment for doc generation with cargo

    //! # This is a heading comment for cargo
    //! ```
    //! fn main()
    //! ```
    //! Reads user input and prints it to the console
    //!
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
