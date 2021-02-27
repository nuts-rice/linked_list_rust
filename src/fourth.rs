use std::rc::Rc;
use std::cell::RefCell;

// An easy way for us to validate if our methods make sense is if we maintain the following invariant: 
// each node should have exactly two pointers to it. Each node in the middle of the list is pointed at 
// by its predecessor and successor, while the nodes on the ends are pointed to by the list itself.


pub struct List<T>{
    head: Link<T>,
    tail: Link<T>,
}

// RefCell<T> uses Rust's lifetimes to implement 'dynamic borrowing', 
// a process whereby one can claim temporary, exclusive, mutable access to the inner value. 
// Borrows for RefCell<T>s are tracked 'at runtime', unlike Rust's native 
// reference types which are entirely tracked statically, at compile time. 

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node <T>{
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}
// Sometimes a type needs to be mutated while having multiple aliases. In Rust this is
// achieved using a pattern called interior mutability. A type has interior mutability if 
// its internal state can be changed through a shared reference to it. This goes against the usual requirement that 
// the value pointed to by a shared reference is not mutated.

// there are occasions when interior mutability might be appropriate
//     Introducing inherited mutability roots to shared types.
//     Implementation details of logically-immutable methods.
//     Mutating implementations of Clone.

impl<T> Node<T>{
    fn new(elem: T) -> Rc<RefCell<Self>>{
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List{head: None, tail: None}
    }
    
    pub fn push_front(&mut self, elem: T){
        //New node needs +2 links, everything else should be +0
        let new_head = Node::new(elem);
        //Option patterns here
        match self.head.take() {
            Some(old_head) =>{
                //Non-empty list, need to connect the old head
                // Seems we need to explicitly borrow a RefCell using borrow_mut
                old_head.borrow_mut().prev = Some(new_head.clone()); //+1 New head
                new_head.borrow_mut().next = Some(old_head);         //+1 old head
                self.head = Some(new_head);             //+1 new head -1 old head
                //total: +2 new_head, +0 old_head--OK!!!
            }
            None => {
                //Empty list, need to set tail
                self.tail = Some(new_head.clone()); //+1 new_head
                self.head = Some(new_head); // +1 new_head
                //total: +2 new_head -- OK!
            }

        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        //Need to take the old head, ensure it's -2
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    //Not emptying list
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                    //total: -2 old, +0 new
                }
                None => {
                    //emptying list
                    self.tail.take();
                    //total: -2 old, (no new)
                }
            }
            old_head.borrow_mut().elem
        })
    }
}