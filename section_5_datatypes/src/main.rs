#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    // =============== Arrays ===============
    let primes = [2, 3, 5, 7, 11];
    let doubles: [f64; 4] = [2.0, 4.0, 6.0, 8.0];
    // Printing this datastructure will fail! But we can use the debug printer:
    println!("{:?}", primes);
    println!("{:?}", doubles);
    // Arrays cannot be resized
    // Can modify elements if mutable, but can never delete.
    // Is indexed (random access)

    // Array with default values
    let numbers = [0;15];
    println!("{:?}", numbers);

    // Can use a const as default
    const DEFAULT: i32 = 3;
    let mut numbers = [DEFAULT; 15];
    println!("{:?}", numbers);
    println!("{:?}", numbers[3]);

    // Update elements (only if mutable)
    numbers[3] = 5;
    println!("{:?}", numbers);

    // Using array iterator
    for number in numbers.iter(){
        println!("{}", number);
    }
    println!("\n");

    // =============== Vectors ===============
    // Are arrays of variable size. Do not really exist as a type in Rust.
    // Built through a construction (class/object/structure)
    let primes: Vec<i32> = Vec::new();
    let mut primes = vec![2, 3, 5];
    println!("{:?}", primes);

    // Adding elements
    primes.push(7);
    println!("{:?}", primes);
    // Removing elements
    primes.remove(2);
    println!("{:?}", primes);

    // Vectors with defaults
    let mut numbers = vec![2;10];
    println!("{:?}", numbers);

    const DEFAULT_1: bool = true;
    let values = vec![DEFAULT_1;8];
    println!("{:?}", values);

    // Update elements
    numbers[5] = 8;
    println!("{:?}", numbers);

    // Using an iterator
    for number in numbers.iter(){
        println!("{:?}", number * number);
    }
    println!("\n");

    // =============== Slices ===============
    // A pointer to a block of memory (pointer + offset/size)
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];
    println!("{:?}", slice);
    // Size is determined at runtime, used on arrays, vectors & strings

    // Mutable slices allow us to change their values
    let mut colors = ["red", "green", "blue", "pink"];
    println!("{:?}", colors);
    update_colors(&mut colors[2..4]);
    println!("{:?}", colors);
}

// Function that takes a slice
fn update_colors(colors_slice: &mut [&str]){
    colors_slice[0] = "yellow";
    colors_slice[1] = "orange";
    // Done at runtime, so we won;t see this error at compile time:
    // colors_slice[2] = "brown";
}
