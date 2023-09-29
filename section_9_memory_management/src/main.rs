
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
 */

#[allow(unused_variables)]
#[allow(unused_assignments)]

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
}

fn get_str() -> &'static str{   // Static indicates a lifetime as long as the program.
    "Hello"                     // Other keywords signify different lifetimes
}
