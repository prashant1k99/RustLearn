fn main() {
    let list = vec![1, 2, 5, 6, 7, 8, 5, 10, 4];
    let l = largest(&list);

    println!("The largest number is {l}");
    // The largest number is 10

    // Now the problem begins if we need to include the float as well:
    let list2 = vec![1.2, 2.43, 4.1, 6.12, 2.8, 64.0];
    // Now we need to create a seperate function to handle float values:
    // Or we can make the largest function generic to handle ints and/or floats
    let l = largest_generic(&list);
    println!("The largest number is {l}");
    // The largest number is 10
    let l = largest_generic(&list2);
    println!("The largest number is {l}");
    // The largest number is 64

    // In Struct
    let _point_a = Point { x: 10, y: 20 };
    // Now if we want to use the float instead of i32:
    let _point_a = PointWithGeneric { x: 10.4, y: 20.0 };
    // Note that we need to use the same type for both x and y.

    // to solve the problem where, we may want to pass multiple types:
    let _point_a = PointWithMultipleGenerics { x: 10, y: 23.45 };
    // Since we have implemented generics on the method as well, now we can use the method as
    // following:
    let point_a = PointWithMultipleGenerics::new(10, 25.33);
    // point.calc
    let point_b = PointWithMultipleGenerics::new(25.44, 20.50);
    // This method is only available as the types are f64, f64
    point_b.calculate_distance();
    // Now let's call the mixup fn:
    let point_c = point_a.mixup(point_b);
    println!("{point_c:?}");
    // PointWithMultipleGenerics { x: 10, y: 20.50 }
}

#[derive(Debug)]
struct PointWithMultipleGenerics<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointWithMultipleGenerics<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }

    fn mixup<X, Y>(
        self,
        point: PointWithMultipleGenerics<X, Y>,
    ) -> PointWithMultipleGenerics<T, Y> {
        PointWithMultipleGenerics {
            x: self.x,
            y: point.y,
        }
    }
}

// This method calculate_distance will not be present if the type is not matching
impl PointWithMultipleGenerics<f64, f64> {
    fn calculate_distance(&self) -> f64 {
        4.0
    }
}

struct PointWithGeneric<T> {
    x: T,
    y: T,
}

struct Point {
    x: i32,
    y: i32,
}

// We are using PartialOrd to specify that the values which are coming are of type which can pe
// compared, ordered, etc..
fn largest_generic<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if largest < item {
            largest = item;
        }
    }

    largest
}

// After creating the function larged_generic, this function becomes void
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if largest < item {
            largest = item;
        }
    }

    largest
}
