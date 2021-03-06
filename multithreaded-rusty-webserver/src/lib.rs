use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>, 
    sender: mpsc::Sender<Job>,

}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

pub struct Worker{
    id: usize , 
    thread: thread::JoinHandle<()>
}

impl Worker{
    fn new(id: usize , receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self{

        let thread = thread::spawn(move ||{
            loop {
                let job = receiver.lock().unwrap().recv().unwrap(); 

                println!("Worker {} got a job; executing.", id);
               
                job.call_box(); 

            }
        }); 

        Worker{
            id,
            thread
        }
    }
}
impl ThreadPool{
    pub fn new(size: usize) -> Self{
        assert!(size > 0); 
        
        let mut workers = Vec::with_capacity(size); 
        let (sender , receiver) = mpsc::channel(); 

        let receiver = Arc::new(Mutex::new(receiver)); 

        for i in 0..size{
            // Create size threads and push them to the vector 
            let worker = Worker::new(i , Arc::clone(&receiver)); 
            workers.push(worker); 
        }
        ThreadPool{
            workers, 
            sender, 
        }
    }

    pub fn execute<F>(&self , f: F)
        where F: FnOnce() + Send + 'static{
            let job = Box::new(f);
            
            self.sender.send(job).unwrap(); 
        }
}