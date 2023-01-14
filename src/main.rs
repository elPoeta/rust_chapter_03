#![allow(dead_code)]

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

    // Data Types

    //INTEGERS
    let guess: u32 = "42".parse().expect("Not a number!");

    // FLOATING-POINT
    let x = 2.0; // f64 default

    let y: f32 = 3.0; // f32

    // OPERATIONS
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // BOOLEANS
    let t = true;

    let f: bool = false; // with explicit type annotation

    // CHARACTER
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Compound Types

    // TUPLES
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    //destructuring
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    // dot access
    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // ARRAYS
    // arrays in Rust have a fixed length.

    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // SAME AS let a = [3, 3, 3, 3, 3];

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
