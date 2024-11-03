fn main() {
    // Vectors are simply growable arrays
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    // Cannot add String to teh vec as it infered type number from the vec.push(1)
    // vec.push("Hello");
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

    dbg!(vec);

    // Alternative short hand, wheere it is immutable by default
    let vec2 = vec!['a', 'b', 'c'];
    println!("Vector2: {:?}", vec2);

    // Reading elements of Vecotrs:
    // There are two ways, index and get method

    // With Index:
    // This can potentially crash the application for index out of bound issue
    let second_value = &vec2[1];
    println!("second_value is {second_value}");
    // second_value is b

    // get method:
    let third_value = vec2.get(8);
    // This returns Option, which is enum of type Some and None
    println!("third_value is {}", third_value.unwrap_or(&'z'));
    // third_value is z
    // Since there's no 8th element it makes z as the case if no value is found
    // We can also use match
    let third_value = match vec2.get(30) {
        Some(value) => value,
        None => {
            println!("The index is out of bound");
            &'z'
        }
    };
    println!("third_value is {}", third_value);
    // third_value is z

    // When teh program has a valid reference, the borrow checker enforces the ownership and
    // borrowing rules to ensure this reference adn any other references to the contents of the
    // vector remain valid.
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    //   v.push(1);
    //   │   ├╴  cannot borrow `v` as mutable because it is also borrowed as immutable
    //   │   │    mutable borrow occurs here rustc (E0502) [51, 5]
    //

    println!("First element is {first}");

    // Iterating over vector
    for i in &mut v {
        // We are dereferenciung the reference operator to get the actual value
        *i *= 2;
        println!("i is {i}");
    }

    // To have mixed types in vector:
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let cells: Vec<SpreadSheetCell> = vec![
        SpreadSheetCell::Int(20),
        SpreadSheetCell::Float(2.3),
        SpreadSheetCell::Text(String::from("Hello")),
        SpreadSheetCell::Text(String::from("World")),
    ];
    println!("My vec is {:?}", cells);
    // My vec is [Int(20), Float(2.3), Text("Hello")]

    match cells.get(6) {
        Some(SpreadSheetCell::Int(value)) => println!("The value is int {value}"),
        Some(_) => println!("This is some other value"),
        None => println!("None"),
    }
}
