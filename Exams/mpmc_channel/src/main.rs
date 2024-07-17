/*
La struct MpMcChannel<E: Send> è una implementazione di un canale su cui possono scrivere molti produttori e da cui possono attingere valori molti consumatori.
Tale struttura offre i seguenti metodi:

    new(n: usize) -> Self    //crea una istanza del canale basato su un buffer circolare di "n" elementi

    send(e: E) -> Option<()>    //invia l'elemento "e" sul canale. Se il buffer circolare è pieno, attende
                                //senza consumare CPU che si crei almeno un posto libero in cui depositare il valore
                                //Ritorna:
                                    // - Some(()) se è stato possibile inserire il valore nel buffer circolare
                                    // - None se il canale è stato chiuso (Attenzione: la chiusura può avvenire anche
                                    //    mentre si è in attesa che si liberi spazio) o se si è verificato un errore interno

    recv() -> Option<E>         //legge il prossimo elemento presente sul canale. Se il buffer circolare è vuoto,
                                //attende senza consumare CPU che venga depositato almeno un valore
                                //Ritorna:
                                    // - Some(e) se è stato possibile prelevare un valore dal buffer
                                    // - None se il canale è stato chiuso (Attenzione: se, all'atto della chiusura sono
                                    //    già presenti valori nel buffer, questi devono essere ritornati, prima di indicare
                                    //    che il buffer è stato chiuso; se la chiusura avviene mentre si è in attesa di un
                                    //    valore, l'attesa si sblocca e viene ritornato None) o se si è verificato un errore interno.

    shutdown() -> Option<()>    //chiude il canale, impedendo ulteriori invii di valori.
                                //Ritorna:
                                    // - Some(()) per indicare la corretta chiusura
                                    // - None in caso di errore interno all'implementazione del metodo.

Si implementi tale struttura dati in linguaggio Rust, senza utilizzare i canali forniti dalla libreria standard né da altre librerie, avendo cura di garantirne
la correttezza in presenza di più thread e di non generare la condizione di panico all'interno dei suoi metodi.
*/

use rand::Rng;
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

struct Channel<T: Send> {
    buff: VecDeque<T>,
    n: usize,
    open: bool,
}

pub struct MpMcChannel<T: Send> {
    ch: Mutex<Channel<T>>,
    cv: Condvar,
}

impl<T: Send> MpMcChannel<T> {
    pub fn new(n: usize) -> Self {
        Self {
            ch: Mutex::new(Channel {
                buff: VecDeque::<T>::new(),
                n,
                open: true,
            }),
            cv: Condvar::new(),
        }
    }

    pub fn send(&self, e: T) -> Option<()> {
        let mut mutex = self.ch.lock().unwrap();
        if !mutex.open {
            return None;
        }; // controllo canale chiuso
        mutex = self.cv.wait_while(mutex, |m| m.buff.len() == m.n).unwrap(); // aspetto finchè il buffer è pieno
        if !mutex.open {
            return None;
        }; // se il canale è stato chiuso
        mutex.buff.push_back(e);
        self.cv.notify_all();
        Some(())
    }

    pub fn recv(&self) -> Option<T> {
        let mut mutex = self.ch.lock().ok()?;
        if !mutex.open && mutex.buff.len() == 0 {
            return None;
        }; // se il canale è chiuso e non ci sono più elementi None
        mutex = self
            .cv
            .wait_while(mutex, |m| m.buff.len() == 0 && m.open)
            .ok()?; // aspetto finchè il buffer è vuoto ed è aperto
        if !mutex.open && mutex.buff.len() == 0 {
            return None;
        }; // se il canale è chiuso e non ci sono più elementi None
        let t = mutex.buff.pop_front()?;
        self.cv.notify_all();
        Some(t)
    }

    pub fn shutdown(&self) -> Option<()> {
        let mut mutex = self.ch.lock().ok()?;
        mutex.open = false;
        self.cv.notify_all();
        Some(())
    }
}

fn main() {
    println!("Please note that the print might not be perfectly synchronized");
    println!("As a matter of fact, in order to grant full synchronization print should be moved when thread has lock");
    println!("Inside both the send() and the return() function\n\n");

    let channel = Arc::new(MpMcChannel::new(5));

    let mut handles = vec![];

    for i in 0..4 {
        handles.push(thread::spawn({
            let channel = channel.clone();
            move || {
                for j in 0..8 {
                    if i < 2 {
                        let time = rand::thread_rng().gen_range(0..1);
                        sleep(Duration::from_secs(time));
                        println!("thread {} sending {}", i, j);
                        channel.send((i, j));
                        if j == 8 {
                            println!("shutting down...");
                            channel.shutdown();
                        }
                    } else {
                        let time = rand::thread_rng().gen_range(2..3);
                        sleep(Duration::from_secs(time));
                        let e = channel.recv();
                        println!(
                            "thread {} received from thread {} value {}",
                            i,
                            e.unwrap().0,
                            e.unwrap().1
                        );
                    }
                }
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}
