pub mod circular_buffer{
    use std::{ops::{Deref, DerefMut, Index, IndexMut}, sync::Mutex};
    pub struct RingBuf<T :Send + Default + Clone> {
        circular_buffer: Mutex<CircularBuffer<T>>,
    }
    impl<T: Send + Default + Clone> RingBuf<T> {
        pub fn new(capacity: usize) -> Self{
            Self{circular_buffer: Mutex::new(CircularBuffer::new(capacity))}
        }

        pub fn read(&self) -> Option<T> {
            let mut cb = self.circular_buffer.lock().unwrap();
            cb.read()
        }

        pub fn write(&self, item: T) -> Result<(), String> {
            let mut cb = self.circular_buffer.lock().unwrap();
            cb.write(item)
        }

        pub fn clear(&mut self) {
            let mut cb = self.circular_buffer.lock().unwrap();
            cb.clear()
        }
        
        pub fn size(&self) -> usize {
            let cb = self.circular_buffer.lock().unwrap();
            cb.size()
        }
        
        // può essere usata quando il buffer è pieno per forzare una
        // scrittura riscrivendo l’elemento più vecchio
        pub fn overwrite(&mut self, item:T) {
            let mut cb = self.circular_buffer.lock().unwrap();
            cb.overwrite(item)
        }
        
        pub fn make_contiguos(&mut self) {
            let mut cb = self.circular_buffer.lock().unwrap();
            cb.make_contiguos()
        }
        
        pub fn tail(&self) -> usize {
            let cb = self.circular_buffer.lock().unwrap();
            cb.tail()
        }
        pub fn head(&self) -> usize {
            let cb = self.circular_buffer.lock().unwrap();
            cb.head()
        }
    }
    
    #[derive(Debug)]
    pub struct CircularBuffer<T: Default + Clone + Send>{ 
        full: bool,
        head: usize,
        tail: usize,
        buffer: Box<[T]>,
        capacity: usize,
    }

    impl<T: Default + Clone + Send> CircularBuffer<T> {
        pub fn new(capacity: usize) -> Self {
            Self {full :false, head: 0, tail:0, buffer: vec![T::default(); capacity].into_boxed_slice(), capacity}
        }
        
        pub fn write(&mut self, item: T) -> Result<(), String> {
            if self.full {
                Err("Buffer pieno!".to_string())
            } else {
                self.buffer[self.tail] = item;
                self.tail = (self.tail + 1) % self.capacity;
                if self.tail == self.head {
                    self.full = true;
                }
                Ok(())
            }
        }
        
        pub fn read(&mut self) -> Option<T> {
            if self.size() == 0 {
                None
            } else {
                let el = self.buffer[self.head].clone();
                self.buffer[self.head] = T::default();
                self.head = (self.head + 1) % self.capacity;
                if self.full {
                    self.full = false;
                }
                Some(el)
            }
            
        }
        
        pub fn clear(&mut self) {
            self.buffer = vec![T::default(); self.capacity].into_boxed_slice();
            self.tail = 0;
            self.full = false;
            self.head = 0;
        }
        
        pub fn size(&self) -> usize {
            if self.full { self.capacity } else if self.tail >= self.head { self.tail - self.head } else { self.capacity - (self.head - self.tail)}
        }
        
        // può essere usata quando il buffer è pieno per forzare una
        // scrittura riscrivendo l’elemento più vecchio
        pub fn overwrite(&mut self, item:T) {
            if self.size() < self.capacity {
                let _ = self.write(item);
            } else {
                self.buffer[self.head] = item;
            }
        }
        
        pub fn make_contiguos(&mut self) {
            if self.head > self.tail {
                let len = self.size();
                let mut buffer = vec![T::default(); self.capacity].into_boxed_slice();
                for i in 0..len{
                    buffer[i] = self.read().unwrap();
                }
                self.head = 0;
                self.tail = len;
                self.buffer = buffer;
            }
        }
        
        pub fn tail(&self) -> usize {
            self.tail
        }
        pub fn head(&self) -> usize {
            self.head
        }
          
    }

    impl<T: Default + Clone + Send> IndexMut<usize>for CircularBuffer<T>{
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            if index > self.size() {
                panic!("Index out of bounds!")
            }else{
                &mut self.buffer[(self.head + index)%self.capacity]
            }
            
        }
    }

    impl<T: Default + Clone + Send> Index<usize> for CircularBuffer<T>{
        type Output = T;
    
        fn index(&self, index: usize) -> &Self::Output {
            if index >= self.size() {
                panic!("Index out of bounds!")
            }else{
                &self.buffer[(self.head + index)%self.capacity]
            }
            
        }
    }

    impl<T: Default + Clone + Send> Deref for CircularBuffer<T>{
        type Target = [T];
    
        fn deref(&self) -> &Self::Target {
            
            if self.tail > self.head {
                &self.buffer[self.head..self.tail]
            }else{
                panic!("The buffer need to be contiguos!")
            }
        }
    }
    impl<T: Default + Clone + Send> DerefMut for CircularBuffer<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            if self.tail > self.head {
                &mut self.buffer[self.head..self.tail]
            }else{
                panic!("The buffer need to be contiguos!")
            }
        }
    }
}
    