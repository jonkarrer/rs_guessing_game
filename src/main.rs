// Import the input/output library (io) from standard library (std:).
// The prelude brings in a bunch of standard packages in every project, but not io.
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    /* 
     * Create a mutable variable let mut guess
     * Assign to a function for a new instance of String type
     * The :: syntax indicates that new is an associated function of the String type.
     * An associated function is a function that is implemented on a type, like String.
     * Overall, this creates a new empty string.
    */
    let mut guess = String::new();

    /* io::stdn
     * Could have wrote std::io::stdin. But we imported it up top.
     * the stdin function returns an instance of std::io::Stdin, 
     * a type that represents a handle to the standard input (stdin) of your terminal.
    */
    /* .readline()
     * Get the input from the user.
     * Store the input in our string variable.
     * read_line() returns an io::Result type. This can be Ok or Err.
     * expect() method will handle this.
    */
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
