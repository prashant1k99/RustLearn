// This is the integration testing of the library

use adder::add_two;

// use common::{end, setup};
mod common;

#[test]
fn integration_test() {
    common::setup();
    let result = add_two(5);
    assert_eq!(7, result);
    common::end();
}

// The output is as following where there are three sections, first sectio is unit testing and
// second section is integratio testing.
// Note if any previous section fails there would not be any output for the proceeding sections
//
// OUTPUT:
// running 11 tests
// test test_rectangle::larger_can_hold_smaller ... ok
// test test_greeting::greeting_contains_name ... ok
// test tests_add::should_panic_with_error_msg ... ignored
// test tests_add::positive_case ... ok
// test tests_add::should_fail ... ok
// test test_rectangle::smaller_cannot_hold_larger ... ok
// test tests_add_two::it_adds_two ... ok
// test tests_printing::this_test_will_fail ... ignored
// test tests_add_two::it_works ... ok
// test tests_add::should_panic - should panic ... ok
// test tests_printing::this_test_will_pass ... ok

// test result: ok. 9 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 0.00s

//      Running tests/integration_tests.rs (target/debug/deps/integration_tests-fedb2afb840c7e9c)

// running 1 test
// test integration_test ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//    Doc-tests adder

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
