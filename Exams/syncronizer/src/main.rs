/*
Un sistema embedded riceve su due porte seriali sequenze di dati provenienti da due diversi
sensori. Ciascun sensore genera i dati con cadenze variabili nel tempo e non predicibili, in quanto il
processo di digitalizzazione al suo interno può richiedere più o meno tempo in funzione del dato
letto. Ogni volta che il sistema riceve un nuovo valore su una delle due porte seriali, deve
accoppiarlo con il dato più recente ricevuto sull'altra (se già presente) e inviarlo ad una fase
successiva di computazione. Il sistema al proprio interno utilizza due thread differenti per leggere
dalle due porte seriali e richiede l'uso di un oggetto di sincronizzazione in grado di
implementare la logica descritta sopra. Tale oggetto offre la seguente interfaccia pubblica:

class Synchronizer {
    public:
    Synchronizer(std::function<void(float d1, float d2)> process);
    void dataFromFirstPort(float d1);
    void dataFromSecondPort(float d2);
}

All'atto della costruzione, viene fornita la funzione process(...) che rappresenta la fase successiva
della computazione. Quando vengono invocati i metodi dataFromFirstPort(...) o
dataFromSecondPort(...), se non è ancora presente il dato dalla porta opposta, questi si bloccano al
proprio interno senza consumare CPU, in attesa del valore corrispondente. Al suo arrivo, viene
invocata una sola volta la funzione process(...). Si implementi tale classe utilizzando le funzionalità
offerte dallo standard C++.
*/

use rand::Rng;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

const N_READS: usize = 10;

pub struct Synchronizer {
    process: fn(f32, f32),
    pair: Mutex<(Option<f32>, Option<f32>)>,
    cv: Condvar,
}

impl Synchronizer {
    pub fn new(process: fn(f32, f32)) -> Self {
        Self {
            process,
            pair: Mutex::new((None, None)),
            cv: Condvar::new(),
        }
    }
    pub fn data_from_first_port(&self, data: f32) {
        let mut mutex = self.pair.lock().unwrap();
        if mutex.1.is_none() {
            // se sono il primo invio e aspetto
            mutex.0 = Some(data);
            mutex = self.cv.wait_while(mutex, |m| m.1.is_none()).unwrap(); // dormo finchè è none il secondo elemento
            let data2 = mutex.1.take().unwrap();
            mutex.0 = None;
            println!("process..");
            (self.process)(data, data2);
        } else {
            mutex.0 = Some(data);
            self.cv.notify_all(); // se sono il secondo invio e notifico
        }
    }

    pub fn data_from_second_port(&self, data: f32) {
        let mut mutex = self.pair.lock().unwrap();
        if mutex.0.is_none() {
            // se sono il primo invio e aspetto
            mutex.1 = Some(data);
            mutex = self.cv.wait_while(mutex, |m| m.0.is_none()).unwrap(); // dormo finchè è none il primo elemento
            let data1 = mutex.0.take().unwrap();
            mutex.1 = None;
            println!("process..");
            (self.process)(data1, data);
        } else {
            mutex.1 = Some(data);
            self.cv.notify_all(); // se sono il secondo invio e notifico
        }
    }
}

fn printer<K: std::fmt::Debug>(i1: K, i2: K) {
    println!("> Values received = {:?}, {:?}", i1, i2);
}

fn main() {
    let synchronizer = Arc::new(Synchronizer::new(printer));

    //il sistema sfrutta due threads per leggere dalle porte
    let h1 = thread::spawn({
        let s = synchronizer.clone();
        move || {
            for _ in 0..N_READS {
                let time = rand::thread_rng().gen_range(0..5);
                sleep(Duration::from_secs(time));
                let val = rand::thread_rng().gen_range(0..5);
                println!("sending {val} from port 1");
                s.data_from_first_port(val as f32);
            }
        }
    });

    let h2 = thread::spawn({
        let s = synchronizer.clone();
        move || {
            for _ in 0..N_READS {
                let time = rand::thread_rng().gen_range(0..5);
                sleep(Duration::from_secs(time));
                let val = rand::thread_rng().gen_range(0..5);
                thread::sleep(Duration::from_secs(4));
                println!("sending {val} from port 2");
                s.data_from_second_port(val as f32);
            }
        }
    });

    h1.join().unwrap();
    h2.join().unwrap();
}
