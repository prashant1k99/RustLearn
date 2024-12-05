fn main() {
    // Enum match patterns
    enum Language {
        English,
        Spanish,
        Russian,
        Japanese,
        Mandarin,
    }

    let language = Language::English;

    match language {
        Language::English => println!("Hello World"),
        Language::Spanish => println!("Hola Mundo!"),
        Language::Russian => println!("Привет, мир"),
        Language::Japanese => println!("こんにちは世界"),
        _ => println!("Unsupported Language"),
    }
    // Hello World

    // Conditional If let expressions:
    // The down side of if let syntax is that compiler doesn't makes the arms exhasutive
    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    // Here we are checking if the authorizatio status is of Some variant then we are storing it in
    // the status variable, since it's of none variant we go in else if block
    if let Some(status) = authorization_status {
        println!("Authorization status: {status}");
    } else if is_admin {
        // Here we check directly for boolean
        println!("Authorization status: admin");
    } else if let Ok(group_id) = group_id {
        // Here we are checking if the value returned from parsing in group_id is Ok then we go
        // inside this block
        if group_id > 30 {
            println!("Auhtorization status: privilaged");
        } else {
            println!("Authorization status: basic");
        }
    } else {
        println!("Authorizatrion status: guest");
    }
    // Auhtorization status: privilaged

    // While let Conditional Loop
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    // 3
    // 2
    // 1

    // For loops
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }
    // a is at index 0
    // b is at index 1
    // c is at index 2

    // let Statements
    let x = 5;
    // let PATTERN = EXPRESSION;
    let (x, y, z) = (1, 2, 3);

    // Function Parameter
    let point = (3, 5);
    print_coordinates(&point);

    //Irrefutable
    // The patterns that always match
    let _x = 5;

    //Refutable
    //Might not match
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{x}");
    }
    // function parameters, if let statements, for loops only accept irrefutable parameters

    //Some examples:
    let x: Option<&str> = None;
    // let Some(x) = x;
    // error[E0005]: refutable pattern in local binding

    // This gives us the warning that it accepts the irrefutable pattern, but got refutable pattern
    if let x = 5 {
        println!("{x}");
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current Location: ({x}, {y})");
}
