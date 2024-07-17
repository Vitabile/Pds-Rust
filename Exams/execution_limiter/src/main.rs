/*
All'interno di un programma è necessario garantire che non vengano eseguite CONTEMPORANEAMENTE più di N invocazioni di operazioni potenzialmente lente.
A questo scopo, è stata definita la struttura dati ExecutionLimiter che viene inizializzata con il valore N del limite.
Tale struttura è thread-safe e offre solo il metodo pubblico generico execute( f ), che accetta come unico parametro una funzione f, priva di parametri
che ritorna il tipo generico R. Il metodo execute(...) ha, come tipo di ritorno, lo stesso tipo R restituito da f ed ha il compito di mantere il conteggio
di quante invocazioni sono in corso. Se tale numero è già pari al valore N definito all'atto della costruzione della struttura dati, attende, senza provocare
consumo di CPU, che scenda sotto soglia, dopodiché invoca la funzione f ricevuta come parametro e ne restituisce il valore. Poiché l'esecuzione della funzione f
potrebbe fallire, in tale caso, si preveda di decrementare il conteggio correttamente. Si implementi, usando i linguaggi Rust o C++, tale struttura dati,
garantendo tutte le funzionalità richieste. use std::sync::{Arc, Condvar, Mutex};
*/

use rand::Rng;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

pub struct ExecutionLimiter {
    counter: Mutex<u32>,
    cv: Condvar,
    limit: u32,
}

impl ExecutionLimiter {
    pub fn new(limit: u32) -> Self {
        Self {
            counter: Mutex::new(0),
            cv: Condvar::new(),
            limit,
        }
    }

    pub fn execute<R: Send>(&self, f: fn() -> R) -> Option<R> {
        let mut lock = self.counter.lock().unwrap();
        lock = self
            .cv
            .wait_while(lock, |value| *value == self.limit)
            .unwrap();
        (*lock) += 1;
        drop(lock);
        let res = std::panic::catch_unwind(f);
        lock = self.counter.lock().unwrap();
        (*lock) -= 1;
        self.cv.notify_all();
        res.ok()
    }
}

fn f() -> String {
    let mut rng = rand::thread_rng();
    let secs: u64 = rng.gen_range(1..=5);
    thread::sleep(Duration::from_secs(secs));
    String::from("Thread finished")
}
fn f2() -> String {
    let mut rng = rand::thread_rng();
    let secs: u64 = rng.gen_range(1..=5);
    thread::sleep(Duration::from_secs(secs));
    panic!("Thread panicked");
}
fn main() {
    let mut handles = vec![];
    let mut rng = rand::thread_rng();
    let exc_limiter = Arc::new(ExecutionLimiter::new(3));
    for i in 0..20 {
        let exc_limiter = exc_limiter.clone();
        let f = if rng.gen::<bool>() { f } else { f2 };
        handles.push(thread::spawn(move || {
            let res = exc_limiter.execute(f);
            println!("T{i} {:?}", res);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}
