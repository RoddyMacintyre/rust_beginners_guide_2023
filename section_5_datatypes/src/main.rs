use crate::Colors::Red;
use crate::Person::Name;

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
    println!("\n");

    // =============== Tuples ===============
    // Collections of various types! (Probably the basics of a structure in Rust?)
    let person = ("John", 27, true);
    let mut person: (&str, i64, bool) = ("John", 27, true);
    println!("{:?}", person);

    // Static: cannot be resized
    // Elements can be updated (as long as the same type)
    // Indexed (random access)
    // LIMITED TO 12 ELEMENTS!
    // Dot notation
    println!("{:?}", person.0);

    // Update tuple variables
    person.0 = "Mike";
    println!("{:?}", person.0);

    // Destructure a tuple (unpacking)
    let (name, age, employed) = person;
    println!("Name:\t{name}\nAge:\t{age}\nEmployed:\t{employed}");
    println!("\n");

    // =============== Structures ===============
    // Equivalent to dict/class (just like JS objects)
    // Instantiate the struct:
    let emp = Employee{
        name: String::from("John"),
        company: String::from("Google"),
        age: 35
    };

    // To print a struct out debug will not work either.
    // Have to annotate the struct definition with a derive declaration
    println!("{:?}", emp);

    // Access with dot notation
    println!("{}", emp.name);
    // Print out using the function added to the struct:
    println!("{}", emp.df_details());
    // Printing the static method
    println!("{}", Employee::static_fn_detail());
    println!("\n");

    // =============== Enums ===============
    // Enumeration of values
    let my_color = Colors::Red;
    println!("{:?}", my_color);
    let my_color = Red;      // Auto generated from IDE
    println!("{:?}", my_color);

    // Add value to enum:
    let person = Name(String::from("Alex"));
    println!("{:?}", person);
}

// Try add element to an enum
#[derive(Debug)]
enum Person{
    Name(String),
    Surname(String),
    Age(u32)
}

// Define an enum and make it debug printable
#[derive(Debug)]
enum Colors{
    Red,
    Green,
    Blue
}

#[derive(Debug)]
struct Employee{
    name: String,
    company: String,
    age: u32
}

// Define a function for the Employee struct
// (always declared outside of the struct declaration itself)
impl Employee{
    fn df_details(&self) -> String {
        return format!("Name: {}, Age: {}, Company: {}", &self.name, &self.age, &self.company);
    }

    // Add static method. It is housed in the struct, but doesn't need a self parameter.
    // Therefore it can be used by the struct, but also non-local code can access it is needed
    // To achieve static, just don't let it expect SELF as a parameter, that's all...
    fn static_fn_detail() -> String {
        return String::from("Details of a person");
    }
}


// Function that takes a slice
fn update_colors(colors_slice: &mut [&str]){
    colors_slice[0] = "yellow";
    colors_slice[1] = "orange";
    // Done at runtime, so we won;t see this error at compile time:
    // colors_slice[2] = "brown";
}
