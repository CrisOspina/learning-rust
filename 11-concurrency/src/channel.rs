use std::{sync::mpsc, thread, time::Duration};

/**
 * Channels and Ownership Transference
 * mpsc: multiple producer, single consumer
 */

pub fn basic() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi from thread inside channel");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received)
}

pub fn multiple() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
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
        println!("Got: {}", received);
    }
}

pub fn multiple_prod_cloned_transmitter() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals: Vec<String> = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread 1"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals: Vec<String> = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
            String::from("thread 2"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
