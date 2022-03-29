mod match_example;
mod if_let;
// Enums allow for a type to be defined by listing the various possible variants
// They can be used to for pattern matching


// In this example, the context of an IP address will be used.
// The IP address will be represented by an enum.
// Since IP addresses only have two possible types (IPV6 or IPV4) and is mutually exlusive, an enum is used.

enum IpAddrKind{
    V4,
    V6,
}

impl IpAddrKind{ // Like a struct, enums can also have methods using impl
    fn print(&self){
        match self{
            IpAddrKind::V4 => println!("IPV4"),
            IpAddrKind::V6 => println!("IPV6"),
        }
    }
}

fn main() {
    // The data inside of an enum can be of any type, including other enums.
    enum IpAddr{
        V4(u8, u8, u8, u8), // A benefit of using enums is that you can have multiple variants, structs cannot do this
        V6(String),
    }    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    IpAddrKind::print(&four); // I like the look of this one more, it's more readable
    four.print();

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    match_example::main();
    if_let::main();
}


fn route(ip_kind: IpAddrKind){} // This does nothing in the doc and I have 0 idea of what I could do with it.



// Option Enum
// Option is a type of enum tthat can be eitherr something or nothing (null).

// I think I understand the concept of Null
// (These are meant to be boxes)
// https://www.reddit.com/r/ProgrammerHumor/comments/6f68rv/difference_between_0_and_null/ <==== Useful meme
// #######
// #     #
// #     #  This is 
// #     #
// #######
// 
// 
// 
//          This is null
// 
//
// The difference being is that in the first example, while the box is empty, the box is still there.
// Whereas in the second example, there is no box.


// <T> : This means it can hold any data type
enum Option<T>{
    Some(T), // This is not null
    None, // This would be null
}