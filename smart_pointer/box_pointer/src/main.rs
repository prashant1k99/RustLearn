// This is called conslist commonly known in functional programming
enum List {
    // Cons(i32, List),
    // To fix it we use Box
    Cons(i32, Box<List>),
    // Similarly Rust when Rust tries to figure out what's the maximum size possible for enum, it
    // now sees that Cons have a fixed size as Box has a fix length which is a pointer to the heap.
    // So for stack it's a fixed length
    Nil,
}

use List::{Cons, Nil};

// Let's use some last example Message
// Now Rust tries to find the maximum size possible for the Message enum and sets its size no
// matter what you pass, be it nil enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    // The value 5 is stored on the heap
    let b = Box::new(5);
    // Usually box is used when you do not know the value of something at compile time but you need
    // it's size on runtime
    // Or when you have large amount of data and you want to transfer the data but you don't want
    // to make a copy of it as it is a large data
    // When you own a value and you want to make sure that it implements a specific trait rather
    // than a specific type [Trait Object]
    println!("b = {b}");
    // when b is out of scope it will de-allocate the data

    // Now for the enum List we do not know how much size List can have as it is of recursive type
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
