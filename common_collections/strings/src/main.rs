use std::ops::Add;
use std::path::is_separator;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // Storing UTF-8 encoded in strings
    let mut hello = String::from("नमस्ते");
    println!("{hello}");
    // नमस्ते

    // To update the string
    hello.push_str("in hindi");
    println!("{hello}");
    // नमस्तेin hindi

    // To only push a single character:
    hello.push('!');
    println!("{hello}");
    // नमस्तेin hindi!

    // To concatinate two string, we can use + operation:
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved but s2 is referenced as we cannot move 2 values in
                       // the single variable, And from here on, s1 is not accessible
    println!("{s3}");
    // Hello, world!

    // To add another string:
    let s4 = String::add(String::from("Hello "), &s2);
    println!("{s4}");
    // Hello world!

    let s1 = String::from("tick");
    let s2 = String::from("tack");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
    // tick-tack-toe

    // Indexing into Strings:
    // In many languages, indexing is possible on strings, but is not available in Rust:
    // let s1 = String::from("hello");
    // let h = s1[0];
    // error[E0277]: the type `str` cannot be indexed by `{integer}`
    //   --> src/main.rs:43:16
    //    |
    // 43 |     let h = s1[0];
    //    |                ^ string indices are ranges of `usize`

    // this is happening because In ASCII a single byte contains the actual value of the character,
    // but for UTF-8, it can be multiple bytes combining together making a single character. So if
    // we just try to access the single byte using index, it gives the error as the single byte
    // does not contain any valid information
    // Let's check the byte value of the hello variable:
    let hello = String::from("नमस्ते");
    for c in hello.as_bytes() {
        print!("{c} ");
        // 224 164 168 224 164 174 224 164 184 224 165 141 224 164 164 224 165 135
    }
    // As you can see it takes more than 25 bytes to store the value of Namaste:
    println!();
    // Similarly let's see the Scalar output:

    for c in hello.chars() {
        print!("{c} ");
        // न म स ् त े
    }
    println!();

    // Let's use grapheme cluster crate to handle it more better
    for e in hello.graphemes(true).collect::<Vec<&str>>() {
        print!("{e} ");
        // न म स्ते
    }
    println!();

    // There are many other helper functions such as contains and replace:
    let s = String::from("this is a test string");
    if s.contains("test") {
        println!("Contains test");
    }
    assert_eq!("this is an actual string", s.replace("a test", "an actual"));
}
