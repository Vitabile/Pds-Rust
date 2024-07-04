pub mod cb {
    use std::sync::mpsc::{channel, Receiver, Sender};

    pub struct CyclicBarrier {
        waiters: Vec<Waiter>,
    } 

    pub struct Waiter {
        receiver: Receiver<()>,
        senders: Vec<Sender<()>>,
    }

    impl Waiter {
        pub fn wait(&self) {
            for tx in &self.senders {
                tx.send(());
            }
            for _ in 0..self.senders.len(){
                self.receiver.recv();
            }
        }
    }

    impl CyclicBarrier {
        pub fn new(n_waiters: usize) -> Self {
            let mut senders = vec![];
            let mut recievers = vec![];

            // create the n channels
            for _ in 0..n_waiters {
                let (tx,rx) = channel();
                senders.push(tx);
                recievers.push(rx);
            }
            let mut waiters = vec![];

            recievers = recievers.into_iter().rev().collect();

            for i in 0..n_waiters {
                waiters.push(Waiter{receiver: recievers.pop().unwrap(),
                                    senders: senders.iter().enumerate().filter_map(|(j,s)| {if i==j {None} else {Some(s.clone())}}).collect()})
            }
            Self {waiters}
        }


        pub fn get_waiter(&mut self) -> Waiter {
            if let Some(waiter) = self.waiters.pop() {
                waiter
            }else{ panic!("No more waiters in the barrier!") }
        }
    }
}