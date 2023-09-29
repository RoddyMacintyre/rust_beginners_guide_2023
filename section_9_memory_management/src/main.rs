
/*
Memory management topics:
- Ownership
- Borrowing
- Lifetimes
- Reference counted variables ("manual" ownership/borrowing)

Ownership
Only one variable can own a piece of memory
    For primitives, copying data is cheap (know the mem size)
    For complex types, ownership is transferred (don't know size at compile time)

Borrowing
Ownership of a piece of data will be passed on, but for a temporary time (dependent on events)
    When the variable is "finished" or destroyed, ownership will be given back to the original var
    So:
        Variables can borrow ownership to other pieces of memory:
        let a = 6;
        let b = &a;     <- give it a reference & to the value 6 in memory
        println!("{}", *b);     <- Use * to access value in mem (this pattern is the same as c++, aka dereferencing)
    BEWARE:
        While borrowed, the original variable cannot use the value. So a += 2; will not work
        until b is finished with the borrowing

        Borrow has to match the mutability status of the var

LIFETIMES
Indication of how long an object will live
Can define our own lifetimes
Rust will prevent parts of objects to outlive that object

Lifetime elision:
    Compiler will build lifetimes for us when evident

REFERENCE COUNTED VARIABLES
Alt solution to bypass ownership/borrowing to let another object handle it explicitly
RC Variable:
    A structure that can hold multiple references to a variable
    In order to share it in different places in the code

    use std::rc::Rc;
    Use .clone() to instantiate multiple references
 */

use std::rc::Rc;

#[allow(unused_variables)]
#[allow(unused_assignments)]

#[derive(Debug)]
struct Person{
    name: String
}

#[derive(Debug)]
struct Dog<'l>{     // Assign lifetime parameter to Dog, and pass it to Person in Dog as well
    name: String,
    owner: &'l Person      // Expecting a lifetime parameter here, because it references another struct with a certain lifetime of its own
}                          // If that doesn't match, you potentially get big issues by pointing to invalid data
                           // Need to tell compiler that Dog will live as long as Person!

// Lifetime elision:
impl Person{
    fn get_name(&self) -> &String{
        &self.name
    }
    /* Compiler does the following implicitly:

    fn get_name<'l>(&'l self) -> &'l String{
        &self.name
    }
     */
}

// Reference counted variables
struct Car{
    brand: Rc<String>
}

// Implement some methods for our car
impl Car {
    fn new(brand: Rc<String>) -> Car{Car{brand: brand}}
    fn drive(&self) {
        println!("{} is driving...", &self.brand);
    }
}

fn main() {
    let i = 5;
    let j = i;  // Transferred ownership (through copying, so both hold the same value)
    println!("{}", j);
    println!("{}", i);

    let v = vec![1, 2, 3, 4, 5];
    // let w = v;
    // Cannot print out v, it's been borrowed to w! V is not accessible anymore.
    // println!("{:?}", v);
    // println!("{:?}", w);

    // Create lambda and use variable v in there
    let foo = |v: Vec<i32>| -> Vec<i32> {
        println!("Vector used in foo");
        v
    };

    let v = foo(v);
    println!("{:?}", v);
    // What's the reason we can use it like this, but not like above?
    // Ownership was transferred through the function call to a complex type (in this case a vector)

    // ===== BORROWING =====
    let mut a = 6;
    {
        let b = &mut a;

        println!("{}", *b);
        *b += 2
    }
    println!("{}", a);  // Is valid as well. But cannot work with a directly under borrowing to b.
                        // Outputs 8 because b added 2

    // *b += 2;    // Is not valid. Immutable borrow occurs here.
                // If we put the 2nd print statement, or rather access to a below this line, it will work
                // Or scope variable b

    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v{
        println!("{}", i);
        // What if we want to push an element in the vector
        // v.push(6);
        // Cannot borrow mutable v when it is also borrowed as immutable for the loop iterator
        // Remember that the iterator is an object that can borrow!
    }

    // LIFETIME
    println!("{}", get_str());

    let p1 = Person{name: String::from("John")};
    let d1 = Dog{name: String::from("Max"), owner: &p1};

    println!("{:?}", d1);

    // Lifetime elision
    let mut a: &String;
    let mut aa: &String;
    {
        let p2 = Person {name: String::from("Mary")};
        a = p2.get_name();
        aa = p1.get_name();
    }
    // Invalid, because p2 doesn't live outside its scope!
    // println!("{}", a);
    println!("{}", aa);     // Works fine, because p1 is declared in the same scope, so is guaranteed the same lifetime

    // Reference counted variables
    let brand = Rc::new(String::from("BMW"));
    // Create a scope and create a clone of this brand, see how may references we will get
    println!("Pointers: {}", Rc::strong_count(&brand));
    {
        let car = Car::new(brand.clone());
        car.drive();
        println!("Pointers: {}", Rc::strong_count(&brand));
    }
    println!("My car is a {}", brand);
    println!("Pointers: {}", Rc::strong_count(&brand));
}

fn get_str() -> &'static str{   // Static indicates a lifetime as long as the program.
    "Hello"                     // Other keywords signify different lifetimes
}
