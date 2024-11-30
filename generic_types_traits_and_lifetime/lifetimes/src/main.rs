fn main() {
    // dangling References:
    // Uncomment the below block of code
    // let result; // 'a
    // {
    //     let y = 10;
    //     // result = &y;
    //     // Error: y does not live long enough to  borrowed value does not live long enough
    // }
    // println!("result = {result}")

    // This is a dangling reference as we result is holding reference of something which is out of
    // scope and is cleared by Rust garbage collector
    // So the lifetime of result is throught the complete application but the lifetime of the
    // variable y is limited to the scope and block

    // now let's take some example
    let s1 = String::from("PSC");
    let result: &str;

    {
        let _s2 = String::from("Prashant Singh");
        // result = longest(&s1, &_s2);
        // Since we have implemented the lifetime in the function compiler gives us the error:
        // ├╴  `s2` does not live long enough
        // │    borrowed value does not live long enough rustc (E0597) [23, 31]

        // Forcing us to make sure that both the strings and result should be of the same lifetime.
        // solution to that is
    }

    let s2 = String::from("Prashant Singh");
    result = longest(&s1, &s2);
    println!("Longest String is {result}");

    // There are three rules to the lifetime:
    // 1) The compiler assignes a lifetime parameter to each parameter that's a reference. In other
    //    words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32)`
    //    A function with two parameters gets two seperate lifetime parameters: `fn foo<'a, 'b>(x:
    //    &'a i32, y: &'b i32) -> &'a i32`
    // 2) If tere is exactly one input lifetime parametere, that lifetime is assignes to all output
    //    lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`
    //    Meaning the lifetime of the output will be the same as the input lifetime in case of
    //    single parameter and we do not have to specify the output lifetime
    // 3) If there are multiple input lifetime parameters, but one of them is &self or &mut self
    //    because this is a method, the lifetime of self is assigned to all teh output lifetime
    //    parameters. This third rule makes methods much nicer to read and write because fewer
    //    symbols are necessary
}

// This gives us the error
// ├╴  missing lifetime specifier
// │    this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y` rustc (E0106) [19, 33]
// Becuase the lifetime of the x and y is undefined making it uncertain about the lifetime of the
// return value, making it a potential dangling reference
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         return x;
//     }
//     y
// }
// to fix this, let's re-write this:
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    y
}
