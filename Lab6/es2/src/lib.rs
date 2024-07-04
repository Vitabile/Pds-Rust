pub mod thread_pool {
    use std::{sync::{mpsc::{channel, Sender}, Arc, Mutex}, thread};

    enum JobMessage{
        NewJob(Box<dyn FnOnce() + Send>),
        WorkerDone(u32), // id del worker libero
    }

    pub struct ThreadPool {        
        tx_event: Sender<JobMessage>,
    }
   
    impl ThreadPool {
        pub fn new(n_workers: usize) -> Self {
            // EventChannel
            let (tx_event, rx_event) = channel::<JobMessage>();
            // JobsChannel
            let (tx_job, rx_job) = channel::<Box<dyn FnOnce() + Send>>();
            let rx_job = Arc::new(Mutex::new(rx_job));
            for idx in 0..n_workers {
                // use a arc with a mutex to access and share the rx across workers
                let rx_job = rx_job.clone();
                let tx_msg = tx_event.clone();

                thread::spawn(move || {
                    loop{
                        // get the receiver
                        let rx_job = rx_job.lock().unwrap();
                        // wait for a Job on the channel
                        let job = rx_job.recv().unwrap();
                        // execute the job()
                        job();
                        // send work done
                        tx_msg.send(JobMessage::WorkerDone(idx as u32)); 
                    }
                    
                });
            }


            // run the scheduler
            thread::spawn( move || {
                loop {
                    let event = rx_event.recv().unwrap(); // waiting for a event
                    match event {
                        JobMessage::NewJob(job) => {
                            tx_job.send(job);
                        }
                        JobMessage::WorkerDone(id) => {
                            println!("Worker {id} done!");
                        }
                    }
                }
            });

            Self {tx_event}
        }

        // manda un messaggio di new job allo scheduler
        pub fn execute(&self, job: Box<dyn FnOnce() + Send>){
            self.tx_event.send(JobMessage::NewJob(job));
        }
        
    }
}