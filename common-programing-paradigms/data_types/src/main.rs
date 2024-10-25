use std::io;

fn floating_number() {
    let x = 2.0;
    println!("Value of x is: {}", x);
    // Value of x is: 2

    let y: f32 = 3.0;
    println!("Value of y is: {}", y);
    // Value of y is: 3
}

fn numeric_operations() {
    let sum = 5 + 10;
    println!("The sum is {}", sum);
    // The sum is 15

    let differnece = 95.5 - 4.2;
    println!("The difference is {}", differnece);
    // The difference is 91.3

    let product = 4 * 20;
    println!("The product is {}", product);
    // The product is 80

    let quotation = 56.7 / 32.2;
    println!("The quotation is {}", quotation);
    // The quotation is 1.7608695652173911
    let truncated = -5 / 3;
    println!("The truncated is {}", truncated);
    // The truncated is -1

    let remainder = 43 % 5;
    println!("The remainder is {}", remainder);
    // The remainder is 3
}

fn boolean_types() {
    // The main way to use Boolean is through conditionals such as an `if` expression.
    let t = true;
    println!("t is {}", t);
    // t is true

    let f: bool = false;
    println!("f is {}", f);
    // f is false
}

fn character_types() {
    let c = 'z';
    println!("c  is {}", c);
    // c  is z

    let z: char = 'ℤ'; // with explicit type annotation
    println!("z  is {}", z);
    // z  is ℤ

    let hearted_eye_cat = '😻';
    println!("emojoi  is {}", hearted_eye_cat);
    // emojoi  is 😻
}

fn base_data_types() {
    // To declare a variable a type should be declared otherwise it gives error:
    // let guess = "43".parse().expect("Not a number");
    // 3 |     let guess = "43".parse().expect("Not a number");
    //   |         ^^^^^        ----- type must be known at this point

    //   Proper way is:
    let guess: u32 = "43".parse().expect("Not a number");
    println!("You guessed: {}", guess);
    // You guessed: 43

    floating_number();

    numeric_operations();

    boolean_types();

    character_types();
}

fn tuple_type() {
    // A tuyple is a general way of grouping together a number of values with a variety of types
    // into one compound type. Tuples have a fixed length: once declared, they cannot grow or
    // shrink in size
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // It can also be declared like this
    let _tup = (400, 2.2, 1);

    let (x, y, z) = tup;
    println!("The value of x, y, z is {}, {}, {} respectively", x, y, z);
    // The value of x, y, z is 500, 6.4, 1 respectively

    // We can laso access a tuple element directly by using a period (.) followed by the index of
    // the value we want to acces. For example:
    let five_hundred = tup.0;
    println!("The value of tup.0 is {}", five_hundred);
    // The value of tup.0 is 500

    let six_point_four = tup.1;
    println!("The value of tup.1 is {}", six_point_four);
    // The value of tup.1 is 6.4

    let one = tup.2;
    println!("The value of tup.2 is {}", one);
    // The value of tup.2 is 1
}

fn array_types() {
    // Another way to have a collection of multiple values is with an array.
    // Unlike a tuple, every element of an array must have the same type. Unlike arrays in some
    // other languiages, arrays in Rust have a fixed length.
    let _a = [1, 2, 3, 4, 5, 6];

    // You can directly print an array as you'll get the error as following
    // println!("The value of a is {}", a);
    // error[E0277]: `[{integer}; 6]` doesn't implement `std::fmt::Display`
    //    --> src/main.rs:114:38
    //     |
    // 114 |     println!("The value of a is {}", a);
    //     |                                      ^ `[{integer}; 6]` cannot be formatted with the default formatter

    // We can also define an array as follwing:
    let _b: [i32; 5] = [1, 2, 3, 4, 5];
    // Where i32 sets the data type and 5 is setting the size of the array

    // Here we are setting the value as 3 for the length of 5
    let _c = [3; 5]; // [3,3,3,3,3]

    // Accessing array:
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    println!("first is {}", first);
    // first is 1

    let second = a[1];
    println!("second is {}", second);
    // second is 2
}

fn invalid_array_access() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is {}", element);
    // Please enter an array index.
    // 7
    // thread 'main' panicked at src/main.rs:158:19:
    // index out of bounds: the len is 5 but the index is 7
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}

fn compound_data_types() {
    // Compound types can group multiple values into one type. Rust has 2 primitive compound types:
    // tuples and arrays.

    tuple_type();

    array_types();

    invalid_array_access();
}

fn main() {
    base_data_types();

    compound_data_types();
}
