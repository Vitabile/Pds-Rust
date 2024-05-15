pub mod List1 {


    #[derive(Debug)]
    pub enum ListLink<T> {
        Cons(T, Box<ListLink<T>>),
        Nil,
    }
    #[derive(Debug)]
    pub struct List<T> {
        head: ListLink<T>,
    }
    

    impl<T> List<T> {
        pub fn new() -> Self {
            // create a empty ListLink
            Self { head: ListLink::Nil } 
        }
    
        // insert a new element at the beginning of the list
        // you may encouter a problem with the borrow checker while trying to move self.head to a new variable
        // why? look at mem::replace for solving it
        pub fn push(&mut self, elem: T) {
            self.head = ListLink::Cons(elem, Box::new(std::mem::replace(&mut self.head, ListLink::Nil)));
        }

        pub fn pop(&mut self) -> Option<T> {
        
            let old_head = std::mem::replace(&mut self.head, ListLink::Nil);
            
            if let ListLink::Nil = &old_head {
                return None;
            }
            else{
                if let ListLink::Cons(e, next) = old_head {
                    self.head = *next;
                    return Some(e);
                }else{unreachable!()}   
            }
        }

        // return a referece to the first element of the list
        pub fn peek(&self) -> Option<&T> {
            match &self.head{
                ListLink::Nil => None,
                ListLink::Cons(e, _ ) => {return Some(e);}
            }
        }

        // uncomment after having implemented the ListIter struct
        // return an interator over the list values
        
        fn iter(&self) -> ListIter<T> {
           ListIter{ curr: &self.head }
        }

        // take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> List<T>{
            let mut list = List::new();
            let mut list_rev = List::new();
            for _ in 0..n{
                let elem = self.pop();
                if elem.is_none(){
                    break;
                }
                list.push(elem.unwrap());
            }
            while let Some(e) = list.pop() {
                list_rev.push(e);
            }
            list_rev
        }
        
    }


    struct ListIter<'a, T> {
        curr: &'a ListLink<T>,
    }

    impl<'a, T> Iterator for ListIter<'a,T> {
        type Item = &'a T;
    
        fn next(&mut self) -> Option<Self::Item> {
            match self.curr {
                ListLink::Cons(e,next) => {self.curr = &(**next);Some(e)}
                ListLink::Nil => None
            }
        }
    }

     

}

pub mod List2 {
    #[derive(Debug)]
    pub struct Node<T> {
        elem: T,
        next: NodeLink<T>,
    }

    impl<T> Node<T> {
        pub fn new(elem: T) -> Self{
            Self { elem, next: None }
        }
    }
    
    type NodeLink<T> = Option<Box<Node<T>>>;

    #[derive(Debug)]
    pub struct List<T> {
        head: NodeLink<T>,
    }

    // for this implementattion, since we are using option, take a look at the take method in Option<T>.
    // It allows to move the value of the option into another option and replace it with None
    // let mut a = Some(5);
    // let b = a.take(); // a is now None and b is Some(5)
    impl<T> List<T> {
        pub fn new() -> Self {
            // create a empty List
            Self { head: None } 
        }
    
        // insert a new element at the beginning of the list
        pub fn push(&mut self, elem: T) {
            self.head = Some(Box::new(Node{elem, next: self.head.take()}));    
        }

        pub fn pop(&mut self) -> Option<T> {
            let old_head = self.head.take();
            if old_head.is_none(){
                return None;
            }else if let Some(mut e) = old_head {
                self.head = (*e).next.take();
                Some((*e).elem) 
            }else{
                unreachable!()
            }
            
        }
        
        // return a referece to the first element of the list
        pub fn peek(&self) -> Option<&T> {
            match &self.head{
                None => None,
                Some(node) => Some(&(**node).elem)
            }
        }

        
        // uncomment after having implemented the ListIter struct
        // return an interator over the list values
        
