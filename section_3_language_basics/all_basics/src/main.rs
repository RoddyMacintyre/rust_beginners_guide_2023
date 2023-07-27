#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(dead_code)]

const URL: &str = "google.com";

fn main() {
    // Implicit types
    let name = "Alex";  // Scalar type of string
    let age = 32;   // 32 bit int

    // Explicit types
    let amount:i64 = 8473926758472;

    // Variables are immutable by default!
    //age = 43;
    // Have to declare it mutable
    let mut length = 34;
    println!("{}", length);

    length = 68;
    println!("{}", length);

    // Shadowing is allowed (declare same variable name multiple times
    let color = "blue";
    let color = 86;
    println!("Color is {}\n", color);

    // Declare multiple variables simultaneously
    let (a, b, c) = (43, 86, "red");

    // ========== SCALAR DATATYPES ==========
    //let pi: f32 = 4;
    let pi: f32 = 4.0;
    // Number separator
    let million:i32 = 1_000_000;
    println!("{}", million);

    // Boolean
    let is_day = true;
    let is_night = false;
    println!("{}", is_day);

    // Char (single quotes, can't be more than one)
    let char1 = 'a';
    let smiley_face = '\u{1F601}';
    println!("{}\n", smiley_face);

    // ========== STRINGS ==========
    // Considered slices if Scalar
    let cat:&'static str = "Fluffy";
    println!("{}", cat);

    // String objects
    let dog = String::new();
    let mut dog = String::from("Max");
    println!("{}", dog);

    // Format! Macro
    let owner = format!("Hi I'm {} the owner of {}", "Roddy", dog);
    println!("{}\n", owner);

    // String methods
    println!("{}", dog.len());
    dog.push(' ');
    dog.push_str("the dog!");
    println!("{}", dog);

    let new_dog = dog.replace("the", "is my");
    println!("{}\n", new_dog);

    // ========== CONSTS ==========
    println!("{}\n", URL);

    // ========== OPERATORS ==========
    // -- & ++ are not supported!
    let a = 4 + 8;
    let b = 10 / 3;
    let c = 10 % 3;
    println!("a={}, b={}, c={}", a, b, c);

    println!("{}", a >= b);

    println!("{}\n", a >= b && b<= c);

    // ========== FUNCTIONS ==========
    say_hi();

    for i in 1..6{
        say_hi()
    }
    println!("\n");

    let mut name = "John";
    // Pass by reference (the actual pointer//value!)
    say_hello(&mut name);
    println!("{}", name);

    // Return values
    let greeting = say_return(&mut name);
    println!("{}", greeting);

}

fn say_hi(){
    println!("Hello there!");
}

// Pass by value or reference BORROWING!!
fn say_hello(name: &mut &str){
    println!("Hello {}", name);
    *name = "Alex";
}

fn say_return(name: &mut &str) -> String{
    let greeting = format!("Hello {}", name);
    //return greeting;
    // If return is the last item in the body of the function,
    // then can do without the return declaration and without the semicolon on the end
    greeting
}
