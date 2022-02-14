/*
    Functions, defined using fn
    snake_case should be used in functions    
*/

fn main() {
    println!("Hello, world!");
    another_function();
    another_function_but_with_parameters(9);
}

fn another_function(){
    println!("Another function!");
}

/*
    Rust does not care where functions are defined, another_function() could be defined before main() and it would still function

*/

fn another_function_but_with_parameters(x:i32){
    println!("Another function but with the parameter being: {}", x);
}

// I stopped at https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#statements-and-expressions