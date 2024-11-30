fn main() {
    // Slices let you reference a cotiguous sequence of elements in a collection rather than the
    // whole colelciton. A slice is a kind of reference, so it does not have ownership.

    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    println!("first word: {word}");

    let sw = second_word(&s);
    println!("Second word: {sw}");

    let slice = &s[..2];

    println!("{slice}");

    // let slice = &s[3..len]; // Same as below
    let slice = &s[3..];
    println!("{slice}");
    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that we could meaningfully use
    // the value 5 with. word is now totally invalid
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut space_counter = 0;
    let mut starting_index = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if space_counter == 0 {
                space_counter += 1;
                starting_index = i + 1;
            } else {
                return &s[starting_index..i];
            }
        }
    }

    &s[starting_index..]
}

// the first_word function has &String as a parameter. We don't want ownership, so this is fine.
// But what should we return? We don't really have a way to talk about part of a string. However,
// we could return the index of the end of the word, indicated by a space.
fn first_word(s: &String) -> &str {
    // Because we need to go through the `String` element by element and check whether a valus is a
    // space, we'll convert our `String` to an array of bytes using `as_bytes` method.
    let bytes = s.as_bytes();

    // We create an iterator over the array of bytes using the `iter` method:
    // `iter` retursn each element in a collection and that `enumerate` wraps the result of `iter`
    // and returns each element as part of a tuple instead. The first element of the tuple returned
    // by `enumerate` is the index, and the second element is a reference to the element. This is a
    // bit more convinient than calculating the index oursleves.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
