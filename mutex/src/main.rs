use std::{sync::{Mutex, Arc}, thread::spawn, rc::Rc};

fn main() {
    println!("Hello, world!");
    let counter = Arc::new(Mutex::new(0));
    let mut threads = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let thread = spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Result {}", *counter.lock().unwrap());
}
