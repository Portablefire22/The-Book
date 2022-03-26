fn main() {
    
    // Example struct provided by the book
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    // Creates an instance of the struct and fills in the fields with key: value pairs
        //Keys being the name of the field and the values being the data inside
    let mut user1 = User{
        //Key: value
        email: String::from("someone@example.com"),
        username: String::from("someuser123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}",user1.email);
    user1.email = String::from("adifferentsomeone@example.com");
    println!("{}", user1.email);
}
