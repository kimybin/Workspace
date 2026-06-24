// TODO: Fix the compiler error.
fn main() {
    // // Variables are immutable by default; use 'mut' to allow mutation.
    let mut x = 3;
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}
