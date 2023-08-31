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
}
