
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
}
