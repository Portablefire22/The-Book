use std::io;
use rand::Rng;
use std::cmp::Ordering;
// The book: https://doc.rust-lang.org/book/title-page.html

/*
    Going to actually try and make this look good.
    Will probably try to follow the conventions of the rust language.
*/

fn main() {
    println!("Guessing Game");
    
    
    /* 
        let - creates variables 
        :: - indicates what it is associated with, String::new = new empty string 
        new - associated function
        Line creates an empty mutable string.
        mut - Mutable, idk what this means currently 11/02/22
    */
    
    
    

    /*
        gen_range() - Range of generation. Includes lower bound, excluded upper bound. 1 <= x < 101
    */
    let secret_number = rand::thread_rng().gen_range(1..101); // Generates a random number local to the current thread. 
    //println!("The secret number is {}", secret_number);       // Not really that secret tbh
    

    loop { // Simple infinite loop :)

        println!("Input your guess: ");
        let mut guess = String::new();


        // ------------------Receiving User Input------------------

        /*
            io - shortened std::io, std::io::stdin() would work if 'use std::io;' is not present
            read_line - Self explanatory tbh
            (&mut guess) - The string the input is APPENDED to. 
                - APPENDS THE DATA, DOES NOT OVERWRITE.
                - Variable must be mutable to allow for changing of the string's content
            & - reference, allows multiple parrts of the code access the same data without copying data to memory.
                - References, like variables, are immutable by default and must have '&mut <variable>' to make it mutable/changeable.

            Enums (From a retarded perspective):
                - These have a set of possibilities called varriants
                - Often used with 'match', a conditional which allows different code to execute based on the variant of the enum during evaluation.
            read_line's variants are: 
                - Ok  : Successful
                - Err : Contain info on how or why it failed
        */
        io::stdin()
            .read_line(&mut guess)              // read_line returns a value (io::Result), these are enums (????).
            .expect("Failed to read line");     // crashes the program if read_line returns 'Err', providing the error message in terminal

        /*
            Converts guess into an unsigned 32 bit integer.
            'Shadows' the old value with a new one. 
                - Essentially removing the old value, allowing us to re-use the name for conversion
            .trim() is required as pressing ENTER to complete the input will append a '\n' to the string. trim removes this
                - '\r' is also added, resulting in '\r\n', if the system is running Windows
        */
        
        /*
            This now uses enum matching to check if the value is a number. If ok, go ahead with program. If Err, start loop again.
        */
        let guess: u32 = match guess            // 2nd guess refers to the original string
            .trim()                             // Eliminates whitespace at beginning or end, ensures only numbers/letters remain.
            .parse() {                          // Parses a string into a number, : u32 required to let Rust know what type we want from parse.
                Ok(num) => num,
                Err(_) => continue,
            };
            //.expect("Please type a number!");   // If parse() fails to convert to a number, outputs "Please type a number!"

        println!("You guessed: {}", guess);

        // ------------------Comparing------------------

        /*
            Not going to lie, this shit is cool
            
            Ordering is an enum with the variants: 'Less', 'Greater' and 'Equal'. cmp compares two variables and returns it with a variant of Ordering.
            match is then used to decide what to do based on the enum's variant returned by cmp.
            Functions like an if, elif. Ends comparing variant to options when it finds a match.
        */
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        /*
            Okay, I'm not going to follow these fucking guidelines, "indent with four spaces"; fuck off.

            TIL VScode converts tabs to spaces. :)
        */
    }

    
}
