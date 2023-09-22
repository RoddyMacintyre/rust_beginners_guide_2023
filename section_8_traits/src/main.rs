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

// OPERATOR OVERLOADING
// Operator is a trait, so works same as other traits for implementing
use std::ops::Add;

#[derive(Debug)]
struct Point{
    x: f64,
    y: f64
}

impl Add for Point{
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

// STATIC DISPATCH
// Means: generic trait will be converted to the required type at compile time
trait Duplicateable{
    fn dupl(&self) -> String;
}

impl Duplicateable for String{
    fn dupl(&self) -> String {
        format!("{0}{0}", *self)
    }
}

impl Duplicateable for i32{
    fn dupl(&self) -> String {
        format!("{}", *self * 2)
    }
}

// What if we want to have a function called duplicate
// that can take a String or i32 and print out the result
// It will figure out what function to create based on this at compile time

// MONOMORPHIZATION
fn duplicate<T: Duplicateable> (x: T){
    println!("{}", x.dupl())
}

// DYNAMIC DISPATCH
// Generic trait that will be converted to the required type at runtime
// It uses a trait reference to decide what to run, and this cannot be done at compile time
fn duplicate_(x: &dyn Duplicateable){
    println!("{}", x.dupl())
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

    let p1 = Point{x: 1.3, y: 4.6};
    let p2 = Point{x: 3.7, y: 1.4};
    let p3 = p1 + p2;
    println!("{:?}", p3);   // :? = Debug formatter

    let aa = 42;
    let bb = "Hi John ".to_string();
    duplicate(aa);
    duplicate(bb);

    let aaa = 42;
    let bbb = "Hi John ".to_string();

    duplicate_(&aaa);
    duplicate_(&bbb);
}
