use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..=10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..=5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // As soon as the main thread is finished, the spawned thread is killed no matter the execution
    // was finished or not
    // To fix that:
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("hi number {} from the second spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
    // So first the complete execution happens then the second spawned thread is terminated

    // To use the values inside the closure function of threads use move as to let compiler know
    // that you are moving the ownership from v to inside closure functio of spawned thread
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
