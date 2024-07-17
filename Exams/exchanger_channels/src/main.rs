/*
La classe generica Exchanger<T> permette a due thread di scambiarsi un valore di tipo T. Essa offre esclusivamente il metodo pubblico T exchange( T t) che blocca il
thread chiamante senza consumare CPU fino a che un altro thread non invoca lo stesso metodo, sulla stessa istanza. Quando questo avviene, il metodo restituisce
l’oggetto passato come parametro dal thread opposto.
*/

use rand::Rng;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

const N_THREADS: usize = 10;

pub struct Exchanger<T: Send> {
    first: Mutex<bool>,
    ch1: Mutex<(Sender<T>, Receiver<T>)>, // mutex usato per la condivisione tra thread
    ch2: Mutex<(Sender<T>, Receiver<T>)>, // mutex usato per la condivisione tra thread
}

impl<T: Send> Exchanger<T> {
    pub fn new() -> Self {
        let (tx1, rx1) = channel::<T>();
        let (tx2, rx2) = channel::<T>();
        Self {
            first: Mutex::new(true),
            ch1: Mutex::new((tx1, rx2)),
            ch2: Mutex::new((tx2, rx1)),
        }
    }

    pub fn exchange(&self, t: T) -> T {
        let mut first = self.first.lock().unwrap();
        if *first {
            // il primo thread per lo scambio
            let mut guard = self.ch1.lock().unwrap();
            guard.0.send(t);
            *first = false;
            drop(first);
            guard.1.recv().unwrap()
        } else {
            let mut guard = self.ch2.lock().unwrap(); // sono il secondo
            guard.0.send(t);
            *first = true;
            drop(first);
            guard.1.recv().unwrap()
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
