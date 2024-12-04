use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    // a initial rc count = 1 println!("a next item = {:?}", a.tail());
    // a next item = Some(RefCell { value: Nil })

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // a rc count after b creation = 2
    println!("b initial rc count = {}", Rc::strong_count(&b));
    // b initial rc count = 1
    println!("b next item = {:?}", b.tail());
    // b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // b rc count after changing a = 2
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // a rc count after changing a = 2

    // Uncomment the next line to see that we have a cycle it will overflow stack
    // println!("a next item = {:?}", a.tail());
    // thread 'main' has overflowed its stack
    // fatal runtime error: stack overflow
    // Explanation:
    // Stack | Heap
    //  a    -> 'a (5, 'b)
    //  b    -> 'b (10, 'a)
    //  Since we are trying to print a then we need to print b as it is it's value, then if we
    //  print b we will again have to print a, resulting into stack overflow problem

    // Now to tackle the problem of waek pointers: where a pointer to the value which it does not
    // own

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // leaf parent = None
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    // leaf strong = 1, weak = 0

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // Since weak smart pointer has no idea if the value is dropped or not, we have to call upgrade
    // then we have access to that iformation
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) }, children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } })
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
