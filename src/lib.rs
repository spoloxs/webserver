use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};

pub struct ThreadPool{
    wokers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

// Fn to handle threads
impl ThreadPool{
    pub fn new(size: usize) -> ThreadPool{
        assert!(size >0);

        let(sender, receiver): (Sender<Job>, Receiver<Job>) = mpsc::channel();
        let receiver: Arc<Mutex<Receiver<Job>>> = 
            Arc::new(Mutex::new(receiver)); // For sharing the same thread

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id,
                Arc::clone(&receiver) ))
        }

        ThreadPool { wokers: workers, sender }
    }

    pub fn execute<F> (&self, f: F)
    where 
        F: FnOnce() + Send + 'static{
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
}

struct Worker { // for handling thread so that it can rework
    id: usize,
    threads: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>> ) -> Worker{
        let thread = thread::spawn(move || loop{
            let job = receiver
                    .lock()
                    .unwrap()
                    .recv()
                    .unwrap();

                println!("Worker {} got a job", id);
        });

        Worker { id: id, threads: thread }
    }
    
}