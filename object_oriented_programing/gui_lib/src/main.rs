use gui_lib::{Button, Draw, Screen};

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing select box: {:?}", self);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            // Let's add an element which does not implements draw trait
            // Box::new(String::from("test")),
            // error[E0277]: the trait bound `String: Draw` is not satisfied
            Box::new(SelectBox {
                width: 100,
                height: 100,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe"),
                ],
            }),
            Box::new(Button {
                width: 15,
                height: 50,
                label: String::from("This is a button"),
            }),
        ],
    };

    screen.run();
}
