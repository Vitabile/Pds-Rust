/*
Una barriera è un costrutto di sincronizzazione usato per regolare l'avanzamento relativo della computazione di più thread.
All'atto della costruzione di questo oggetto, viene indicato il numero N di thread coinvolti.

Non è lecito creare una barriera che coinvolga meno di 2 thread.

La barriera offre un solo metodo, wait(), il cui scopo è bloccare temporaneamente l'esecuzione del thread che lo ha invocato, non ritornando fino a che non sono giunte
altre N-1 invocazioni dello stesso metodo da parte di altri thread: quando ciò succede, la barriera si sblocca e tutti tornano. Successive invocazioni del metodo wait()
hanno lo stesso comportamento: la barriera è ciclica.

Attenzione a non mescolare le fasi di ingresso e di uscita!

Una RankingBarrier è una versione particolare della barriera in cui il metodo wait() restituisce un intero che rappresenta l'ordine di arrivo: il primo thread ad avere
invocato wait() otterrà 1 come valore di ritorno, il secondo thread 2, e così via. All'inizio di un nuovo ciclo, il conteggio ripartirà da 1.

Si implementi la struttura dati RankingBarrier a scelta nei linguaggi Rust o C++ '11 o successivi.
*/

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub struct RankingBarrier {
    shared_data: Mutex<(u32, bool)>, // counter, flag per indicare la fase di uscita
    size: u32,
    cv: Condvar,
}

impl RankingBarrier {
    pub fn new(size: u32) -> Option<Self> {
        if size < 2 {
            None
        } else {
            Some(Self {
                shared_data: Mutex::new((0, false)),
                size,
                cv: Condvar::new(),
            })
        }
    }

    pub fn wait(&self) -> u32 {
        let mut lock = self.shared_data.lock().unwrap();
        lock = self.cv.wait_while(lock, |l| l.1).unwrap(); // aspetto finchè siamo in fase di uscita

        let res = lock.0 + 1;
        lock.0 = res; // incremento il numero di thread in wait
        lock = self
            .cv
            .wait_while(lock, |l| l.0 != self.size && !l.1)
            .unwrap(); // sleep finchè non arriva l'N-esimo thread e siamo in ingresso
        if !lock.1 {
            lock.1 = true
        }; // se sono il primo aggiorno la flag e attivo la fase di uscita
        lock.0 -= 1;
        if lock.0 == 0 {
            lock.1 = false;
        } // se sono l'ultimo riattivo la fase di ingresso
        self.cv.notify_all();
        res
    }
}

fn main() {
    assert!(RankingBarrier::new(1).is_none());
    let barrier = Arc::new(RankingBarrier::new(10).unwrap());
    let mut handles = vec![];
    for id in 1..=10 {
        let barrier = barrier.clone();
        let h = thread::spawn(move || {
            for round in 1..=3 {
                let order = barrier.wait();
                println!("Thread {id} arrived {order} in round {round}")
            }
        });
        handles.push(h);
    }
    for h in handles {
        h.join().unwrap();
    }
}
