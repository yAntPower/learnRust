use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}
struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}
impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("poll");
        let mut shared_state = self
            .shared_state
            .lock()
            .expect("get shared_state lock err!");
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        println!("TimerFuture new");
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));
        let shared = shared_state.clone();
        thread::spawn(move || {
            println!("thread start");
            thread::sleep(duration);
            let mut sharedstate = shared
                .lock()
                .expect("The thread calls the lock variable when TimerFuture is constructed");
            sharedstate.completed = true;
            if let Some(wk) = sharedstate.waker.take() {
                wk.wake();
            }
            println!("thread end");
        });
        println!("TimerFuture new end");
        TimerFuture { shared_state }
    }
}
