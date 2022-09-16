use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        sync::mpsc::{sync_channel, Receiver, SyncSender},
        sync::{Arc, Mutex},
        task::Context,
        time::Duration,
    },
    timer_future::TimerFuture,
};
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}
impl Executor {
    fn run(&self) {
        println!("run");
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let cx = &mut Context::from_waker(&*waker);
                println!("run 1");
                if future.as_mut().poll(cx).is_pending() {
                    println!("run2");
                    *future_slot = Some(future);
                }
                println!("run 3");
            }
        }
    }
}
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}
impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        println!("spawn ");
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("通道拥堵");
    }
}
struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        println!("wake_by_ref ");
        let newtask = arc_self.clone();
        arc_self.task_sender.send(newtask).expect("msg");
    }
}
fn new_executor_and_spawner() -> (Spawner, Executor) {
    const MAX_QUEUED_TASKS: usize = 100;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Spawner { task_sender }, Executor { ready_queue })
}
fn main() {
    let (sp, ex) = new_executor_and_spawner();
    sp.spawn(async {
        println!("howdy");
        TimerFuture::new(Duration::from_secs(2)).await;
        println!("over");
    });
    drop(sp);
    println!("drop spawn");
    ex.run();
}
