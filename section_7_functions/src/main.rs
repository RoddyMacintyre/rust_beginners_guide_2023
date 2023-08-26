#[allow(unused_variables)]
#[allow(unused_assignments)]

// Globals are unsafe, surprise, surprise...
// They are never out of scope in Rust, so need to manage it ourselves
// Use `unsafe` keyword for globals

static mut R: i32 = 0;

/* ========== CLOSURES ==========
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

/* ========== HIGHER ORDER FUNCTIONS ==========
    - A function that takes another function as a parameter
        fn apply(f: fn(i32) -> i32, a: i32){}
        apply(|x| -> x + 1, a);

Rust stl has a lot of higher order functions available.
 */

macro_rules! my_macro{
    () => (println!("First macro"))  // Just parentheses is "match anything and everything"
}

// macro_rules! name {
//     ($name: expr) => (println!("Hey {}", $name))      // Match to an expression
// }

// Make macro name take as many arguments as it can
macro_rules! name {
    ($($name: expr), *) => ($(println!("Hey {}", $name);)*)
}

// Multiple match statements
macro_rules! xy{
    (x => $e: expr) => (println!("X is {}", $e));
    (y => $e: expr) => (println!("Y is {}", $e));
}

// Macro that creates a function. Catch identifier in its own function and print it out
macro_rules! build_fn{
    ($fn_name: ident) => (
        fn $fn_name(){
            println!("{:?} was called", stringify!($fn_name));
        }
    )
}

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
    print!("\n");

    // ***========== Higher order functions part ==========***
    let square = |a: i32| a * a;
    apply(square, 6);
    print!("\n");

    // No HOF vs HOF
    // Calc the sum of all the squares < 500, only for even numbers.
    let limit = 500;
    let mut sum = 0;
    for i in 0..{
        let isq = i * i;
        if isq > limit {
            break;
        }
        else{
            if is_even(i){
                sum += isq;
            }
        }
    }
    println!("The sum is: {}", sum);

    // With HOF (map, take_while, filter, fold)
    let sum2 = (0..).map(|x| x * x)
        .take_while(|&x| x <= limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("The sum is: {}", sum2);
    print!("\n");

    // ***========== Macros part ==========***
    /*
    Way to write code in a shorthand manner.
    Meta programming: Write code that writes code

    - Match some expression and perform some operation
    - Compiler fills out macro code at runtime (replaces the macro with actual code)

    println!() is a macro (Know this by the ! appending the callee name)
    formaat()! as well

    Define your own with the "macro_rules!" keyword

    example:
    macro_rules! my_macro{
        (match) => (code to run)
    }
    */
    my_macro!();
    name!("John");
    name!("Alex", "Mary", "Had", "A", "Little", "Lamb");
    xy!(x => 5);
    xy!(y => 3 * 9);
    // Build function and then call it immediately after. Meta-programming (sort of)
    build_fn!("Hey");
    hey();
}

// Define a macro


fn is_even(x: i32) -> bool {
    x % 2 == 0
}

fn apply(f: fn(i32) -> i32, a: i32){
    println!("Result:\t\t{}", f(a));
}
