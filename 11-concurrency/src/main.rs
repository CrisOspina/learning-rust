mod threads;
mod channel;
mod mutex;

use std::thread;
use std::time::Duration;

fn main() {
    // threads::creating_thread();
    // threads::waiting_all_threads();
    // channel::basic();
    // channel::multiple();
    // channel::multiple_prod_cloned_transmitter();
    // mutex::basic();
    mutex::with_multiple_threads();

    init_thread_main()
}

fn init_thread_main() {
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
