/*
ERROR HANDLING

- Working with files
- Errors
- Helper methods
- ? Operator

WORKING WITH FILES
    can handle error explicitly with the expect() method

 */

use std::fs::{File, OpenOptions, remove_file};
use std::io::{Read, Write};

#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    // ===== WORKING WITH FILES =====

    // File creation
    // let mut file = File::create("src/example.txt").expect("Create failed...");
    // // Write to a file. It overrides the contents of the file.
    // // Need append to add to existing content
    // file.write_all("Hello World!\n".as_bytes()).expect("Write failed...");
    // Append content to a file
    let mut file = OpenOptions::new()
        .append(true)
        .open("src/example.txt")
        .expect("Cannot open file");

    // file.write_all("Adding content to the file\n".as_bytes()).expect("Write failed...");

    // Read from a file
    // Uses unwrap() method which is also an error handling method.
    // It either returns the correct value or raise an error
    let mut file = File::open("src/example.txt").unwrap();
    // Place file contents in a variable
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);

    // Delete a file
    remove_file("src/example.txt").expect("Delete failed...");
}
