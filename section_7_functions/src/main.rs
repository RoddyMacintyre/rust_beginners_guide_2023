#[allow(unused_variables)]
#[allow(unused_assignments)]

// Globals are unsafe, surprise, surprise...
// They are never out of scope in Rust, so need to manage it ourselves
// Use `unsafe` keyword for globals

static mut R: i32 = 0;

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
}
