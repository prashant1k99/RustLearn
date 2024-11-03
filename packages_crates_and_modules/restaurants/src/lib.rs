use self::back_of_house::Breakfast;

mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    mod serving {
        // use super::hosting;

        fn take_order() {
            // This is absolute path
            crate::front_of_house::hosting::seat_at_table();
        }

        fn serve_order() {
            // This is a relative path, since the hosting module is sibling of the current serving
            // module, we use super
            super::hosting::seat_at_table();
        }

        fn take_payment() {
            // Another example of relative path
            serve_order();
        }
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

fn eat_at_restaurant() {
    let breakfast = back_of_house::Breakfast::summer("wheat");
    // this give breakfast to us
}

// Another example where multiple imports are there for the same fn or method names:
use std::fmt::Result;
use std::io::Result as IoResult;

fn function() -> Result {}

fn function2() -> IoResult<()> {}
