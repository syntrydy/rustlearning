use std::{sync::mpsc, thread, time};

/// This is the main function of my program
fn main() {
    let (tx, rx) = mpsc::channel();

    println!("Main thread");
    println!("{:?}", thread::current().id());
    let ten_millis = time::Duration::from_secs(10);
    let handle = thread::spawn(move || {
        println!("Running in a new thread");
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(ten_millis);
        }
        println!("{:?}", thread::current().id());
        drop(tx);
        75
    });
    for val in rx {
        println!("Recieving value {}", val);
    }
    println!("DOING SOME  OTHER WORK");
    let result = handle.join().unwrap();
    do_work(5);
    println!("Done {result}");
}

/// this function actually do some work
/// # Arguments
///
/// a is unsigned integer of 32 bits
/// # Returns
///
/// a multiply by a
fn do_work(a: u32) {
    println!("Welcome {}", a * a);
}
