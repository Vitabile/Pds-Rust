/*
Un paradigma frequentemente usato nei sistemi reattivi e costituito dall'astrazione detta Looper.
Quando viene creato, un Looper crea una coda di oggetti generici di tipo Message ed un thread.
II thread attende - senza consumare cicli di CPU - che siano presenti messaggi nella coda,
li estrae a uno a uno nell'ordine di arrivo, e li elabora.

II costruttore di Looper riceve due parametri, entrambi di tipo (puntatore a) funzione: process( ... ) e cleanup().
La prima è una funzione responsabile di elaborare i singoli messaggi ricevuti attraverso la coda;
tale funzione accetta un unico parametro in ingresso di tipo Message e non ritorna nulla;
La seconda è una funzione priva di argomenti e valore di ritorno e verra invocata dal thread incapsulato
nel Looper quando esso stara per terminare.

Looper offre un unico metodo pubblico, thread safe, oltre a quelli di servizio, necessari per gestirne ii ciclo di vita:
send(msg), che accetta come parametro un oggetto generico di tipo Message che verra inserito nella coda
e successivamente estratto dal thread ed inoltrato alla funzione di elaborazione.
Quando un oggetto Looper viene distrutto, occorre fare in modo che ii thread contenuto al suo interno
invochi la seconda funzione passata nel costruttore e poi termini.

Si implementi, utilizzando il linguaggio Rust o C++, tale astrazione tenendo conto che i suoi
 metodi dovranno essere thread-safe.
*/

use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

pub struct Looper<Msg: Send + 'static> {
    tx: Mutex<Option<Sender<Msg>>>,
    h: Option<JoinHandle<()>>,
}

impl<Msg: Send + 'static> Drop for Looper<Msg> {
    fn drop(&mut self) {
        let mut lock = self.tx.lock().unwrap();
        drop(lock.take().unwrap()); // drop del sender per eseguire cleanup
        self.h.take().unwrap().join().unwrap(); // attendo la fine di cleanup (ho bisogno di conusumare l'handle per questo serve l'option)
    }
}
impl<Msg: Send + 'static> Looper<Msg> {
    pub fn new(process: fn(Msg), cleanup: fn()) -> Self {
        let (tx, rx) = channel::<Msg>();

        // thread
        let h = thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                process(msg);
            }
            cleanup();
        });
        Self {
            tx: Mutex::new(Some(tx)),
            h: Some(h),
        }
    }

    pub fn send(&self, msg: Msg) {
        let lock = self.tx.lock().unwrap();
        lock.as_ref().unwrap().send(msg);
    }
}
