



fn main() {

    // variables & mutability
    let mut i = 5; // mutable variable
    println!("\nThe value of i is: {i}");
    i = 6;
    println!("The value of i is: {i}");

    let x = 10; // immutable variable (by default)
    println!("The value of x is: {x}");

    // constants evaluation
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");
}
