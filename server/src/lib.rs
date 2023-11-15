use std::{thread, sync::{mpsc::{self}, Arc, Mutex}};

struct Worker {
    id: usize,
    thread: Option< thread::JoinHandle<()> >
}

impl Worker {
    fn new( id: usize , receiver: Arc< Mutex< mpsc::Receiver<Message> > >) -> Worker {
        let thread = thread::spawn( move || loop { 
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Hey i got a Job id : {}", id);
                    job();
                }   
                Message::Terminate => {
                    println!("Hey i got a terminated : {}", id);
                    break;
                }             
            }
        });
        Worker { id, thread:Some(thread) }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl Drop for ThreadPool {
    fn drop( &mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for w in  &mut self.workers {
            println!("Server thread Shuttdown: {}", w.id );
            if let Some(thread) = w.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool {
    pub fn new( size:usize )-> ThreadPool {
        assert!( size > 0); // this function is panic

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new( Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            // populate the treads here
            workers.push( Worker::new(i, Arc::clone(&receiver)) );
        }

        ThreadPool { workers , sender }
    }
    pub fn execute<F> (&self, f:F ) where F:FnOnce() + 'static + Send {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

