use std::rc::Rc;

enum List {
    // To solve the below problem
    // Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // Let's say we want to create a Cons list where 2 parent can have same 1 child node.
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // But this gives us an error as the value of a is already moved to b
    // To fix that we will change the enum List
    // let c = Cons(4, Box::new(a));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // count after creating a = 1
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    // count after creating b = 2

    // We can also write it as
    // let _b = Cons(3, a.clone());

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        // count after creating c = 3
    }

    println!(
        "count after after c goes out of scope {}",
        Rc::strong_count(&a)
    );
    // count after after c goes out of scope 2
    // Note that this does not clones the data but just keeps the counter of references
}
