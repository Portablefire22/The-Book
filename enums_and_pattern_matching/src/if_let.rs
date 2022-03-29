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

    //let coin = Coin::Penny;

    // if let, as you can imagine, combines if and let into a single expression.
    // match example
    let config_max = Some(3u8);
    match config_max{
        Some(x) => println!("The maximum is configured to be {}", x),
        None => println!("No value"),
    }
    
    // if let example
    // Using if let is less verbose than using match but the exhaustive checking from _ or other is lost.
    // Else can also be added to the end of the if let statement to act as _.
    let config_max = Some(3u8);
    if let Some(max) = config_max{
        println!("The maximum is configured to be {}", max);
    }
    
    let mut count = 0;
    if let Coin::Quarter(state) = coin{
        println!("State quarter from {:?}!", state);
    }
    else{
        count += 1;
    }
    println!("{}", count);
}