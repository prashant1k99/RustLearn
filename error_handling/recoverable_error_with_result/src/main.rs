// Result is an enum which has 2 types: Ok(T) and Err(E)

use core::panic;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let r = divide(4, 2);
    println!("R = {r}");
    // R = 2

    // divide(4, 0);
    // this will crash the application:
    // thread 'main' panicked at src/main.rs:14:5:
    // attempt to divide by zero

    let r = match divide_with_result(4, 0) {
        Ok(value) => value,
        Err(err) => {
            println!("{err}");
            -1
        }
    };
    // Please do not divide with 0
    println!("Result is {r}");
    // Result is -1

    // Let's try with a simple example of file openingg:
    let file = File::open("hello.txt");
    let _file = match file {
        Ok(file) => {
            println!("able to open the non exisitng file");
            file
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error}")
            }
        },
    };
    // do file operation as we are sure that file exists at this point
    println!("the file exists");

    // Error propogation
    // Notice that we are repeatingg oursleves a lot
    match read_username_from_file() {
        Ok(_) => {
            println!("Ggot usernames")
        }
        Err(err) => {
            println!("failed to get username: {err:?}")
        }
    }

    // This cane be solved using ? operator:
    // So we just propogate the error to the code block from where it is beingg called from
    match read_username_from_file_with_propogation() {
        Ok(_) => {
            println!("Ggot usernames")
        }
        Err(err) => {
            println!("failed to get username: {err:?}")
        }
    }
    // ? operator can only be used in functions whose return type is compatible with the value of ?
}

fn read_username_from_file_with_propogation() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn divide(x: i32, y: i32) -> i32 {
    x / y
}

fn divide_with_result(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        return Err(String::from("Please do not divide with 0"));
    }
    Ok(x / y)
}
