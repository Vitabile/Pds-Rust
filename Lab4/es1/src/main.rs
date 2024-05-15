use crate::es0401::{List1,List2,List3};

mod es0401;
fn main() {
    let mut l1 = List1::List::<i32>::new();
    let mut l2 = List2::List::<i32>::new();
    let mut l3 = List3::DList::<i32>::new();

    println!("==========TESTING LIST1=========");
    println!("{:?}", l1);
    println!("pop {:?} -> {:?}",l1.pop(), l1);
    l1.push(1);
    println!("push: 1 -> {:?}", l1);
    l1.push(2);
    println!("push: 2 -> {:?}", l1);
    println!("pop: {:?} -> {:?}", l1.pop(), l1);
    println!("peek: {:?} -> {:?}", l1.peek(), l1);
    l1.push(2);
    println!("push: 2 -> {:?}", l1);
    l1.push(3);
    println!("push: 3 -> {:?}", l1);
    println!("Take 2 -> New {:?}; Old {:?}", l1.take(2), l1);

    println!("==========TESTING LIST2=========");
    println!("{:?}", l2);
    println!("pop {:?} -> {:?}",l2.pop(), l2);
    l2.push(1);
    println!("push: 1 -> {:?}", l2);
    l2.push(2);
    println!("push: 2 -> {:?}", l2);
    println!("pop: {:?} -> {:?}", l2.pop(), l2);
    println!("peek: {:?} -> {:?}", l2.peek(), l2);
    l2.push(2);
    println!("push: 2 -> {:?}", l2);
    l2.push(3);
    println!("push: 3 -> {:?}", l2);
    println!("Take 2 -> New {:?}; Old {:?}", l2.take(2), l2);

    println!("==========TESTING LIST3=========");
    println!("{:?}", l3);
    l3.push_front(2);
    println!("push_front: 2 -> {:?}", l3);
    l3.push_front(3);
    println!("push_front: 3 -> {:?}", l3);  
    l3.push_front(4);
    println!("push_front: 4 -> {:?}", l3);   
    l3.push_back(1);
    println!("push_back: 1 -> {:?}", l3);
    l3.push_back(0);
    println!("push_back: 0 -> {:?}", l3);
    println!("pop_front: {:?} -> {:?}", l3.pop_front(), l3);
    println!("pop_back: {:?} -> {:?}", l3.pop_back(), l3);
    println!("pop n=2: {:?} -> {:?}", l3.popn(2), l3);
}

