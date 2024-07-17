/*

Binary semaphores are synchronization mechanisms that have integer values that range from 0 (zero) to 1 (one). As a result, this type of semaphore gives a single point of access to a key portion. It signifies that only one individual will have simultaneous access to the critical part

*/

use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

pub struct BinarySemaphore {
    flag: Mutex<bool>,
    cv: Condvar,
}

impl BinarySemaphore {
    pub fn new() -> Self {
        Self {
            flag: Mutex::new(false),
            cv: Condvar::new(),
        }
    }

    pub fn acquire(&self) {
        let mut flag = self.flag.lock().unwrap();
        flag = self.cv.wait_while(flag, |f| *f).unwrap(); // attendo se è occupato
        *flag = true;
    }

    pub fn release(&self) {
        let mut flag = self.flag.lock().unwrap();
        *flag = false;
        self.cv.notify_one();
    }
}

fn main() {
    let sem = Arc::new(BinarySemaphore::new());
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
