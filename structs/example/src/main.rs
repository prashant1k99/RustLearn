fn main() {
    // Simple implementation without structs and tuples
    simple_impl();

    // Refactored code with tuples:
    refactored_with_tuples();

    // Refactoring with Structs: Adding more meaning:
    refactored_with_structs();

    // To be able to log the Struct values
    // Add `#[derive(Debug)]` on top of the struct
    // Then it can be logged as following:
    // println!("rect1 is {rect1:?}");
    // rect1 is Rectangle { width: 30, height: 50 }
    // This logs the type and fields
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn refactored_with_structs() {
    // We use structs to add meaning by labeling the data. We can transform the tuple we're using
    // into a struct with a name for the whole as well as names for the parts
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {rect1:?}");
    println!(
        "The area of the rectanlge is {} square pixels.",
        refactored_area_structs(&rect1)
    );
    // The area of the rectanlge is 1500 square pixels.
}

fn refactored_area_structs(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn refactored_with_tuples() {
    let rect = (30, 50);

    println!(
        "The area of the rectanlge is {} square pixels.",
        refactored_tuple_area(rect)
    );
    // The area of the rectanlge is 1500 square pixels.
}

fn refactored_tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn simple_impl() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectanlge is {} square pixels.",
        simple_area(width, height)
    );
    // The area of the rectanlge is 1500 square pixels.
}

fn simple_area(w: u32, h: u32) -> u32 {
    w * h
}
