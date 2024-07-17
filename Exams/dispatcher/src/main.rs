/*
In un sistema concorrente, ciascun thread può pubblicare eventi per rendere noto ad altri thread quanto sta facendo.
Per evitare un accoppiamento stretto tra mittenti e destinatari degli eventi, si utilizza un Dispatcher: questo è un oggetto thread-safe che offre il metodo

        dispatch(msg: Msg)

mediante il quale un messaggio di tipo generico Msg (soggetto al vincolo di essere clonabile) viene reso disponibile a chiunque si sia sottoscritto.
Un thread interessato a ricevere messaggi può invocare il metodo

        subscribe()

del Dispatcher: otterrà come risultato un oggetto di tipo Subscription mediante il quale potrà leggere i messaggi che da ora in poi saranno pubblicati attraverso
il Dispatcher. Per ogni sottoscrizione attiva, il Dispatcher mantiene internamente l'equivalente di una coda ordinata (FIFO) di messaggi non ancora letti.
A fronte dell'invocazione del metodo dispatch(msg:Msg), il messaggio viene clonato ed inserito in ciascuna delle code esistenti. L'oggetto Subscription offre il
metodo bloccante

        read() -> Option<Msg>

se nella coda corrispondente è presente almeno un messaggio, questo viene rimosso e restituito; se nella coda non è presente nessun messaggio e il Dispatcher esiste
ancora, l'invocazione si blocca fino a che non viene inserito un nuovo messaggio; se invece il Dispatcher è stato distrutto, viene restituito il valore corrispondente
all'opzione vuota.

Gli oggetti Dispatcher e Subscription sono in qualche modo collegati, ma devono poter avere cicli di vita indipendenti: la distruzione del Dispatcher non deve impedire la
consumazione dei messaggi già recapitati ad una Subscription, ma non ancora letti; parimenti, la distruzione di una Subscription non deve impedire al Dispatcher di
consegnare ulteriori messaggi alle eventuali altre Subscription presenti.

Si implementino le strutture dati Dispatcher e Subscription, a scelta, nel linguaggio Rust o C++11.
*/

use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Dispatcher<Msg: Send + Clone> {
    senders: Mutex<Vec<Sender<Msg>>>,
}

impl<Msg: Send + Clone> Dispatcher<Msg> {
    pub fn new() -> Self {
        Self {
            senders: Mutex::new(Vec::new()),
        }
    }

    pub fn dispatch(&self, msg: Msg) {
        let mut lock = self.senders.lock().unwrap();
        lock.retain(|s| s.send(msg.clone()).is_ok()); // elimino i sender nel caso di Subscription distrutta
    }

    pub fn subscribe(&self) -> Subscription<Msg> {
        let (tx, rx) = channel();
        let mut lock = self.senders.lock().unwrap();
        lock.push(tx);
        Subscription { rx }
    }
}

pub struct Subscription<Msg: Send + Clone> {
    rx: Receiver<Msg>,
}

impl<Msg: Send + Clone> Subscription<Msg> {
    pub fn read(&self) -> Option<Msg> {
        self.rx.recv().ok()
    }
}

fn main() {
    let dispatcher = Arc::new(Dispatcher::<u32>::new());
    let d = dispatcher.clone();
    let h1 = thread::spawn(move || {
        let sub = d.subscribe();
        drop(d);
        for _ in 0..5 {
            let e = sub.read();
            println!("**T1 read {e:?}");
        }
    });
    let d = dispatcher.clone();
    let h2 = thread::spawn(move || {
        let sub = d.subscribe();
        drop(d);
        for _ in 0..10 {
            let e = sub.read();
            println!("++T2 read {e:?}");
        }
    });
    let d = dispatcher.clone();
    let h3 = thread::spawn(move || {
        let sub = d.subscribe();
        drop(d);
        for _ in 0..15 {
            let e = sub.read();
            println!("--T3 read {e:?}");
        }
    });
    thread::sleep(Duration::from_secs(3));
    for e in 0..10 {
        println!("  Dispatcher sent {e:?}");
        dispatcher.dispatch(e);
    }
    drop(dispatcher);
    h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();
}
