pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests_add {
    use super::*;

    #[test]
    fn positive_case() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn should_fail() {
        let result = add(3, 5);
        assert_ne!(result, 4);
    }

    // To check for the panic, we can do it by adding should_panic attribute
    #[test]
    #[should_panic]
    fn should_panic() {
        panic!("Unknown reasion");
    }

    // to write the custom messages for panic:
    #[test]
    #[should_panic(expected = "Something went wrong")]
    fn should_panic_with_error_msg() {
        panic!("Unknown reasion");
        // note: panic did not contain expected string
        //       panic message: `"Unknown reasion"`,
        //  expected substring: `"Something went wrong"`
    }
    // This gives us better understanding that why it paniced
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    pub fn can_hold(&self, another_rect: &Rectangle) -> bool {
        self.width >= another_rect.width && self.height >= another_rect.height
    }
}

#[cfg(test)]
mod test_rectangle {
    // here we are using super::* to bring the other modules to the test module
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            height: 20,
            width: 30,
        };
        let smaller = Rectangle {
            height: 15,
            width: 30,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            height: 20,
            width: 30,
        };
        let smaller = Rectangle {
            height: 15,
            width: 30,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

#[cfg(test)]
mod tests_add_two {
    use super::*;

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    // to use Result<T> in the test cases:
    // Here we are defining the custom Result type as default one expects Error
    #[test]
    fn it_works() -> std::result::Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
    // String::from("Hello")
}

#[cfg(test)]
mod test_greeting {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // Here we pass the custom failure message so that we know what's the issue
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was \"{result}\""
        );
    }
}

// By default the tests run in parallel, if we want to run them in order, we can achieve it by
// setting the following argument:
// cargo test -- --test-threads=1
// Where test-threads denote number of threads allowed parallely, we can control it and see how it
// affects in different settings

// Now bydefault we don't see the console output in the result but if we want to se it, we can do
// that by using following command
// cargo test -- --show-output
// Example:
pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

#[cfg(test)]
mod tests_printing {
    use super::prints_and_returns_10;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }
}
// With this and the argument --show-output we are able to see the println! output in the test
// suite

// To Run a Subset of test from the suite of tests:
// To run a single test out of all, we just specify which tests to run:
// cargo test tests_printing
// Output:
// running 2 tests
// test tests_printing::this_test_will_pass ... ok
// test tests_printing::this_test_will_fail ... FAILED
//
// To filter out test based on the test names, example:
// cargo test add
// Output:
// running 6 tests
// test tests_add::positive_case ... ok
// test tests_add::should_fail ... ok
// test tests_add_two::it_works ... ok
// test tests_add_two::it_adds_two ... ok
// test tests_add::should_panic_with_error_msg - should panic ... FAILED
// test tests_add::should_panic - should panic ... ok

// To ignore some test cases, just add #[ignore] attribute to the test case. As added in
// tests_printing test
//
// cargo test tests_printing
// running 2 tests
// test tests_printing::this_test_will_fail ... ignored
// test tests_printing::this_test_will_pass ... ok
