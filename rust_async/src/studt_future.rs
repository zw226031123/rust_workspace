use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;
#[allow(dead_code)]
fn main() {}

pub struct TimeFuture {
    shared_state: Arc<Mutex<ShareState>>,
}

struct ShareState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimeFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("[{:?}] Polling TimeFuture...", thread::current().id());
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            println!("[{:?}] TimeFuture completed...", thread::current().id());
            Poll::Ready(())
        } else {
            println!("[{:?}] TimeFuture pending...", thread::current().id());
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
impl TimeFuture {
    pub fn new(duration: Duration) -> Self {
        println!("[{:?}] 开始创建新的 TimeFuture...", thread::current().id());
        let shared_state = Arc::new(Mutex::new(ShareState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            println!(
                "[{:?}] TimeFuture 生成新的线程并开始睡眠...",
                thread::current().id()
            );
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;

            if let Some(waker) = shared_state.waker.take() {
                println!(
                    "[{:?}] TimeFuture 新线程获得 waker,并进行wake...",
                    thread::current().id()
                );
                waker.wake()
            } else {
                println!(
                    "[{:?}] TimeFuture 新线程没获得 waker...",
                    thread::current().id()
                );
            }
        });
        println!("[{:?}] 返回新的 TimeFuture...", thread::current().id());
        TimeFuture { shared_state }
    }
}
