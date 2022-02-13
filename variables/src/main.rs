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
        

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
