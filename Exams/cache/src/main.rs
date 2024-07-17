/*
Un componente con funzionalità di cache permette di ottimizzare il comportamento di un sistema riducendo il numero di volte in cui una funzione è invocata,
tenendo traccia dei risultati da essa restituiti a fronte di un particolare dato in ingresso. Per generalità, si assuma che la funzione accetti un dato di
tipo generico K e restituisca un valore di tipo generico V.

Il componente offre un unico metodo get(...) che prende in ingresso due parametri, il valore k (di tipo K, clonabile) del parametro e la funzione f (di tipo K -> V)
responsabile della sua trasformazione, e restituisce uno smart pointer clonabile al relativo valore.

Se, per una determinata chiave k, non è ancora stato calcolato il valore corrispondente, la funzione viene invocata e ne viene restituito il risultato;
altrimenti viene restituito il risultato già trovato.

Il componente cache deve essere thread-safe perché due o più thread possono richiedere contemporaneamente il valore di una data chiave: quando questo avviene e il dato
non è ancora presente, la chiamata alla funzione dovrà essere eseguita nel contesto di UN SOLO thread, mentre gli altri dovranno aspettare il risultato in corso di
elaborazione, SENZA CONSUMARE cicli macchina.

Si implementi tale componente a scelta nei linguaggi C++ o Rust.
*/

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Eq, PartialEq)]
enum EntryValue<V> {
    Pending,
    Value(Arc<V>),
}

pub struct Cache<K: Clone + Eq + Hash + std::fmt::Display, V: std::fmt::Display> {
    hashmap: Mutex<HashMap<(K, fn(K) -> V), EntryValue<V>>>,
    cv: Condvar,
}

impl<K: Clone + Eq + Hash + std::fmt::Display, V: std::fmt::Display> Cache<K, V> {
    pub fn new() -> Self {
        Self {
            hashmap: Mutex::new(HashMap::new()),
            cv: Condvar::new(),
        }
    }

    pub fn get(&self, k: K, f: fn(K) -> V) -> Arc<V> {
        let mut map = self.hashmap.lock().unwrap();
        if map.contains_key(&(k.clone(), f)) {
            // chiave già presente in cache
            match map.get(&(k.clone(), f)).unwrap() {
                EntryValue::Value(v) => {
                    println!(
                        "Valore trovato in cache!!\n Res: {} per k: {}",
                        v.clone(),
                        k.clone()
                    );
                    return v.clone();
                }
                EntryValue::Pending => {
                    println!("Valore in lavorazione! Sleep... per k: {}", k.clone());
                    map = self
                        .cv
                        .wait_while(map, |m| match m.get(&(k.clone(), f)).unwrap() {
                            EntryValue::Pending => return true,
                            _ => return false,
                        })
                        .unwrap(); // dormo finchè è pending

                    let value = if let EntryValue::Value(v) = map.get(&(k.clone(), f)).unwrap() {
                        Some(v)
                    } else {
                        None
                    };
                    println!(
                        "Valore pronto! Mi risveglio\nRes: {} per k: {}",
                        value.unwrap().clone(),
                        k.clone()
                    );
                    return value.unwrap().clone();
                }
            }
        } else {
            // se non è ancora presente
            map.insert((k.clone(), f), EntryValue::Pending);
            drop(map); // metto pending e lascio il lock
            let res = Arc::new(f(k.clone()));
            map = self.hashmap.lock().unwrap();
            map.insert((k.clone(), f), EntryValue::Value(res.clone()));
            println!(
                "Nuovo valore inserito in cache!\nRes: {} per k: {}",
                res.clone(),
                k.clone()
            );
            self.cv.notify_all();
            return res;
        }
    }
}

fn identity(i: i32) -> f64 {
    if i % 2 == 0 {
        thread::sleep(Duration::from_secs(5));
    }
    i as f64
}
fn square(i: i32) -> f64 {
    (i as f64).powi(2)
}

fn main() {
    let cache = Arc::new(Cache::<i32, f64>::new());
    let mut handles = vec![];
    for i in 0..10 {
        let cache = cache.clone();
        handles.push(thread::spawn(move || {
            let arg = i % 5;
            cache.get(arg, identity);
        }));
    }
    for i in 0..10 {
        let cache = cache.clone();
        handles.push(thread::spawn(move || {
            let arg = i % 4;
            cache.get(arg, square);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}
