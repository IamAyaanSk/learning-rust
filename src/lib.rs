// This is a library crate, no binary code is created for this

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

mod hello {
    pub fn greet_customer() {
        crate::front_of_house::hosting::add_to_waitlist();

        // Using super to reference parent module in relative path
        super::front_of_house::hosting::add_to_waitlist();
    }
}

// Similarly we can use pub for structs

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // Summer breakfast
        // If we don't take &self as an arg, we create an associated fn not an instance fn
        pub fn summer_breakfast(toast: &str) -> Breakfast {
            Breakfast {
                toast: toast.to_string(),
                seasonal_fruit: String::from("Mango"),
            }
        }
    }
}

pub fn make_a_summer_meal() {
    // This is invalid, we can't directly use struct if all values are not public
    // let mut meal1 = back_of_house::Breakfast {
    //     toast: String::from("Test"),
    //     seasonal_fruit: String::from("Test"),
    // };

    let mut meal1 = back_of_house::Breakfast::summer_breakfast("Wheat");
    // To access a data of struct we need to make that property a pub
    println!("Public data in struct: {}", meal1.toast);
    meal1.toast = String::from("Ragi");
    println!("Public data in struct mutated: {}", meal1.toast);
}

// Similarly we can create public enums

mod kitchen {
    // All variants are public by default
    pub enum Water {
        Sparkling,
        Still,
    }
}

pub fn order_water() {
    let order1 = kitchen::Water::Sparkling;
    let order2 = crate::kitchen::Water::Still;
}

// We can also use keyword to bring a path in scope

pub use crate::front_of_house::hosting;

// Convention
// Functions --> upto parent module
// Struct, Enums --> complete path
// Exception --> when bringing two enum/struct with same name from different modules
use std::fmt::Result;
use std::io::Result as IoResutl;
// or simply bring only parent to scope

// We can also use nested paths when bringing multiple things from a path
use std::io::{self, Write};
// Sim

pub fn queue_customer() {
    hosting::add_to_waitlist();
}

// To use a module scope in some other external file we export it
// eg:
pub use back_of_house::Breakfast;
// Can be accessed as tutorial::Breakfast;
// Instead of tutorial::back_of_house::Breakfast;

// We can move a module contents to a different file
pub mod authentication;
