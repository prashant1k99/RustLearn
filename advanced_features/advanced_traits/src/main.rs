// Associated types are placeholder which you can add to your trait then methods can implement
// those traits
// The difference between associated types and generics is that with associated type there can
// only be one type, where as generics support multiple types
pub trait Iterator {
    // So if you want to impelement the Iterator type you'll have to define a concrete type for
    // Item. The concrete type is unknown until trait is implemented
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// Example:
struct Counter {}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

// So if we try to implement trait for u16, we will get error
// impl Iterator for Counter {
//     type Item = u16;

//     fn next(&mut self) -> Option<Self::Item> {
//         Some(0)
//     }
// }
// error[E0119]: conflicting implementations of trait `Iterator` for type `Counter`

// So if we want to solve this we will have to implement Generic
pub trait GenericIterator<T> {
    fn next(&mut self) -> Option<T>;
}

struct GenericCounter {}

impl GenericIterator<u32> for GenericCounter {
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}

impl GenericIterator<u16> for GenericCounter {
    fn next(&mut self) -> Option<u16> {
        Some(0)
    }
}
// And we do not get any error
// So when there should be only 1 implementation for a type then we should implement associated
// type instead of Generics

use std::fmt;
// Default Generic Type parameters and Operator Overloading
// Generic type parameters could specify a default concrete type, this allows implementors to not
// have to specify the concret types, unless it's different then the default type
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Implementation of Add trait looks like following:
// trait Add<Rhs = Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }

// Example when we have to specify the type in implementation:
struct Millimeters(u32);
struct Meters(u32);

// Here we are setting the type as we are going to pass the second value as meter
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

// Calling methods with the same name:
// Rust allows you to have 2 traits with the same name and implement both traits on 1 type:
// Incase where there are same names of the traits then you need to mention which trait you are
// refering to. Example:
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("*waving arms furiosly*");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("We are taking off now!!!");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("I call the winds to blow you away in skies...");
    }
}
// For runing example check main function

// However this is not true for associated types because they don't take self as parameters
// Example:
trait AT_Pilot {
    fn fly();
}

trait AT_Wizard {
    fn fly();
}

struct AT_Human;

impl AT_Human {
    fn fly() {
        println!("*waving arms furiosly*");
    }
}

impl AT_Pilot for AT_Human {
    fn fly() {
        println!("We are taking off now!!!");
    }
}

impl AT_Wizard for AT_Human {
    fn fly() {
        println!("I call the winds to blow you away in skies...");
    }
}
// For execution check main function

// Super triats, when a trait has the dependence for a functionality of other traits, example:
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // Now if we need to_string to work, then we need display trait
        let output = self.to_string();
        let len = output.len();

        println!("{output} [{len}]");
    }
}
// Now lets use the OutlinePrint:
struct Point2 {
    x: i32,
    y: i32,
}

// We get error when we just want to implement OutlinePrint trait
// impl OutlinePrint for Point2 {}
// error[E0277]: `Point2` doesn't implement `std::fmt::Display`
// To resolve this we need to add display trait on Point2

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point2 {}
// For execution refer to the main function

// New type pattern
// To get around Orphan rule we implement this
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 2 } + Point { x: 2, y: 1 },
        Point { x: 3, y: 3 }
    );

    // So now if we want to use the traits:
    let person = Human;
    person.fly();
    // *waving arms furiosly*
    // Now for wizards and pilot:
    Pilot::fly(&person);
    // We are taking off now!!!
    Wizard::fly(&person);
    // I call the winds to blow you away in skies...

    // For associated tpye:
    AT_Human::fly();
    // *waving arms furiosly*
    <AT_Human as AT_Wizard>::fly();
    // I call the winds to blow you away in skies...
    <AT_Human as AT_Pilot>::fly();
    // We are taking off now!!!

    let p = Point2 { x: 1, y: 3 };
    p.outline_print();
    // (1, 3) [6]

    // New type pattern
    let w = Wrapper(vec![String::from("Hello"), String::from("World")]);
    println!("w = {w}");
    // w = [Hello, World]
}
