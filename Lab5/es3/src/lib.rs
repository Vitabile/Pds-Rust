pub mod cb {
    use std::{fmt::Debug, sync::{Condvar, Mutex}};
    #[derive(Debug, Clone)]
    struct Barrier<T: Debug + Clone> {
        items: Vec<T>,
        open: bool,
        n_threads: usize,
        limit: usize
    }

    impl<T: Debug + Clone> Barrier<T>{
        fn new(limit: usize) -> Self {
            Self {items: vec![], open: false, n_threads: 0, limit}
        }
    }

    pub struct CyclicBarrier<T: Debug + Clone>{
        cb: (Mutex<Barrier<T>>,Condvar),

    }

    impl<T: Debug + Clone> CyclicBarrier<T> {
        pub fn new(limit: usize) -> Self {
            CyclicBarrier {
                cb: (Mutex::new(Barrier::new(limit)),Condvar::new())
            }
        }

        pub fn wait(&self, item: T) -> Vec<T>{
            let (lock, cv) = &self.cb;

            let mut barrier = lock.lock().unwrap();
            barrier.items.push(item);

            // wait while the barrier is close
            while !(*barrier).open {
                // increment threads in the barrier
                (*barrier).n_threads += 1;
                if (*barrier).n_threads == (*barrier).limit {
                    (*barrier).open = true; // now the door is open (the threads can exit and new cannot entry)
                    cv.notify_all();
                }else{
                    // go to sleep except in the case of the last that open the door and exit
                    barrier = cv.wait(barrier).unwrap();
                }
                
                (*barrier).n_threads -= 1;
            }
            let res = barrier.items.clone();
            // if all the threads are out close the door (the threads can now entry again)
            if (*barrier).n_threads == 0 {
                (*barrier).open = false;
                barrier.items = vec![];
                cv.notify_all();
            }
            res
        }

    }

}