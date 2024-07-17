/*
In una macchina utensile, sono in esecuzione N thread concorrenti, ciascuno dei quali rileva continuamente una sequenza di valori, risultato dell'elaborazione delle
misurazioni di un sensore. I valori devono essere raggruppati N a N in una struttura dati per essere ulteriormente trattati dal sistema. A questo scopo è definita la
seguente classe thread-safe:

    class Joiner {
        public: Joiner(int N); // N is the number of values that must be conferred
        std::map<int, double> supply(int key, double value);
    };

Il metodo bloccante supply(...) riceve una coppia chiave/valore generata da un singolo thread e si blocca senza consumare CPU fino a che gli altri N-1 thread hanno inviato
le loro misurazioni. Quando sono arrivate N misurazioni (corrispondenti ad altrettante invocazioni concorrenti), si sblocca e ciascuna invocazione precedentemente bloccata
restituisce una mappa che contiene N elementi (uno per ciascun fornitore). Dopodiché, l'oggetto Joiner pulisce il proprio stato e si prepara ad accettare un nuovo gruppo di
N misurazioni, in modo ciclico.

Si implementi tale classe, facendo attenzione a non mescolare nuovi conferimenti con quelli della tornata precedente (un thread appena uscito potrebbe essere molto veloce
a rientrare, ripresentandosi con un nuovo valore quando lo stato non è ancora stato ripulito).
*/

use rand::Rng;
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

const N_THREADS: i32 = 3;

pub struct Joiner {
    n_threads: i32,
    mutex: Mutex<(HashMap<i32, f64>, i32, bool)>, // mappa, contatore e flag per la fase si uscita
    cv: Condvar,
}

impl Joiner {
    pub fn new(n_threads: i32) -> Self {
        Self {
            n_threads,
            mutex: Mutex::new((HashMap::new(), 0, false)),
            cv: Condvar::new(),
        }
    }

    pub fn supply(&self, k: i32, v: f64) -> HashMap<i32, f64> {
        let mut guard = self
            .cv
            .wait_while(self.mutex.lock().unwrap(), |m| m.2)
            .unwrap(); // dormo se siamo in fase di uscita

        guard.1 += 1; // new entry
        guard.0.insert(k, v);
        guard = self
            .cv
            .wait_while(guard, |m| m.1 < self.n_threads && !m.2)
            .unwrap(); // dormo se non sono arrivati tutti e se siamo in ingresso
        guard.1 -= 1;
        let res = guard.0.clone();

        if guard.1 == self.n_threads - 1 {
            // il primo thread a superare la barriera cambia la flag
            guard.2 = true;
            self.cv.notify_all()
        }
        if guard.1 == 0 {
            // l'ultimo thread che supera la barriera resetta la flag e resetta l'hashmap
            guard.2 = false;
            guard.0.clear();
            self.cv.notify_all();
        }
        res
    }
}

fn main() {
    //main is not required in the exam
    let barrier = Arc::new(Joiner::new(N_THREADS));

    let mut vt = Vec::new();

    for i in 0..N_THREADS {
        vt.push(thread::spawn({
            let b = barrier.clone();
            move || {
                for _ in 0..5 {
                    let rng: u64 = rand::thread_rng().gen_range(1..5);
                    sleep(Duration::from_secs(rng));

                    let v: f64 = rand::thread_rng().gen();
                    let map = b.supply(i, v);
                    println!("\nMap returned by Thread #{i}\n{:?}\n", map);
                }
            }
        }));
    }

    for t in vt {
        t.join().unwrap();
    }
}
