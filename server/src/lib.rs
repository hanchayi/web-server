use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{ Receiver, Sender};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
}

pub struct Job;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new (id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });
        Worker {
            id,
            thread,
        }
    }
}

impl ThreadPool {
    pub fn new (size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        // multiple producer, single consumer
        let ( sender, receiver ) = mpsc::channel();

        // to share ownership across multiple threads and allow the threads to mutate the value, we need to use Arc<Mutex<T>>. 
        // The Arc type will let multiple workers own the receiver
        // Mutex will ensure that only one worker gets a job from the receiver at a time.
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F) 
        where 
            F: FnOnce() + Send + 'static,
    {

    }
}