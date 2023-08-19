#[allow(unused_variables)]
#[allow(unused_assignments)]

// Globals are unsafe, surprise, surprise...
// They are never out of scope in Rust, so need to manage it ourselves
// Use `unsafe` keyword for globals

static mut R: i32 = 0;

/* Closures
    - A function within a function
    - Anonymous function (lambda expression)

    In Rust expressed as follows:
    no return-type, single expression:
        |a: i32, b: i32| println!("{}", a + b)  - No curly bracket needed for scope
    return-type/multiple expressions:
        |a: i32, b: i32| -> i32 {a + b}         - Curly brackets needed for scope

Why closures?
    - Functions can be assigned to a variable (they are first-class objects)
        let sum = |a: i32, b: i32| -> i32 {a + b};
        sum(2, 3);

A Closure can be generic (not needing to declare types of the variables)
 */

fn main() {
    // Creating scopes...
    {
        let a = 3;
        println!("{}", a);
    }
    // println!("{}", a);

    unsafe{
        R = 4;
        println!("R = {}", R);
    }

    unsafe{
        println!("R = {}", R);
    }
    print!("\n");

    // ***========== Closures part ==========***
    let a = |a: i32| a + 1;
    println!("{}", a(6));

    // Curly braces because more than 1 expression in the lambda
    let b = |b: i32| -> i32 {
        let c = b + 1;
        c
    };
    println!("{}", b(4));

    // Generic closure
    let gen = |x| println!("{}", x);
    // gen(3);
    // Try another type on the generic. Won't work because of compile time guarantees.
    // If it has been used as an int, will assume it from there on out.
    // ===== SWITCH BETWEEN STATEMENTS TO SEE IT IN ACTION! =====
    //gen(true);
    gen(3.5);
}
