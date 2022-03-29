#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


mod front_of_house {
     pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house{
    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }
    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    fn fix_incorrect_order(){
        cook_order();
        super::front_of_house::serving::serve_order();
    }
    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Since the toast field in the back_of_house::Breakfast struct is public, we can read and write to meal.toast.
    // meal.seasonal_fruit cannt be read or written to as the field is private.
    // In contrast, if an enum is public, all of its variants are public.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // These can be directly accessed as they are public.
    // Enums are pretty useless if you can't access all of their variants.
        // This causes enums to be public by default.
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}
