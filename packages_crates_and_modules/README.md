# Managing Growing Projects with Packages, Crates and Modules

As you write large programs, organizing code will become increasingly important. By grouping related functionality and seperating code with distinct features, you'll clarify where to find coode that implements a particular feature and where to go to change how a feature works.

Rust has a number of features that allow you to manage you code's organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, something collectively referred to as the *module system*, include:

- **Pacakge**: A Cargo feature that lets you build, test, and share crates
- **Crates**: A tree of modules that produces a library or executable
- **Modules** and **use**: Let you control the organization, scope and privacy of paths
- **Paths**: A way of naming an item, such as a struct, function or module

## Packages and Crates:
A crate is the smallest amount of code that the Rust compiler considers at a time. Even if you run `rustc` rather than `cargo` and pass a single source code file, the compiler considers that file to be a crate. Crates cna contain modules, and the modules may be defined  in other files that get compiled with the crate, as we'll see in the coming sections. A crate can come in one of two forms: a binary crate or a library crate. Binary crates are programs you can compile to an executable that you can run, such as a command-line program or a server. Each must have a function called `main` that defines what happens when executable runs. All the crates we've creaated so far have been binary crates.
    
Library crates don't have `main` function, and they don't compile to an executable. Instead, they define functionailty intended to be shared with multiple projects. For example, the `rand` crate we used in `basics/guessing_game` proivides functionality that generates random numbers. Most of the time when Rustaceans say "crate", they mean library crate, and they use "crate" interchangeably with the general programming concept of a "library".
    
The `crate root` is a source file that Rust compiler starts from and makes up the root module of your crate

A `package` is a bundle of one or more crates that provides a set of functionality. A package contains a *Cargo.toml* file that describes how to build those crates. Cargo is actually a pacakge that contains teh binary creaet for the cli tool you've been using to build your code.
The Cargo pacakge also contains a library craet that the binary create depends on. Other projects can depend on the Cargo library create to use the same logic the Cargo cli tool uses. A package can contain as many binary creates as you like, but at most only one library create. A pacakge must contain at least one create, whether that's a library or binary crate.

Let's walk through what happens when we create a package. First we enter the command `cargo new my-project`:
```sh
$ cargo new my-project
    Creating binary (application) `my-project` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

$ ls my-project
Cargo.toml      src

$ ls my-project/src
main.rs

```

## Defining Modules to Control Scope and Privacy
In this section, we'll talk about modules and other parts of the module system, namely *paths*, which allow you to name iutems; the `use` keyword that brings a path into scope; and the `pub` keyword to make items public. We'll also discuss  the `as` keyword, external pacakges, and teh glob operator.

### Modules Chear Sheet

Before we get to the details of modules and paths, here we provide a quick reference on how modules, paths, the `use` keyword, and the `pub` keyword work in teh compiler, and how most developers organize their code. We'll be going throuigh examples of each of these rules thorughout this chapter, but this is a great place to refer to as reminder of how modules work.

- **Start from the create root**: When compiling a create, the compiler first looks in the create root file (usually *src/lib.rs* for a library crate or a *src/main.rs* for a binary create) for a code to compiler.

- **Declaring modules**: In the create root file, you can declare new modules; say you declare a "garden" module with `mod garden;`. The compiler will look for the module's code in these places:
    - Inline, within curly brackets that replace the semicolon following `mod garden`
    - In the file *src/garden.rs*
    - In the file *src/garden/mod.rs*

- **Declaring submodules**: In any file other than the create root, you can declare submodules. For example, you might declare `mod vegetables;` in `src/garden.rs`. The compiler will look for the submodule's code within the directory named for the parent module in theses places:
    - Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon
    - In the file *src/garden/vegetables.rs*
    - In the file *src/garden/vegetables/mod.rs*

- **Paths to code in modules**: Once a module is part of your create, you can refer to code in that module from anywhere else in that same create, as long as the privacy rules allow, using the path to the code. For example, an `Asperagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`.

- **Private vs. public**: Code within a module is private from its parent modules by detault. To make a module public, declare it with `pub mod` instead of `mod`. To make items within a publiuc module public as well, use `pub` befire their declarations.

- **The `use` keyword**: Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to
`crate::garden::vegetables::Asparagus`, you can create a shortcut with `use` 
`crate::garden::vegetables::Asparagus;` and from then on you only need to write `Asparagus` to make use of that type in the scope.

Here, we create a binary named `backyard` that illustrates these rules. The crate's directory also named `backyard`, contains these files and directories:
```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```
the crate root file in this case is *src/main.rs*, and it contains:

Filename: src/main.rs
```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
```

The `pub mod garden;` line tells the compiler to include the code it finds in `src/garden.rs`, which is:
Filename: src/garden.rs
```rust
pub mod vegetables;
```

Here, `pub mod vegetables;` means the code in *src/garden/vegetables.rs* is included too. That code is:
```rust
#[derive(Debug)]
pub struct Asparagus {}
```

Now let's get into the details of these rules and demostrate them in action

### Grouping Related Code in Modules

*Modules* let use organize code within a crate for readability and easy reuse. Modules also allow us to control the *privacy* of items because code within a module is private by default. Private items are internal implemetnation details not available for outside use. We can choose to make modules and items within them public, which exposes them to allow external code to use and depend on them.

