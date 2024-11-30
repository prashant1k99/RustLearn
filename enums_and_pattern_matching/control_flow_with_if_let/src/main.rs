#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // The if let syntax lets you combine if and let into a less verbose way to handle values that
    // match one pattern while ignoring the rest:
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    // The maximum is configured to be 3

    // If the value is Some, we print out the value in the Some variants binding the value to the
    // variable max in the pattern. We don't want to do anything with None value. But we are
    // forced to implemnet _ in order to satisfy the compiler condition.
    // Alternatively we can do is:
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
        // The maximum is configured to be 3
    }

    // We can also use else:
    let mut count = 0;
    let coin = Coin::Penny;
    match coin {
        Coin::Quarter(state) => println!("State quatrter from {state:?} !"),
        _ => count += 1,
    }
    println!("Total count is {count}");
    // Total count is 1

    // We can achieve the same with if let:
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}
