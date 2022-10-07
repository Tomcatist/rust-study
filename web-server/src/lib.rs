use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

//struct Job;

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {

    /// 创建一个新的线程池
    ///
    /// 数字size是创建线程的数量
    ///
    /// 如果size为0，则会产生panic！
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        // 创建一个size大小的Vec
        let mut workers = Vec::with_capacity(size);
        // 根据size大小创建线程
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool{ workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }

}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl <F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

impl Worker {
    fn new (id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            //let job = receiver.lock().unwrap().recv().unwrap();
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {} got a job; executing!", id);
                job.call_box();
                //(*job)();
            }
        });
        Worker {id, thread}
    }
}