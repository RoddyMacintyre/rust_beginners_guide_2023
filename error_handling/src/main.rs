/*
ERROR HANDLING

- Working with files
- Errors
- Helper methods
- ? Operator

WORKING WITH FILES
    can handle error explicitly with the expect() method

ERRORS
    2 types of errors:
        - Recoverable
            Program can continue functioning and we can manage/handle the error
            'Result' enum
        - Unrecoverable
            Program cannot continue functioning
            'panic!(message)' macro -- Stops execution of code in an uncontrolled manner

HELPER METHODS
    Unwrap:
        Returns the data if available, otherwise will panic

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
    // let mut file = OpenOptions::new()
    //     .append(true)
    //     .open("src/example.txt")
    //     .expect("Cannot open file");

    // file.write_all("Adding content to the file\n".as_bytes()).expect("Write failed...");

    // Read from a file
    // Uses unwrap() method which is also an error handling method.
    // It either returns the correct value or raise an error
    // let mut file = File::open("src/example.txt").unwrap();
    // // Place file contents in a variable
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // println!("{}", contents);
    //
    // // Delete a file
    // remove_file("src/example.txt").expect("Delete failed...");

    // ERRORS
    // Unrecoverable errors (panic!())
    let v = vec![1, 2, 3];
    // Below won't execute, and gives message "thread 'main' panicked at 'index out of bounds'"
    // panic! macro was invoked
    // v[10];
    // Can raise our own panic macros if we expect unrecoverable errors.
    //panic!("Something went wrong. Cannot proceed...");

    // Recoverable errors (enum Result<>...)
    let f = File::open("main.jpeg");    // Return type is a Result<File> (FP!)
    match f{
        Ok(f) => {
            println!("File found! {:?}", f);
        }
        Err(e) => {
            println!("File not found...\n {:?}", e)
        }
    }

    println!("Continuing on with the execution");

    // Option enum
    // There will be multiple error options:

    divide(Some(1));
    divide(Some(10));
    divide(None);
    divide(Some(0));

}

const ANSWER_TO_LIFE: i32 = 42;

fn divide(x: Option<i32>){
    match x {
        Some(0) => panic!("Cannot divide by 0..."),
        Some(x) => println!("The result is {}", ANSWER_TO_LIFE / x),
        None => println!("None received, the answer is {}", ANSWER_TO_LIFE)
    }
}
