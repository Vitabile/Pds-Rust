use es2::thread_pool::ThreadPool;
use std::{time::Duration,thread};

fn main() {
    // alloca i worker
    let threadpool = ThreadPool::new(10);

    for x in 0..100 {
        threadpool.execute(Box::new(move || {
            println!("long running task {}", x);
            thread::sleep(Duration::from_millis(1000))
        }))
    }
    // just to keep the main thread alive
    loop {thread::sleep(Duration::from_millis(1000))};
}