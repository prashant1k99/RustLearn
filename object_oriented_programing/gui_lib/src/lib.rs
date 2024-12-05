// Let's make a library where every component should have a method called Draw, so that we can use
// those components
pub trait Draw {
    fn draw(&self);
}

// Generic implementation:
// When using generics, we are limited by 1 type, meaning only 1 type is used
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

// Trait Object implementation:
// It supports any type as long as it has the trait implemented
// It has some runtime cost
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // Draw button 1
        println!("Draw button: {:?}", self);
    }
}
