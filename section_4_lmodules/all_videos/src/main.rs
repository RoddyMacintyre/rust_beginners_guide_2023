// Import player module
mod player;
// Import crate
use crate::archive::arch::arch_file as arc;
mod archive;

// Use external crate:
use rand::{Rng, thread_rng};  // Random Number Generator
use rand::distributions::Alphanumeric;

fn main() {
    player::play_movie("Snatch.mp4");
    player::play_audio("rhcp.mp3");

    clean::perform_cleanup();
    // Nested module call
    clean::files::clean_files();

    // Access crate functionalit
    arc("somefile.txt");

    let mut rng = rand::thread_rng();
    let a: i32 = rng.gen();
    println!("{}", a);

    // Bounded int
    let b: i32 = rng.gen_range(0, 10);
    println!("bounded int: {}", b);

    // Bounded float
    println!("Bounded float: {}", rng.gen_range(0.0, 100.0));

    // String generation
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();
    println!("Gen string: {}", rand_string);
}

// Can encapsulate modules within a file as well. Can also nest modules.
mod clean{
    pub fn perform_cleanup(){
        println!("Cleaning hdd");
    }

    pub mod files{
        pub fn clean_files(){
            println!("Removing unused files");
        }
    }
}
