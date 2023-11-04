mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// Exposing paths with the pub Keywword
// export using absolute and relative path
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

mod back_of_house_v2 {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruits: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruits: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant_v2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house_v2::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

// Making enum public, makes all its variats public
mod back_of_house_v3 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_v3() {
    let order1 = back_of_house_v3::Appetizer::Soup;
    let order2 = back_of_house_v3::Appetizer::Salad;
}
