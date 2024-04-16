use circular_buffer::circular_buffer::CircularBuffer;

#[cfg(test)]
mod tests {
    use super::*;

    

    #[test]
    fn add_element(){
        let mut a: CircularBuffer<usize> = CircularBuffer::new(5);
        let _ = a.write(1);
        assert_eq!(a.size(),1);
    }
    
    #[test]
    fn add_then_read_element(){
        let mut a: CircularBuffer<usize> = CircularBuffer::new(5);
        let _ = a.write(1);
        let element = a.read();
        assert_eq!(element.unwrap(),1);
    }
    
    #[test]
    fn add_then_read_n_elements(){
        let mut a: CircularBuffer<usize> = CircularBuffer::new(5);
        for i in 0..5{
            let _ = a.write(i);
        }
        for i in 0..5{
            let element = a.read();
            assert_eq!(element.unwrap(),i)
        }
    }
    

    #[test]
    fn read_from_empty_buffer(){
        let mut a: CircularBuffer<usize> = CircularBuffer::new(5);
        assert_eq!(a.read(),None)
    }

    #[test]
    fn write_full_buffer(){
        let mut a: CircularBuffer<usize> = CircularBuffer::new(1);
        let _ = a.write(1);
        assert!(a.write(2).is_err())
    }
    
    #[test]
    fn overwrite(){
        let mut a: CircularBuffer<usize> = CircularBuffer::new(2);
        let _ = a.overwrite(1);
        let _ = a.overwrite(2);
        let _ = a.overwrite(10);
        assert_eq!(a.read(),Some(10));
    }

    #[test]
    fn make_continues(){
        let mut a: CircularBuffer<usize> = CircularBuffer::new(5);
        let _ = a.write(1);
        let _ = a.write(2);
        let _ = a.write(3);
        a.read();
        a.read();
        let _ = a.write(4);
        let _ = a.write(5);
        let _ = a.write(6);
        // a = [6,_,3,4,5]
        a.make_contiguos();
        // a = [3,4,5,6,_]
        assert_eq!(a.head(),0);
        assert_eq!(a.tail(),4);

    }
    #[test]
    fn check_index_trait(){
        let mut buf: CircularBuffer<usize> = CircularBuffer::new(5);
        let _ = buf.write(1);
        let _ = buf.write(2);
        let _ = buf.write(3);
        assert_eq!(buf[0], 1);
        assert_eq!(buf[1], 2);
        assert_eq!(buf[2], 3);
        buf[1] = 10;
        assert_eq!(buf[1],10);
    }

    #[test]
    fn check_deref_trait(){
        let mut buf: CircularBuffer<usize> = CircularBuffer::new(5);
        let _ = buf.write(1);
        let _ = buf.write(2);
        let _ = buf.write(3);
        assert_eq!([1,2,3],*buf);
    }
}