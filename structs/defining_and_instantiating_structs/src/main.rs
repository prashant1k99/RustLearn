#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // Structs are siumilar to tuples, in that both hold multiple related values. Like tuples, the
    // pieces of a struct can be differnet types. Unlike with tuples, in a struct you'll name each
    // piece of data so it's clear what the values mean

    let user1 = User {
        active: true,
        username: String::from("some_username"),
        email: String::from("some@user.gom"),
        sign_in_count: 1,
    };

    // We cannot change a single field after initialization if the struct is immutable.if you want to
    // modify a field make the user struct mutable
    // user1.email = String::from("another@user.com");
    // error[E0594]: cannot assign to `user1.email`, as `user1` is not declared as mutable
    //   --> src/main.rs:21:5
    //    |
    // 21 |     user1.email = String::from("another@user.com");
    //    |     ^^^^^^^^^^^ cannot assign

    let mut user2 = build_user(
        String::from("second@user.com"),
        String::from("some@email.com"),
    );

    user2.active = false;
    println!("{:?}", user2);
    // User { username: "some@email.com", email: "second@user.com", sign_in_count: 1, active: false }

    // Creating instances from other instances with Struct Update syntax
    let _user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@email.com"),
        sign_in_count: user1.sign_in_count,
    };
    // or a simple approach would be expand the struct.
    let user3 = User {
        email: String::from("another@email.com"),
        ..user2
    };
    println!("user3.email: {}", user3.email);

    // Using Tuple Structs Without Named fields to create different types
    tuple_struct();

    // Unit-Like Structs without Any Fields
    unit_structs();
}

struct AlwaysEqual;

fn unit_structs() {
    // You can also define structs that don't have any fields! These are called unit-like structs
    // because they behave similarly to `()`. Unit-like structs can be useful when you need to
    // implement a trait on some type but don't have any data that you want to store in the type
    // itself.
    let _subject = AlwaysEqual;
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_struct() {
    // Rust also supports struct that look similar to tuples, called tuple structs. Tuple structs
    // have the added meaning the struct name provides but don't have names associated with their
    // fields;
    let black = Color(0, 0, 0);
    println!("{}", black.0);

    let origin = Point(0, 0, 0);
    println!("{}", origin.0)
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // Similar to Javascript, we can initialize with shorthand
        username,
        email,
        sign_in_count: 1,
    }
}
