// We define a module with mod keyword followed by the name of the module. The body of the modules
// then goes inside curly brackets. Inside modules, we can place other modules, as in this case
// with the modules hosting and serving. Moduels can also hold definition for other items, such as
// structs, enums, constants, etc..

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
