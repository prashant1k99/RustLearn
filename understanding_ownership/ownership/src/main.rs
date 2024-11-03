fn main() {
    // Ownership is a set of rules that govermn how a Rust program manages memory. All programs
    // have to manage the way they use a computer's memory while running. Some languages have
    // garbage collection that regularly looks for no-longer-used memory as the program runs; in
    // other languages, te programmer must explicitly allocate and free the memory. Rust uses a
    // thirf approach: memory is managed through a suystem of ownership with a set of rules that
    // the compiler checks. If any of the rules are violated, the program won't complile. None of
    // the features of ownership will slow down your program while it's running.
    //
    // Ownership Riles:
    // First let's take a look at the ownership rules. Keep these rules in mind as we work through
    // the examples that illustrate them:
    // - Each value in Rust has an owner
    // - There can only be one owner at a time
    // - When the owner goes out of scope, the value will be dropped.
    println!("Hello, world!");
    {
        // s is not valid here, it's not yet declared
        let s = "hello"; // s is valid from this point forward

        println!("The value of s is: {}", s)
        // do stuff with s
    } // this scope is now over, and s is no loger available
      // println!("The value of s is: {s}")
      // error[E0425]: cannot find value `s` in this scope
      //   --> src/main.rs:24:35
      //    |
      // 24 |     println!("The value of s is: {s}")
      //    |                                   ^
      //    |
      // help: the binding `s` is available in a different scope in the same function
      //   --> src/main.rs:19:13
      //    |
      // 19 |         let s = "hello"; // s is valid from this point forward
      //    |             ^

    // For more information about this error, try `rustc --explain E0425`.
    // error: could not compile `ownership` (bin "ownership") due to 1 previous error

    // The string type
    let _s = String::from("hello");
    // _s.push_str("Hello");
    // └╴  cannot mutate immutable variable `_s` rust-analyzer (E0384)
    // The double `::` operator allows us to namespace this particular `from` function under the
    // `String` type rather than using some sort of name like `stirng_from`.

    // The mutated string:
    let mut s = String::from("Hello");
    s.push_str(", world!"); // push_str() appends a literal to a string

    println!("{}", s); // This will print `hello, world`

    // Only one owner at a time:
    let s = String::from("Hello");
    let t = s;

    // Now once we have assigned the value of s to t, meaning now t is owner of the value of s. So
    // if we try to access s, we'll get error as following
    // println!("{s}");
    // error[E0382]: borrow of moved value: `s`
    //   --> src/main.rs:57:15
    //    |
    // 54 |     let s = String::from("Hello");
    //    |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
    // 55 |     let t = s;
    //    |             - value moved here
    // 56 |
    // 57 |     println!("{s}");
    //    |               ^^^ value borrowed here after move
    println!("t: {}", t);
    // t: Hello

    // Varibale and Data interating with Clone
    let s1 = String::from("String2");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {}", s2);
    // s1 = String2, s2 = String2

    // Stack-Only Data: Copy
    let x = 5;
    let y = x;
    println!("x = {x}, y = {}", y);
    // x = 5, y = 5;
    // Teh types such as integers have a known size at compile time are stored entirelyon the
    // stack, so copies of the actual values are quick to make. That means there's no reason we
    // would want to prevent `x` from being a valid after we create the variable `y`.
    // Here are some of the types that implement `Copy`:
    // - All the integer types, such as u32.
    // - The Boolean type, `bool`, with values true or false.
    // - The character type, `char`.
    // - Types, if they only contain types that also implement `Copy`. For Example, (i32, i32)
    // implements `Copy`, but (i32, String) does not.

    // When owner goes out of scope, the value is dropped:
    // We cannot access the value of i, once it's out of scope
    {
        let i = String::from("Hello");
        println!("i: {}", i);
        // i: Hello
    }
    // println!("i: ", i);
    // error: argument never used
    //   --> src/main.rs:78:21
    //    |
    // 78 |     println!("i: ", i);
    //    |              -----  ^ argument never used
    //    |              |
    //    |              formatting specifier missing

    // Ownership and Functions:
    ownership_fn();

    // Return value and scope
    return_values_scope();
}

fn return_values_scope() {
    // Returning value can als transfer ownership.

    let _s1 = give_ownership(); // give_ownership moves its return value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves
                                        // its return value into s3
} // Here, s3 goes out of scope and is dropped, s2 was moved, so nothing happens. s1 goes out of
  // scope and is dropped

fn give_ownership() -> String {
    // give_ownership will move its return value into the function
    // calls it
    let some_string = String::from("yours"); // some_stirng comes into scope

    some_string // some_string is returns and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn ownership_fn() {
    let s3 = String::from("hello"); // s3 comes into scope

    take_ownershop(s3); // s3's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    make_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x
                  // afterword
} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn take_ownershop(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed

fn make_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}")
} // Here, some_integer goes out of scope. Nothing special happes.
