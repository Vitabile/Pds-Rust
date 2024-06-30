use std::sync::Arc;
use es3::cb;

fn main() {
    
    let abarrier = Arc::new(cb::CyclicBarrier::new(3));
    let mut vt = Vec::new();
    for i in 0..3 {
        let cbarrier = abarrier.clone();
        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                let res = cbarrier.wait(j);
                println!("after barrier {} {}, elements: {:?}", i, j, res);
            }
        }));
    }
    for t in vt {
        t.join().unwrap();
    }
        
}
