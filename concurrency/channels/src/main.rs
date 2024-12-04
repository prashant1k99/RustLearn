use std::sync::mpsc;
// mpsc stands for multiple producers and single consumer
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap();

        // We cannot use the variable msg after we have sent it to the channel response
        // println!("msg is {msg}");
        // ├╴  borrow of moved value: `msg`
    });

    // recv method blocks the main thread execution while awaits response from the channel
    // try_recv method will not block the main thread instead it will return the result type
    // immediately
    let received = rx.recv().unwrap();
    println!("Got: {received}");
    // Got: hi

    // Wiht multiple values:
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
        // Got: Hi
        // Got: from
        // Got: the
        // Got: thread
    }

    // Creating multiple threads
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
            String::from("four"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
        // Got: 1
        // Got: one
        // Got: 2
        // Got: two
        // Got: 3
        // Got: three
        // Got: four
        // Got: 4
    }
}