        fn iter(&self) -> ListIter<T> {
           ListIter{ curr: &self.head }
        }

        // take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> List<T>{
            let mut list = List::new();
            let mut list_rev = List::new();
            for _ in 0..n{
                let elem = self.pop();
                if elem.is_none(){
                    break;
                }
                list.push(elem.unwrap());
            }
            while let Some(e) = list.pop() {
                list_rev.push(e);
            }
            list_rev
        }
        
    }
    struct ListIter<'a, T> {
        curr: &'a NodeLink<T>,
    }

    impl<'a, T> Iterator for ListIter<'a,T> {
        type Item = &'a T;
    
        fn next(&mut self) -> Option<Self::Item> {
            match self.curr {
                Some(node) => {self.curr = &(**node).next; Some(&(**node).elem)}
                None => None
            }
        }
    }
}

// *****
// double linked list suggestion: use Rc, since we need more than one reference to the same node
// for mutating the list and changing the next and prev fields we also need to be able to mutate the node, therefore we can use RefCell

// how to access content of Rc<RefCell<T>>:
// es let a = Rc::new(RefCell::new(5));
// let mut x = (*a).borrow_mut();  // with (*a) we dereference the Rc, with (*a).borrow_mut() we get a mutable reference to the content of the RefCell
// *x = 6; // we can now change the content of the RefCell

// to take a value from a Rc (useful when popping a value from the list): usually it is not possible since it may be referenced elsewhere.
// if you can guarantee it's the only reference to the value  youu can use Rc::try_unwrap(a).unwrap().into_inner() to get the value
// it first takes out the value from the Rc, then it tries to unwrap the value from the Result, and finally it takes the inner value from the Result
// see here
// https://stackoverflow.com/questions/70404603/how-to-return-the-contents-of-an-rc

// other hint that may be useful: Option<T> has a default clone implementation which calls the clone of T. Therefore: 
// Some(T).clone() ->  Some(T.clone())
// None.clone() -> None


pub mod List3{
    use std::rc::{Rc,Weak};
    use std::cell::{Ref, RefCell};


    type NodeLink<T> = Option<Rc<RefCell<DNode<T>>>>; // we define a type alias for better readibility
    // Example
    type NodeBackLink<T> = Option<Weak<RefCell<DNode<T>>>>;
    
    #[derive(Debug)]
    struct DNode<T> {
        elem: T,
        prev: NodeBackLink<T>, // here we can't put NodeLink to avoid a cycle reference, what do we use?
        next: NodeLink<T>
    }

    impl<T> DNode<T>{
        pub fn new(elem: T) -> Self{
            Self{ elem, prev: None, next: None}
        }
        pub fn with_prev(elem: T, prev: NodeBackLink<T>) -> Self{
            Self{ elem, prev, next: None}
        }
        pub fn with_next(elem: T, next: NodeLink<T>) -> Self{
            Self{ elem, prev: None, next}
        }
        pub fn full_link(elem: T, prev: NodeBackLink<T>, next: NodeLink<T>) -> Self{
            Self{ elem, prev, next}
        }
    }
    #[derive(Debug)]
    pub struct DList<T> {
        head: NodeLink<T>,
        tail: NodeLink<T>
    }

    impl<T: std::fmt::Debug> DList<T> {
        pub fn new() -> Self {
            // create a empty List
            Self { head: None, tail: None } 
        }
        
        /*
        // insert a new element at the head
        pub fn push_front(&mut self, elem: T) {
            if self.head.is_none() && self.tail.is_none(){
                let node = Rc::new(RefCell::new(DNode::new(elem)));
                self.head = Some(node);
                self.tail = self.head.clone();
            }else{
                self.head = Some(Rc::new(
                                    RefCell::new(
                                        DNode::with_next(
                                            elem, 
                                            {
                                                let rc_head = self.head.as_ref().unwrap();
                                                let mut ref_head = (*rc_head).borrow_mut();
                                                (*ref_head).prev = Some(Rc::downgrade(&(self.head.as_ref().unwrap())));
                                                self.head.take()
                                            }
                                        )
                                    )
                                )
                            );
            }
        }
         */

