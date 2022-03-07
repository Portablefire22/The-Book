fn main() {
    println!("Hello, world!");
    let s = String::from("This_is_a_test"); // Put "Hello!" in s
    
    println!("{}",first_word(&s)); // Send reference to first_word) and print result

    println!("{}", string_slicing(&s)); // Sends reference to string_slicing and print result
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // Convert string to an array of bytes
    // i represents index in tuple, &item represents reference in tuple.
    for(i, &item) in bytes.iter().enumerate(){ // Creates an iterator over array with iter()
                                               // Enumerate wraps the result of iter into a tuple (index, reference)
        if item == b' ' { // Checks if current byte is equal to the byte value of ' '
            return i; // Returns current iteration if space is encountered (returns length of first word)
        }
    }

    s.len() // Return length of s if loop does not return
}

fn string_slicing(s: &String) -> &str { // &str indicated string slice

    let first_word_length = first_word(s);  // Gets length of first word
    let result = &s[0..first_word_length/2]; // Gets the first half of the first word
    result // Returns result

    /*
        String literals are stored in the binary and thus are immutable.
        A string slice points to the specific point in binary.
    */

}