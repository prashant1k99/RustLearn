enum IpAdddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAdddrKind,
    address: String,
}

enum IpAddr1 {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // Where structs give you a way of grouping together related fields and data, like a `Rectangle`
    // with its `width` and `height`, enums give you a way of saying a value is one of a possible
    // set of values.
    let _four = IpAdddrKind::V4;
    // println!("{four}");
    // We cannot print four as IpAddrKind is a Null Value enum
    let _size = IpAdddrKind::V6;

    // We can use it in combination with struct:
    let _home = IpAddr {
        kind: IpAdddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    // Enum with values:
    let _home = IpAddr1::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr1::V6(String::from("::1"));

    let _home = IpAddr2::V4(127, 0, 0, 1);
    let _loopback = IpAddr2::V6(String::from("::1"));

    // Let's look at another example of an enum with wide variety of types embedded in its
    // variatns.
    let m = Message::Write(String::from("Hello"));
    m.call();

    // The Option Enum and Its Advantage over Null Values:
    // The problem with null values is that if you try to use a null value as a no-null value,
    // you'll get  an error of some kind. Because this nulll or not-null property is pervasive,
    // it's extremely easy to make this kind of error.
    // This enum is Option<T>, and it is defined by the standard library.
    //  enum Option<T> {
    //     None,
    //     Some(T),
    // }

    let _some_number = Some(4);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;

    // Because Option<T> and T (Where T can be any type) are different types, teh compiler won't
    // let us use an Option<T> value as if it were definitly a valid value. For example, this code
    // won't complile, becaude it's tryingh to add an i8 to an Option<i8>
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;
    // error[E0277]: cannot add `Option<i8>` to `i8`
    //   --> src/main.rs:69:17
    //    |
    // 69 |     let sum = x + y;
    //    |                 ^ no implementation for `i8 + Option<i8>`
    //    |
    //    = help: the trait `Add<Option<i8>>` is not implemented for `i8`
}

enum Message {
    Quit,                       // `Quit` has no data associated with it at all.
    Move { x: i32, y: i32 },    // `Move` has named fields, like a struct does.
    Write(String),              // `Write` includes a single `String`.
    ChangeColor(i32, i32, i32), // `ChangeColor` includes three `i32` values.
}
// There is one more similarity between enums and structs: just as we're able to define methods on
// structs using `impl`, we're also able to define methods on enums. Here's a method named `call`
// that we could define on our `Message` enum:
impl Message {
    fn call(&self) {
        // Method body would be defined hgere
    }
}
