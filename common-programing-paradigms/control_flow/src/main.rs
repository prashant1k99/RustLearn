fn main() {
    // The ability to run some code depending on whether a condition is true and to run some code
    // repeatedly while a condition is true a basic building block in most programming langiages.
    // The mose common constructs taht let you control the flow of execution of Rust code are if
    // expressions and loops
    println!("Hello, world!");

    if_expressions(4);
    // condition was true

    loop_expressions();
}

fn loop_expressions() {
    // the `loop` keyword tells Rust to execute a block of code over and over again forever or
    // until you explicitly tell it to stop.
    // loop {
    //     println!("again")
    // }
    // This code will keep on looping forever as there is no stop condition

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
    // The result is 20

    // If you have loops within loops, `break` and `continue` apply to the innermost loop at that
    // point. You can optionally specify a loop label on a loop that you can then use with `break`
    // or `continue` to specify that those keywords apply to the labeled loop instead of the
    // innermost loop. Loop labels must bregin with a single quote.
    let mut count = 0;

    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    // Count = 0
    // remaining = 10
    // remaining = 9
    // Count = 1
    // remaining = 10
    // remaining = 9
    // Count = 2
    // remaining = 10
    // End count = 2

    // Conditional Loops with while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!!");
    // 3!
    // 2!
    // 1!
    // LIFTOFF!!!!

    // For Loop
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}")
    }
    // the value is: 10
    // the value is: 20
    // the value is: 30
    // the value is: 40
    // the value is: 50
    for number in (1..4).rev() {
        println!("{number}!")
    }
    println!("LIFTOFF!!!!");
    // 3!
    // 2!
    // 1!
    // LIFTOFF!!!!

    // It should print all alphabets, `..=` is used to include z
    for letter in 'a'..='z' {
        println!("{letter}")
    }
}

fn if_expressions(number: i32) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }

    // Using if in a let statement:

    let x = if number == 3 { number } else { 3 };
    println!("The value of x is {}", x)
    // The value of x is 3

    // let x = if number == 3 { number } else { "three" };
    //     ├╴  `if` and `else` have incompatible types │    expected integer, found `&str` rustc (E0308) [25, 46]
}
