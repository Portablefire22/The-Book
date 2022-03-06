fn main() {


    /*
        Shadowing requires the use of let, without the compiler will cry that the variable is immutable
    */

    let x = 5;                              // This variable is shadowed by the next instancing
    
    let x = x + 1;                          // This shadows the original instance.

    {                                           // Scope, when this ends x reverts from 12 to 6
        let x = x *2;                       // This shadows the 2nd instance for the scope.
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}" ,x);       // x is now 6 as the above scope has ended.
        

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Constant

    /*
        Tuples are ways of grouping together a number of values with differing types into one compound value
        Tuples are created with a comma-seperated list of values inside brackets
        Individual tuple values can be accessed with <tuple_name>.<pos>
        eg: tup.0 would equal 500
    */
    let tup = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("The value of y is: {}", y);


    // Arrays
    /*
        Arrrays allocate data onto the stack rather than the heap, (idk the exact details, this is explained in next chapter)
        A vector is similar to an array but can grow and shrink in size.
        Arrays fixed length, vector adjustable.
        nine times out of ten, use a vector unless i specifically know to use an arrray
    
        [i32; 5] in below example indicates the data type of the list (32 bit integer) and the size (5)
        Indexing is same as python <array>[index]. Starts at 0.

        Rust has automatic index checking, can't accidentally access invalid memory by doing array[10] in array[i32; 5];
            - This crashes the program with a panic!
    */
    let a: [i32; 5] = [1,2,3,4,5];
    println!("{}",a[3]);



}
