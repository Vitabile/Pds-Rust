mod lib;

fn main() {
    let mut cbarrier = lib::cb::CyclicBarrier::new(3);
    let mut vt = Vec::new();
    for i in 0..3 {
        let waiter = cbarrier.get_waiter();
        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                waiter.wait();
                println!("after barrier {} {}", i, j);
            }
        }));
    }
    for handler in vt{
        handler.join().unwrap();
    }
}
