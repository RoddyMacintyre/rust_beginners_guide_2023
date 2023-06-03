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

}
