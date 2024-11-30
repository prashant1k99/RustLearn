fn immutable_vars() {
    let i = 10;
    println!("The value of i is {i}");
    // The value of i is 10

    // You cannot mutate the immutable values as these values can never be changed
    // i = 20;
    // error[E0384]: cannot assign twice to immutable variable `i`
    // In order to make a variable with such that it can be later changed, use the keyword called
    // mut

    let mut j = 10;
    println!("The value of j is {j}");
    // The value of j is 10

    j = 20;
    println!("The updated value of j is {j}");
    // The updated value of j is 20
}

// This is constant, for constant we have to specify the datatype along with the declaration and
// initializartion
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn shadowing() {
    // by shadowing we do not make the variable mutable, it only becomes re-assigneable but we
    // cannot simply keep on re-assigning it without let

    let x = 5;
    println!("Value of x is {x}");
    // Value of x is 5

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
        // The value of x in the inner scope is: 10
    }

    // The outer value of x cannot be changed by the inner logic
    let x = x + 1;
    println!("Updated value of x is {x}");
    // Updated value of x is 6

    let spaces = "   ";
    let spaces = spaces.len();
    println!("How many spaces: {spaces}")
    // How many spaces: 3
}

fn main() {
    println!("===============");
    immutable_vars();
    println!("===============");
    println!("Constant var value is {THREE_HOURS_IN_SECONDS}");
    println!("===============");
    shadowing();
}
