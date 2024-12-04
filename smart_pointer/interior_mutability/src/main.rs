pub trait Messenger {
    fn send(&self, msg: &str);
}

fn main() {
    let a = 5;
    // let b = &mut a;
    // error[E0596]: cannot borrow `a` as mutable, as it is not declared as mutable

    let mut c = 10;
    let d = &c;
    // *d = 20;
    // error[E0594]: cannot assign to `*d`, which is behind a `&` reference
}
