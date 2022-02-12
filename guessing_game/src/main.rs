use std::io;

// The book: https://doc.rust-lang.org/book/title-page.html

fn main() {
    println!("Guessing Game");
    println!("Input your guess: ");
    
    /* 
        let - creates variables 
        :: - indicates what it is associated with, String::new = new empty string 
        new - associated function
        Line creates an empty mutable string.
        mut - Mutable, idk what this means currently 11/02/22
    */
    let mut guess = String::new();
    
    // ------Receiving User Input------

    /*
        io - shortened std::io, std::io::stdin() would work if 'use std::io;' is not present
        read_line - Self explanatory tbh
        (&mut guess) - The string the input is APPENDED to. 
            - APPENDS THE DATA, DOES NOT OVERWRITE.
            - Variable must be mutable to allow for changing of the string's content
        & - reference, allows multiple parrts of the code access the same data without copying data to memory.
            - References are immutable by default and must have '&mut <variable>' to make it mutable and changeable.
    */
    io::stdin().read_line(&mut guess);



}
