/*

A counting semaphore. Conceptually, a semaphore maintains a set of permits. Each acquire() blocks
if necessary until a permit is available, and then takes it. Each release() adds a permit, potentially
releasing a blocking acquirer. However, no actual permit objects are used; the Semaphore just keeps
a count of the number available and acts accordingly.

*/

use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

pub struct Semaphore {
    count: Mutex<u32>,
    cv: Condvar,
}

impl Semaphore {
    pub fn new(count: u32) -> Self {
        Self {
            count: Mutex::new(count),
            cv: Condvar::new(),
        }
    }

    pub fn acquire(&self) {
        let mut count = self.count.lock().unwrap();
        count = self.cv.wait_while(count, |c| *c == 0).unwrap(); // attendo se il count è 0
        *count -= 1;
    }

    pub fn release(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        self.cv.notify_one();
    }
}

fn main() {
    let sem = Arc::new(Semaphore::new(3));
    let mut hs = vec![];
    for i in 0..20 {
        let sem = sem.clone();
        hs.push(thread::spawn(move || {
            sem.acquire();
            println!("#{i} thread acquired!");
            thread::sleep(Duration::from_secs(2));
            println!("#{i} thread released!");
            sem.release();
        }));
    }

    for h in hs {
        h.join().unwrap();
    }
}
