use std::collections::btree_map::Values;

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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky person");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    // Rust has an extremely powerful control flow construct called match that allows you to
    // compare a value against a series of patterns and then execute code based on which pattern
    // matches. Patterns can be made up of literal values, variable names, wildcards, and many
    // other things.
    let value = value_in_cents(Coin::Penny);
    println!("{value}");
    // Lucky person
    // 1
    // The big difference between if and match is if needs to be evaluated based on boolean, but
    // matches can have any type.

    // Pattersn that Bind to values
    let value = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{value}");
    // State quarter from Alaska!
    // 25

    // Matching with Option<T>
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    // Matches are Exhaustive:
    // The arm's pattern must cover all possibilities. Meaning if we do not cover arm of None in
    // the plus_one function, the compiler will throw error as the option None is not covered.

    // Catch -all Patterns and the _ placeholder:
    // Using enums, we can also take special actions for a few particular values, but for all other
    // values  take one default action. Imagine we're implementing a game where, if you roll a 3 on
    // dice roll, your player doesn't move, but instead gets a new fancy hat. If you roll a 7, your
    // player loses a fancy hat. For all other values, your player moves:
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    // Moving player 9 blocks

    // Rust also has a pattern we can use when we want a catch-all but don't use the value in the
    // catch-all pattern: _:
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
    // Player needs to reroll the dice
}

fn add_fancy_hat() {
    println!("Adding fancy Hat")
}
fn remove_fancy_hat() {
    println!("Removing fancy Hat")
}
fn move_player(num_spaces: u8) {
    println!("Moving player {num_spaces} blocks")
}
fn reroll() {
    println!("Player needs to reroll the dice")
}

// When we call plus_one(five), the variable x in the body of plus_one will have the value Some(5).
// We then compare that agains each match arm:
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
