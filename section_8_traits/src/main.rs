/*

Traits expand on the flexibility we have when we use structures.
    - Similar to OOP interfaces
    - Defines what we can do with our structures

Topics:
    - Generics                  (different parameters, same function)
    - dyn                       (keyword to allow dynamic types at runtime)
    - Operator overloading
    - Static dispatch
    - Dynamic dispatch

 */

//use test::stats::Stats;

#[allow(unused_variables)]
#[allow(unused_assignments)]

// Define some structures
struct RustDev{
    awesome: bool
}

struct JavaDev{
    awesome: bool
}

// Now create a trait
trait Developer{
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) {println!("Hello world!")}

}

// Implement trait for both RustDev and JavaDev
impl Developer for RustDev{
    fn new(awesome: bool) -> Self {
        RustDev{awesome: awesome}
    }

    fn language(&self) -> &str {
        "Rust"
    }

    fn say_hello(&self) {
        println!("println!(\"Hello world!\");");
    }
}

impl Developer for JavaDev{
    fn new(awesome: bool) -> Self {
        JavaDev{awesome: awesome}
    }

    fn language(&self) -> &str {
        "Java 1.8"
    }

    fn say_hello(&self) {
        println!("System.out.println(\"Hello world!\")");
    }
}

// GENERICS
// Define a trait to exemplify Trait generics
trait Bark{
    fn bark(&self) -> String;
}

// Create some structs for the Generics
struct Dog{
    species: &'static str
}

struct Cat{
    color: &'static str
}

impl Bark for Dog{
    fn bark(&self) -> String {
        return format!("{} barking", self.species)
    }
}

// Now define a function bark
fn bark_it<T: Bark>(b: T){
    println!("{}", b.bark())
}

// RETURNING TRAITS
// struct Dog{}
// struct Cat{}

trait Animal{
    fn make_noise(&self) -> &'static str;
}

impl Animal for Dog{
    fn make_noise(&self) -> &'static str {
        "Woof"
    }
}

impl Animal for Cat{
    fn make_noise(&self) -> &'static str {
        "Meow"
    }
}
// Implement trait:
// Cannot return a trait because of Rust memory guarantees.
// Needs to know what the size of the returned value will be at compile time.
// Can use a box for this purpose
fn get_animal(rand_number: f64) -> Box<dyn Animal>{
    if rand_number < 1.0 {
        Box::new(Dog{species: "retriever"})
    }
    else{
        Box::new(Cat {color: "black"})
    }
}

// ADDING TRAITS TO EXISTING STRUCTURES.
// Create trait and add to struct that you didn't create
trait Summable<T>{
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>{
    fn sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for i in self{
            sum += *i;
        }
        sum
    }
}

fn main() {
    // let r = RustDev{awesome: true};
    let r = RustDev::new(false);
    let j = JavaDev::new(false);

    println!("{}", r.language());
    r.say_hello();
    println!("");

    let dog = Dog{species: "retriever"};
    let cat = Cat{color: "black"};

    bark_it(dog);
    // Cannot say:
    // bark_it(cat);
    println!("The animal says {}", get_animal(0.5).make_noise());
    println!("The animal says {}", get_animal(2.0).make_noise());

    let a = vec![1, 2, 3, 4, 5];
    // Vector now has trait Summable with .sum() callable
    println!("sum = {}", a.sum());
    let b = vec![1.0, 2.0, 3.0];
    // Not available at this time. Need to implement first!
    // println!("sum = {}", b.sum());
}
