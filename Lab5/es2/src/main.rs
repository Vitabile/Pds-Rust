use std::{sync::Arc, thread, time::Duration};

use es2::circular_buffer::RingBuf;

fn main() {
    let buf: Arc<RingBuf<i32>> = Arc::new(RingBuf::new(5));
    // caso di buffer mediamente full
    let buf1 = buf.clone();
    let producer = thread::spawn(move || {
        loop {
            match buf.write(1) {
                Ok(_) => println!("Write 1!"),
                Err(s) => println!("{s}"),
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
    let reciever = thread::spawn(move ||{
        loop {
            match buf1.read() {
                Some(e) => println!("Read {e}!"),
                None => println!("Empty! Niente da leggere."),
            }
            thread::sleep(Duration::from_secs(2));
        }
    });
    producer.join().unwrap();
    reciever.join().unwrap();
}
