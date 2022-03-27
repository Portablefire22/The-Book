mod example_program;
// Example struct provided by the book
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    
    
    // Creates an instance of the struct and fills in the fields with key: value pairs
        //Keys being the name of the field and the values being the data inside
    let mut user1 = User{
        //Key: value
        email: String::from("someone@example.com"),
        username: String::from("someuser123"),
        active: true,
        sign_in_count: 1,
    };

    // The entire struct must be mutable, singular fields cannot be made mutable.
    println!("{}",user1.email);
    user1.email = String::from("adifferentsomeone@example.com");
    println!("{}", user1.email);

    user1 = build_user(
        String::from("someone@example.com"), 
        String::from("someuser123"),
    );

    println!("{}", user1.username);

    // Virgin, boring code, repeats shit
    let user2 = User{
        active: user1.active,
        username: user1.username,
        email: String::from("Anotheremail@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Chad, straight to the point, values your time
    let user3 = User{
        email: String::from("another@example.com"),
        ..user2 // Signals that the rest of the data comes from the user2
    };

    // Doing the above moves the data around, this means that user1 data cannout be used anymore
    //  (hence why user3 uses user2 instead of user1, which theoretically has the same data).
    //  This only happens because String does not support Copy, if only sign_in_count or active were yoinked then
    //  user1 would still be active as the data would be copied instead of moved.

    example_program::rect_area();
}

/* Boring, icky, forced to write shit twice
fn build_user(email: String, username:String) -> User{ 
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
*/

// Chad, doesn't require double writing shit
fn build_user(email: String, username:String) -> User{
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// A type of struct that looks similar to tuples, fields are without names and only use the data types.
//  Often used when naming each field would be verbose or redundant.
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

fn tuple_structs(){
    // Black and origin are differrent types as they are different tuple structures.
    //  This is important when passing Colour as an arguement as Point would not be accepted
    //      (black could be used where origin cannot).
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}

//Unit-like struct
// These are used to give a type to a value but do not have any data.
struct AlwaysEqual;

fn unit_like_struct(){
    let subject = AlwaysEqual;
}


