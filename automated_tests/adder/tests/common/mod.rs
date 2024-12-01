// If we create directly comon.rs in tests folder, then Rust will treat it as an integration test,
// instead of setting it as a common helper module.

pub fn setup() {
    println!("Setting up test...")
}

pub fn end() {
    println!("Test completed")
}