         pub fn push_front(&mut self, elem: T) {
            if self.head.is_none() && self.tail.is_none(){
                let node = Rc::new(RefCell::new(DNode::new(elem)));
                self.head = Some(node);
                self.tail = self.head.clone();
            }else{
                let new_head = Some(Rc::new(RefCell::new(DNode::with_next(elem, self.head.clone()))));
                let rc_head = self.head.take().unwrap();
                let mut ref_head = (*rc_head).borrow_mut();
                (*ref_head).prev = Some(Rc::downgrade(&(new_head.clone().unwrap())));
                self.head = new_head;
            } 
        }  
        

        pub fn push_back(&mut self, elem: T) {
            if self.head.is_none() && self.tail.is_none(){
                let node = Rc::new(RefCell::new(DNode::new(elem)));
                self.head = Some(node);
                self.tail = self.head.clone();
            }else{
                let new_tail = Some(Rc::new(RefCell::new(DNode::with_prev(elem, Some(Rc::downgrade(&(self.tail.clone().unwrap())))))));
                let rc_tail = self.tail.take().unwrap();
                let mut ref_tail = (*rc_tail).borrow_mut();
                (*ref_tail).next = new_tail.clone();
                self.tail = new_tail;
            } 
        }  
        
       
        
        
        
        pub fn pop_front(&mut self) -> Option<T> {
            if self.head.is_none(){
                return None;
            }else{
                let rc_head = self.head.take().unwrap();
                if Rc::strong_count(&rc_head) > 1 {
                    self.head = None;
                    self.tail = None;
                    return Some(Rc::try_unwrap(rc_head).unwrap_or_else(|_| panic!("idk")).into_inner().elem);
                }else{
                    let node = Rc::try_unwrap(rc_head).unwrap_or_else(|_| panic!("idk")).into_inner();
                    let rc_next = node.next.clone().unwrap();
                    let mut ref_next = (*rc_next).borrow_mut();
                    (*ref_next).prev = None;
                    self.head = node.next;
                    return Some(node.elem);
                }
            }
        }

        pub fn pop_back(&mut self) -> Option<T> {
            if self.tail.is_none(){
                return None;
            }else{
                let rc_tail = self.tail.take().unwrap();
                if Rc::strong_count(&self.head.clone().unwrap()) > 2 {
                    self.head = None;
                    self.tail = None;
                    return Some(Rc::try_unwrap(rc_tail).unwrap_or_else(|_| panic!("1")).into_inner().elem);
                }else{
                    // get the previus node and upgrade the weak to rc
                    let a = (*rc_tail).borrow();
                    let up_prev = std::rc::Weak::upgrade(&(*a).prev.as_ref().unwrap()).unwrap();
                    // get the next of the previous and set to None
                    let mut b = (*up_prev).borrow_mut();
                    (*b).next = None;
                    
                    drop(a);
                    // get the elem and update tail
                    let node = Rc::try_unwrap(rc_tail).unwrap_or_else(|_| panic!("2")).into_inner();
                    self.tail = std::rc::Weak::upgrade(&(node.prev.unwrap()));
                    return Some(node.elem);
                }
            }
        }
        pub fn popn(&mut self, n: usize) -> Option<T>{
            let mut i = 0;
            let mut arr:Vec<T> = Vec::new();
            while i <= n {
                let elem = self.pop_front();
                if elem.is_none() {
                    for _ in 0..i {
                        self.push_front(arr.pop().unwrap());
                    }
                    return None;
                }
                if i==n {
                    for _ in 0..i {
                        self.push_front(arr.pop().unwrap());
                    }
                    return elem
                }
                arr.push(elem.unwrap());
                i += 1;
            }
            None
        }
}

}
