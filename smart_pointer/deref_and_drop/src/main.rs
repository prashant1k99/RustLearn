use std::ops::Deref;

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // Here * derefs the y meaning it goes and gets the value from the reference
    // assert_eq!(5, y);
    // error[E0277]: can't compare `{integer}` with `&{integer}`

    let _y = Box::new(x);
    // Box is the smart pointer that implements deref trait

    // Let's create our own custom smart pointer called my_box
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // Now *y gives error
    // assert_eq!(5, *y);
    // error[E0614]: type `MyBox<{integer}>` cannot be dereferenced

    // To fix this we need to implement deref trait on MyBox struct
    assert_eq!(5, *y);
    // Now it works after implementing deref trait
    // This is equal to
    assert_eq!(5, *(y.deref()));

    // Implicit deref coersion with functions and methods:
    let m = MyBox::new(String::from("Rust"));
    // Here we are passing the reference of MyBox
    hello(&m);
    // &MyBox<String> -> &String -> &str
    // Here rust automatically handles the datatype instead of specifying explicitly
    // hello(&(*m)[..]);

    // Rust does the deref coercion when it finds types and traits impelemtations in three cases:
    // 1) From &T to &U when T: Deref<Target=U>
    // 2) From &mut T to &mut U when T: DerefMut<Target=U>
    // 3) From &mut T to &U when T: Deref<Target=U>
    // Not it does not does from immutable T to mutable U as it needs to follow borrowing rules
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
