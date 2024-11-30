fn main() {
    // Functions are prevalent in Rust code. the `main` function is the most important funciton,
    // which is the entry point of many programs. We use the keyword `fn` to declare new fucntion.
    // Rust code uses snake case as the conventional style of funciton and variable names, in
    // which all letters and underscores seperate words.
    println!("Hello, world!");

    another_funciton("Prashant".to_string());

    print_labeled_measurement(5, 'h');

    statement_and_expressions();
}

fn statement_and_expressions() {
    // Statements are instructions that perform some action and do not return a value
    // Expressions evalutate to a resultant value.
    let _y = 6; // This is a statement

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {}", y);
    // The value of y is 4
    // Here the expression { let x = 3; x + 1 } is a block that, in this case evaluates to `4`.
    // That value gets bound to `y` as part of the let statement. Note that the `x + 1` line
    // doesn't have semicolon at end. Expressions do not include ending semicolons.
    // If you add a semicolon to the end of an expression, you turn into a statement, and it will
    // then not return a value.

    let x = five();
    println!("The value of x is: {}", x);
    // The value of x is: 5

    let x = plus_one(x);
    println!("The value of x is: {}", x);
    // The value of x is: 6
}

// There are no function calls, macros, or even let statements in `five` function-just the number
// `5` by itself. Note that the function's return type is specified too, as `-> i32`.
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value} {unit_label}")
    // The measurement is: 5 h
}

fn another_funciton(x: String) {
    println!("Hello {}", x)
    // Hello Prashant
}
