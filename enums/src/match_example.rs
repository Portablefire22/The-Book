
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState{
    Alabama,
    Alaska,
    // --snip--
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


pub fn main(){
    let coin = Coin::Quarter(UsState::Alaska);
    println!("{}",value_in_cents(coin));

    fn plus_one(x: Option<i32>) -> Option<i32>{ // YOU CAN PUT FUNCTIONS IN MAIN?? HAVE I BEEN MISSING SOMETHING?
        match x{
            None => {
                println!("SPONGEBOB ME BOY, WHY THE FUCK HAVE YOU GIVEN ME NOTHING");
                None
            },
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);
}
// Pass the enum value to value_in_cents and return a u32 value
fn value_in_cents(coin: Coin) -> u32{
    match coin{ // Basically a switch statement for the different enum variants, these can return any data type.
        Coin::Penny => {
            println!("SPONGEBOB ME BOY, I'VE FOUND A PENNY!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("SPONGEBOY, ME QUARTER IS FROM {:?}",state);
            25
        },
    }
}
// In a match all possibilities must be covered, if a possibility isn't covered then the compiler will complain.
// This is because the compiler doesn't know what to do with the value.
// This can be fixed by using '_' or 'other'
// '_' is used when you don't care about the value.
// 'other' is used when you don't care about the value, but you want to use it.
