mod front_of_house;

pub use crate::front_of_house::hosting;

use rand:: { Rng, CryptoRng, ErrorKind::Transient };

use std::io::*;

pub fn eat_at_restaurant() {
    // absolute path
    hosting::add_to_waitlist();

    // relative path
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    /* Fails because it has a private field
    let meal2 = back_of_house::Breakfast {
        toast: String::from("Wheat"),
        seasonal_fruit: String::from("peaches") // Private field
    }
    */

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let secret_number = rand::thread_rng().gen_range(1, 101);

}

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub enum Appetizer {
        Soup, // enum members are public by default if the enum is public
        Salad
    }


    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
use std::fmt;
use std::io::Result as IoResult;
fn function1() -> fmt::Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}
