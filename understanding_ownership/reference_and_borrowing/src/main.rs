fn main() {
    // Sometimes we want to access the data without transferring ownership. We can do this by
    // borrowing using (&) references:
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
    // The length of 'hello' is 5.

    // Mutable reference:
    let mut s2 = String::from("Hello");
    println!("s2 before: {s2}");
    // s2 before: Hello
    change(&mut s2);
    println!("s2 is not: {s2}");
    // s2 is not: Hello, world

    // Mutable references have one big restriction: if you have a mutable reference to a value, you
    // can have no other referene to that value. This code that attepts to create two mutable
    // references to `s` will fail:
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{r1}, {r2}");
    // error[E0499]: cannot borrow `s` as mutable more than once at a time
    //   --> src/main.rs:25:14
    //    |
    // 24 |     let r1 = &mut s;
    //    |              ------ first mutable borrow occurs here
    // 25 |     let r2 = &mut s;
    //    |              ^^^^^^ second mutable borrow occurs here
    // 26 |
    // 27 |     println!("{r1}, {r2}");
    //    |               ---- first borrow later used here

    // The benefit of having this restriction is that Rust can prevent data races at compile time.
    // A data race is similar to a race condition and happens when these three behaviours occur:
    // - Two or more poiunters access the same data at the same time.
    // - At least one of the pointers is being used to write to the data.
    // - There's no mechanism being used to synchronizes access to the data.
    allow_multiple_mutables();

    // Rust enforces a similar rule of combinig mutable and immutable references:
    combining_mutable_immutable();

    // Dangling References:
    // In languages with pointers, it's easy to erronously create a dangling pointer - a pointer
    // that references a location in memory that may have been given to someone else-by freeing
    // some meory while preserving a pointer to that memory. In Rust, by contrast, the compiler
    // guarantees that references will never be dangling references: if you have a reference to
    // some data, the compiler ensures that data will not go out of scope before the reference to
    // that data does.
    dangling_pointer();

    // Rules of References:
    // - At any given time, you can have either one mutable reference or any number of immutable
    // references.
    // - References must always be valid.
}

fn dangling_pointer() {
    // let reference_toNothing = dangle();
    let value = no_dangle();
    println!("{value}")
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
//     // error[E0515]: cannot return reference to local variable `s`
//     //   --> src/main.rs:66:5
//     //    |
//     // 66 |     &s
//     //    |     ^^ returns a reference to data owned by the current function
// }
fn no_dangle() -> String {
    String::from("Hello")
}

fn combining_mutable_immutable() {
    let mut s = String::from("Hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    // let r3 = &mut s; // BIG problem
    // println!("{}, {} and {}", r1, r2, r3);
    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    //   --> src/main.rs:56:14
    //    |
    // 53 |     let r1 = &s; // no problem
    //    |              -- immutable borrow occurs here
    // ...
    // 56 |     let r3 = &mut s; // BIG problem
    //    |              ^^^^^^ mutable borrow occurs here
    // ...
    // 59 |     println!("{}, {} and {}", r1, r2, r3);
    //    |                               -- immutable borrow later used here

    // Solution to that is use the immutable reference before initializtion:
    println!("{r1} adn {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s;
    println!("{r3}");
}

fn allow_multiple_mutables() {
    let mut s = String::from("hello");
    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference wioth no problems.
    let _r2 = &mut s;
}

fn change(some_stirng: &mut String) {
    some_stirng.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String

    // We cannot modify it as this is immutable reference
    // s.push_str(" ");
    s.len()
} // Here, s goes out of scope. But becuase it does not have ownership of what it refers to, it is
  // not dropeed
