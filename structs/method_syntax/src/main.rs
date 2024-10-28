#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // Associated functions:
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 25,
        height: 45,
    };

    println!("The area of rectangle is {} square pixels.", rect1.area());
    // The area of rectangle is 1500 square pixels.

    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    // Can rect1 hold rect2: true

    // Associated functions: All functions defined within an `impl` block are called associated
    // functions because they're associated with the type named after the `impl`. We can define
    // associated functions that don't have `self` as their first parameter (and thus are not
    // methods) because they don't need an instance of the type to work with. We've already used
    // one function like this: the String::from function that's defined on the `String` type.
    let rect3 = Rectangle::square(12);
    println!("rect3 width is {}", rect3.width);
    // rect3 width is 12
}
