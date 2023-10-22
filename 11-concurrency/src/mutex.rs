use std::{
    sync::{Arc, Mutex, MutexGuard},
    thread::{self, JoinHandle},
};

pub fn basic() {
    let m: Mutex<i32> = Mutex::new(5);

    {
        let mut num: MutexGuard<'_, i32> = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

pub fn with_multiple_threads() {
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles: Vec<_> = vec![];

    for _ in 0..10 {
        let counter: Arc<Mutex<i32>> = Arc::clone(&counter);
        let handle: JoinHandle<()> = thread::spawn(move || {
            let mut num: MutexGuard<'_, i32> = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
