mod front_of_house;

fn deliver_order() {}

mod back_of_house {

    pub enum Appetizer {
        Soup,
        Salad
    }

    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasona_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasona_fruit: String::from("peach")
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();

        super::deliver_order()
    }

    fn cook_order() {}

}

mod customer {

    // using use to expose a ft
    use crate::front_of_house::{self, hosting};
    use crate::back_of_house;

    pub fn eat_at_restaurant() {

        // using exposed hosting mod
        hosting::add_to_waitlist();
    
        // checking out buying a toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // since toast is public, we can edit it
        meal.toast = String::from("Wheat");
        println!("The meal I ordered is {:#?}!", meal);
    
        let order1 = back_of_house::Appetizer::Salad;
        let order2 = back_of_house::Appetizer::Soup;
    
    
        // using absolute path
        crate::front_of_house::hosting::add_to_waitlist();
    
        // using relative path
        front_of_house::hosting::add_to_waitlist();
    }
}