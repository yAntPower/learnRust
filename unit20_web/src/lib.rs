use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;
enum Message {
    NewJob(Job),
    Terminate,
}
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    ///创建线程池
    /// 线程池中线程的数量
    /// # Panics
    /// ‘new’ 函数在num为0时会panic
    pub fn new(num: u32) -> ThreadPool {
        let size = num as usize;
        assert!(size > 0);
        let (sender, recv) = mpsc::channel();
        let recv = Arc::new(Mutex::new(recv));
        let mut workers = Vec::with_capacity(size);
        for i in 0..num {
            workers.push(Worker::new(i as usize, Arc::clone(&recv)));
        }
        ThreadPool { workers, sender }
    }
    pub fn excute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
    fn dorp(&mut self) {
        for work in &mut self.workers {
            if let Some(thread) = work.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, recver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            println!("{}号线程开始调用闭包", id);
            let msg = recver
                .lock()
                .expect("调用闭包失败")
                .recv()
                .expect("调用闭包时获取接受数据错误。");
            match msg {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });
        Worker {
            id:id,
            thread: Some(thread),
        }
    }
}
