### 1) Hello World!
`main` function is important in rust as every execution begins from this function 
```rust
fn main() {
    println!("Hello Worlld")
}
```

### 2) Hello Cargo
Cargo is Rust’s build system and package manager. 
Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. 
(We call the libraries that your code needs dependencies.)

To create a new project in Cargo:
```sh
cargo new <package-name>
```
It will create a new package similar to `npm init`

It has also initialized a new Git repository along with a .gitignore file. 
Git files won’t be generated if you run cargo new within an existing Git repository; you can override this behavior by using `cargo new --vcs=git`

Now the `cargo new` will generate a `Cargo.toml` file and it should contian the following:
```toml
# This contains the pacakge informatiuon
[package]
name = "hello-cargo"
version = "0.1.0"
edition = "2021"
# Here the dependencies of the package will be added
[dependencies]
```

Now with this we can run following comands using cargo in our project:
```sh
cargo build
```
This will build the project, this build will have `target/debug/<project-name>`.
```sh
cargo run
```
this will run the project code with debug mode enabled

```sh
cargo check
```
Cargo also provides a command called cargo check. This command quickly checks your code to make sure it compiles but doesn’t produce an executable:

Let’s recap what we’ve learned so far about Cargo:

- We can create a project using cargo new.
- We can build a project using cargo build.
- We can build and run a project in one step using cargo run.
- We can build a project without producing a binary to check for errors using cargo check.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

