use std::hash::Hash;

fn main() {
    // This pattern is useful when you want to take action based on the value received.
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        //..
        _ => println!("anything"),
    }

    // In this case y defined inside of the match case is not same as the y defined outside of
    // match scope
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    // Matched, y = 5

    // Matching multiple patterns
    let x = 5;
    match x {
        1 | 5 => println!("one or five"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    // one or five

    // Matching range of values
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("Something else"),
    }
    // one through five

    let x = 'e';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
    // early ASCII letter

    // Patterns that destructure values
    let p = Point { x: 0, y: 6 };

    // Here we are destructing the struct
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(6, y);

    match p {
        Point { x, y: 0 } => {
            println!("On the x axis at {x}")
        }
        Point { x: 0, y } => {
            println!("On the y axis at {y}")
        }
        Point { x, y } => {
            println!("On another axis: ({x}, {y})")
        }
    }
    // On the y axis at 6

    // Destructuring enums
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("Quit")
        }
        Message::Move { x, y } => {
            println!("Move to x: {x}, y: {y}")
        }
        Message::Write(text) => {
            println!("Text message: {text}")
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color: (r, g, b) = ({r}, {g}, {b})")
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color: (r, s, v) = ({h}, {s}, {v})")
        }
    }
    // Change color: (r, s, v) = (0, 160, 255)

    // prefixing a variable name with _ and just using _ has a different meaning
    let s = Some(String::from("Hello!"));

    // if let Some(_s) = s {
    // This gives error in the println for s
    // error[E0382]: borrow of partially moved value: `s`
    // Whereas this just ignores the value instead of binding it to it.
    if let Some(_) = s {
        println!("found string");
    }
    println!("{:?}", s);

    // If you want to ignore all part of a value except a certain.
    let origin = ThreeDPoint { x: 0, y: 0, z: 0 };

    match origin {
        ThreeDPoint { x, .. } => println!("x is {x}"),
    }
    // x is 0

    let numbers = (2, 4, 6, 8, 10, 12);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first} .. {last}")
        } // (.., second, ..) => {} // error: `..` can only be used once per tuple pattern
    }
    // Some numbers: 2 .. 12

    // Match Guards
    let num = Some(5);

    // Here the if should also match along with the patterns
    match num {
        Some(x) if x < 5 => println!("less than five: {x}"),
        Some(x) => println!("{x}"),
        None => (),
    }
    // less than five: 4

    let x = Some(5);
    let y = 10;

    match x {
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }
    // Default case, x = Some(5)

    // Multiple patterns and have match guard apply to each pattern
    let x = 3;
    let y = true;

    match x {
        // Here x has to be either 4, 5 or 6 and y should be true
        // 4 | 5 | 6 if y => println!("yes"),
        4..=5 if y => println!("yes"),
        _ => println!("no"),
    }
    // no

    // @ operator
    enum CustomMessage {
        Hello { id: i32 },
    }

    let msg = CustomMessage::Hello { id: 5 };

    match msg {
        CustomMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        CustomMessage::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        CustomMessage::Hello { id } => {
            println!("Found Some other id: {id}")
        }
    };
    // Found an id in range: 5
}

struct ThreeDPoint {
    x: i32,
    y: i32,
    z: i32,
}

struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
