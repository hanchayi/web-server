use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{ Receiver, Sender};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

//  an owned pointer to a callable value
//  (with the original type unknown and dynamically change) 
// such as closures (with no argument or no return value), 
// which can be sent across threads and lives as long as the program itself.
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new (id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // 循环的执行recv
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; excuting");
                    job();
                },
                Err(_) => {
                    println!("Worker {id} shut down");
                    break;
                },
            }
        });
        Worker {
            id,
            thread: Some(thread),
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

        ThreadPool { workers, sender: Some(sender) }
    }

    /**
     * 执行
     */
    pub fn execute<F>(&self, f: F) 
        where 
            F: FnOnce() + Send + 'static,
    {
        // 分配一个job
        let job = Box::new(f);
        // 发送job
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down workder {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}