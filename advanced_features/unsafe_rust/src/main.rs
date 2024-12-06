// Unsafe Rust exists, so that we can write Low level systems programming even writing our own
// Operating System.
// Abilities of Unsafe Rust:
// - Dereference a raw pointer
// - Call an unsafe function or a method
// - Acces or modify a mutable static variable
// - Implement an unsafe trait
// - Access fields of a union
//
// Note: unsafe does not disables the Borrow checker from the code

use core::slice;

fn main() {
    // Raw pointers:
    let mut num = 5;

    // this is an immutable raw pointer
    let r1 = &num as *const i32; // *const i32

    // this is a mutable raw pointer
    let r2 = &mut num as *mut i32; // *mut i32

    // Differences between Raw pointers and references and smart pointers:
    // - Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or
    // multiple mutable pointers to the same location
    // - Aren't guarenteed to point to a valid memory
    // - Are allowed to be null
    // - Don't implement any automatic cleanup

    // This is a random memory address where we are not sure what lies
    // let address = 0x012345usize;
    // let r3 = address as *const i32;

    // 1) To dereference a raw pointers, we need to create an unsafe block:
    unsafe {
        println!("r1 is: {}", *r1);
        // r1 is: 5
        println!("r2 is: {}", *r2);
        // r2 is: 5
    }

    // 2) Ability to call unsafe functions and methods in our code

    // Unsafe functions and methods are same as our usual implementations, except they have unsafe
    // keyword in front of them:
    unsafe fn dangerous() {}
    // unsafe functions and methods do not adheer to strict checking of parameter types, etc..
    // Where developer is responsible of verifying all the parameters and execution of code
    unsafe {
        // Unsafe functions must be called from inside of other unsafe functions or unsafe block.
        dangerous();
    }

    // To create a safe abstraction over unsafe code:
    // We can wrap unsafe functions inside safe functions.
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // We'll try to implement split_at_mut in safe code and we'll see errors, even though we are
    // sure that it's correct.
    // let (a, b) = split_at_mut(r, 3);

    // Unsafe implementation:
    let (a, b) = split_at_mut_unsafe(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Functions that call external code:
    unsafe { println!("Absolute value of -3 according to C: {}", abs(-3)) }

    // To call our Rust code from other languages, use no_mangle attribute
    // Check teh function called `call_from_c` to check

    // Ability to access and modify mutable static variable
    // global variables are called static in Rust
    // They are not like constants as the static variables can be mutated unlike constants
    // And constants duplicate the data instead of sharing the same copy, whereas static variables
    // share the same data
    println!("name is {HELLO_WORLD}");
    // name is Hello World

    // 3) Accessing anf modifying static variables is unsafe as it can potentially lead to race
    // condition
    add_to_count(3);

    unsafe {
        println!("COUNTER: {COUNTER}");
    }
    // COUNTER: 3

    // 5) Access fields of Union
    // Unions are similar to structs but only one declared field is used in a particular instance
    // at one time
    // Unions are primarly used to interact with C unions therefore Rust cannot guarentee the
    // datatypes of return values
}

// 4) Unsafe Trait implementation:
unsafe trait FOO {
    // methods go here
}

unsafe impl FOO for i32 {
    // method implementation go here
}

static HELLO_WORLD: &str = "Hello World";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// Now if we were to impelement split_at_mut in safe rust code:
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();

//     assert!(mid <= len);

//     // Here rust is complaining that we are trying to use the same mutable variable twice in the
//     // same block as it's not smart enough to know that the values accessed are different
//     (&mut slice[..mid], &mut slice[mid..])
// }

// To implement this in unsafe code:
fn split_at_mut_unsafe(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            // Here we need to pass the pointer of start and till the position we need
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Here we declare the expected types from the external code
extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
