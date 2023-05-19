fn main() {
    println!("Hello, world!");
    // Formatting
    println!("My name is {} and I'm {} years old", "Roddy", 40);
    // Expression formatting
    println!("a + b = {}", 3 + 9);
    // Positional args
    println!("{0}, has a {2} and {0} has a {1}", "Roddy", "another cat", "cat");
    // Named args
    println!("{name} {surname}", surname="Macintyre", name="Roddy");
    // Printing traits (convert the args into certain representations
    println!("binary: {:b}, hex: {:x}, octal: {:o}", 16, 16, 16);
    // Debug (normally can't print complex structures, can do with {:?})
    println!("array: {:?}", [1, 2, 3]);
}
