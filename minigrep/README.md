# MiniGrep

This is a practice CLI for Rust lang...
It accepts 2 commnd line arguments: the filepath and a string to search for.

It should run as following for testing:
```sh
cargo run -- searching example-filename.txt
```

## Learnings:
This are the learnings for the follwing chapters

1) Accepting Command Line Argument
We achieve that using standard package of env.
```rust
use std::env;

fn main() {
    // To get the arguments pased in the function
    let args: Vec<String> = env::args().collect();
    // The first argument in the args will be the calling of the binary target
    // "target/debug/minigrep",

    // To save the arguments values in the variable:
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    // Searching for hello
    println!("In file {file_path}");
    // In file example.txt
}

```

2) To Read the contents of the file, use the following code:
```rust
...
    // To Read the file's content:
    let contents = match fs::read_to_string(file_path) {
        Ok(val) => val,
        Err(_) => {
            panic!("Unable to read file")
        }
    };
    println!("With Text:\n {contents}");
...
```
