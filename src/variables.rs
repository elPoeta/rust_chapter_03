fn main() {
    // CONSTANTS
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    // INMUTABLE
    let i = 9;
    println!("The value of i is: {i}");

    // MUTABLE
    let mut k = 5;
    println!("The value of k is: {k}");
    k = 6;
    println!("The value of k is: {k}");

    // SHADOWING
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // VALID
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");

    /* COMPILE-TIME ERROR

    let mut spaces = "   ";
    spaces = spaces.len();

    */
}
