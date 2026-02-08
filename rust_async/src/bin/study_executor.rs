use futures::FutureExt;
use futures::future::BoxFuture;
use futures::task::{ArcWake, waker_ref};
use rust_async::studt_future::TimeFuture;
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::sync::{Arc, Mutex};
use std::task::Context;
use std::thread;
use std::time::Duration;

fn main() {
    let (executor, spawner) = new_executor_and_spawner();
    let future = async {
        println!("[{:?}] howdy!", thread::current().id());
        TimeFuture::new(Duration::new(2, 0)).await;
        println!("[{:?}] done!", thread::current().id());
    };
    spawner.spawn(future);
    println!("[{:?}] Drop Spawner ...", thread::current().id());
    drop(spawner);
    executor.run();
}
struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

struct Executor {
    ready_queued: Receiver<Arc<Task>>,
}
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        println!("[{:?}] wake_by_ref", thread::current().id());
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many  task queued");
    }
}
impl Spawner {
    fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        println!(
            "[{:?}] 将 Future 组成 Task,放入 Channel...",
            thread::current().id()
        );
        self.task_sender.send(task).expect("too many tasks queued");
    }
}
impl Executor {
    fn run(&self) {
        println!("[{:?}] Executor running...", thread::current().id());
        while let Ok(task) = self.ready_queued.recv() {
            println!("[{:?}] 接受到任务...", thread::current().id());
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                println!("[{:?}] 从任务中取得 Future ...", thread::current().id());
                let waker = waker_ref(&task);
                println!("[{:?}] 获得 waker by ref ...", thread::current().id());
                let context = &mut Context::from_waker(&*waker);
                println!(
                    "[{:?}] 获得 Context 准备进行 poll ...",
                    thread::current().id()
                );
                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);
                    println!("[{:?}] Poll::Pending ...", thread::current().id());
                } else {
                    println!("[{:?}] Poll::Ready ...", thread::current().id());
                }
            }
        }
        println!("[{:?}] Executor run 结束 ...", thread::current().id());
    }
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queued) = sync_channel(MAX_QUEUED_TASKS);
    println!(
        "[{:?}] 生成 Executor 和 Spawner (含发送端,接收端)...",
        thread::current().id()
    );
    (Executor { ready_queued }, Spawner { task_sender })
}
