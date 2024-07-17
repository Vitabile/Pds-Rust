/*
La classe generica Exchanger<T> permette a due thread di scambiarsi un valore di tipo T. Essa offre esclusivamente il metodo pubblico T exchange( T t) che blocca il
thread chiamante senza consumare CPU fino a che un altro thread non invoca lo stesso metodo, sulla stessa istanza. Quando questo avviene, il metodo restituisce
l’oggetto passato come parametro dal thread opposto.
*/

use rand::Rng;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

const N_THREADS: usize = 10;

pub struct Exchanger<T: Send> {
    values: Mutex<(Option<T>, Option<T>)>,
    cv: Condvar,
}

impl<T: Send> Exchanger<T> {
    pub fn new() -> Self {
        Self {
            values: Mutex::new((None, None)),
            cv: Condvar::new(),
        }
    }

    pub fn exchange(&self, t: T) -> T {
        let mut guard = self.values.lock().unwrap();

        if guard.0.is_none() {
            // sono il primo
            guard.0 = Some(t);
            guard = self
                .cv
                .wait_while(guard, |values| values.1.is_none())
                .unwrap();
            guard.1.take().unwrap() // prendo l'elemento e imposto None
        } else {
            guard.1 = Some(t);
            let res = guard.0.take().unwrap(); // prendo l'elemento e imposto a None
            self.cv.notify_all();
            res
        }
    }
}

fn main() {
    println!("\nwarning: some prints might be out of order\n");
    let exchanger = Arc::new(Exchanger::new());

    //let mut vec_join = Vec::new();
    thread::scope(|s| {
        for i in 0..N_THREADS {
            s.spawn({
                let e = exchanger.clone();
                move || {
                    let time = rand::thread_rng().gen_range(0..20);
                    sleep(Duration::from_secs(time));
                    println!("thread {} began exchanging procedure", i);
                    let v = e.exchange(i);
                    println!("> thread {} got value {}", i, v);
                }
            });
        }
    });
}
