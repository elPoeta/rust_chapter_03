fn main() {
    println!("main function!");

    //  Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value. Let’s look at some examples.

    // one statement
    let y = 6;
    println!("The value of y is: {y}");

    //expression
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
