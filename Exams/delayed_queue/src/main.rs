/*
    Una DelayedQueue<T:Send> è un particolare tipo di coda non limitata che offre tre metodi
    principali, oltre alla funzione costruttrice:
        1. offer(&self, t:T, i: Instant) : Inserisce un elemento che non potrà essere estratto prima
           dell'istante di scadenza i.
        2. take(&self) -> Option<T>: Cerca l'elemento t con scadenza più ravvicinata: se tale
           scadenza è già stata oltrepassata, restituisce Some(t); se la scadenza non è ancora stata
           superata, attende senza consumare cicli di CPU, che tale tempo trascorra, per poi restituire
           Some(t); se non è presente nessun elemento in coda, restituisce None. Se, durante l'attesa,
           avviene un cambiamento qualsiasi al contenuto della coda, ripete il procedimento suddetto
           con il nuovo elemento a scadenza più ravvicinata (ammesso che ci sia ancora).
        3. size(&self) -> usize: restituisce il numero di elementi in coda indipendentemente dal fatto
           che siano scaduti o meno.
    Si implementi tale struttura dati nel linguaggio Rust, avendo cura di renderne il comportamento
    thread-safe. Si ricordi che gli oggetti di tipo Condvar offrono un meccanismo di attesa limitata nel
    tempo, offerto dai metodi wait_timeout(...) e wait_timeout_while(...)).
*/

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::sync::{Condvar, Mutex};
use std::time::Instant;

pub struct DelayedQueue<T: Send + Ord> {
    buff: Mutex<BinaryHeap<(Reverse<Instant>, T)>>,
    cv: Condvar,
}

impl<T: Send + Ord> DelayedQueue<T> {
    pub fn new() -> Self {
        Self {
            buff: Mutex::new(BinaryHeap::new()),
            cv: Condvar::new(),
        }
    }

    pub fn offer(&self, t: T, i: Instant) {
        let mut lock = self.buff.lock().unwrap();
        lock.push((Reverse(i), t));
        self.cv.notify_all();
    }
    pub fn take(&self) -> Option<T> {
        let mut lock = self.buff.lock().unwrap();
        while !lock.is_empty() {
            let (i, t) = lock.iter().last().unwrap();
            let duration = (*i).0 - Instant::now();
            if (*i).0 < Instant::now() {
                self.cv.notify_all();
                return Some(lock.pop().unwrap().1);
            } else {
                lock = self.cv.wait_timeout(lock, duration).unwrap().0;
            }
        }
        None
    }

    pub fn size(&self) -> usize {
        let lock = self.buff.lock().unwrap();
        lock.len()
    }
}
