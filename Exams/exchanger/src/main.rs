/*
Si realizzi l’implementazione della struttura dati Exchanger<T: Send> (e dei metodi e delle
funzioni necessarie) utile per realizzare una comunicazione bidirezionale.
Ciascun lato della comunicazione dispone di un’istanza della struttura Exchanger<T: Send>.
La comunicazione avviene invocando il metodo
fn exchange(&self, t:T) -> Option<T>
che, una volta invocato, si blocca fino a quando non viene invocato il metodo corrispettivo
sulla struttura corrispondente al lato opposto della comunicazione, dopodiché restituisce il
valore che è stato passato come argomento al metodo corrispondente al lato opposto (che
farà altrettanto), sotto forma di Some(t).
Lo scambio può essere ripetuto un numero arbitrario di volte.
Se una delle due strutture formanti la coppia viene distrutta, un'eventuale chiamata, bloccata
sul metodo della struttura restante, terminerà restituendo il valore None.
Si implementi tale struttura in linguaggio Rust avendo cura che la sua implementazione
sia thread-safe.

*/

use std::sync::mpsc::{channel, Receiver, Sender};

pub struct Exchanger<T: Send> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T: Send> Exchanger<T> {
    pub fn new() -> (Exchanger<T>, Exchanger<T>) {
        let (tx1, rx1) = channel::<T>();
        let (tx2, rx2) = channel::<T>();
        (Self { tx: tx2, rx: rx1 }, Self { tx: tx1, rx: rx2 })
    }

    pub fn exchange(&self, t: T) -> Option<T> {
        self.tx.send(t).ok()?; // se l'altra parte viene distrutto recv() fallisce e torna None
        self.rx.recv().ok() // se l'altra parte viene distrutta il send
    }
}

#[cfg(test)]
mod test {
    use crate::Exchanger;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn can_exchange_a_value() {
        let (x1, x2) = Exchanger::new();
        let h1 = thread::spawn(move || {
            assert_eq!(Some(2), x1.exchange(1));
        });
        let h2 = thread::spawn(move || {
            assert_eq!(Some(1), x2.exchange(2));
        });
        h1.join().unwrap();
        h2.join().unwrap();
    }
    #[test]
    fn can_exchange_multiple_values() {
        let (x1, x2) = Exchanger::new();
        let h1 = thread::spawn(move || {
            for i in 0..10 {
                assert_eq!(Some(10 + i), x1.exchange(i));
            }
        });
        let h2 = thread::spawn(move || {
            for i in 10..20 {
                assert_eq!(Some(i - 10), x2.exchange(i));
            }
        });
        h1.join().unwrap();
        h2.join().unwrap();
    }
    #[test]
    fn dropping_an_exchanger_unblocks_the_other() {
        let (x1, x2) = Exchanger::new();
        let h1 = thread::spawn(move || {
            assert_eq!(None, x1.exchange(1));
        });
        thread::sleep(Duration::from_millis(1));
        drop(x2);
        h1.join().unwrap();
    }
}

fn main() {}
