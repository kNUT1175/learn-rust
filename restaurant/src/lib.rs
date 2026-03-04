mod front_of_house {
    use crate::front_of_house;
    use crate::front_of_house::back_of_house::Breakfast;

    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }

fn deliver_order(){}


mod back_of_house {

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer{
        Soup(String),
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}

    
//pub use crate::front_of_house::hosting;    
    
pub fn eat_at_restaurant(){

    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //Relative path
    front_of_house::hosting::add_to_waitlist();

    let _order_1 = back_of_house::Appetizer::Salad;
    let _order_2 = back_of_house::Appetizer::Soup;

    //Order a breakfast in the summer with rye toast
    let mut meal:Breakfast = back_of_house::Breakfast::summer("Rye");
    //Change our mind about what to eat
    meal.toast = String::from("Wheat");
    println!("I'd like to eat {} toast please!", meal.toast)

    //The next line won't compile if we uncomment it
    // we aren't allowed to modify the season fruit that comes with the meal.
    //meal.seasonal_fruit = String::from("Blueberries");
}

}