use std::cell::{Ref, RefCell};
use std::rc::Rc;

// An easy way for us to validate if our methods make sense is if we maintain the following invariant:
// each node should have exactly two pointers to it. Each node in the middle of the list is pointed at
// by its predecessor and successor, while the nodes on the ends are pointed to by the list itself.

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

// RefCell<T> uses Rust's lifetimes to implement 'dynamic borrowing',
// a process whereby one can claim temporary, exclusive, mutable access to the inner value.
// Borrows for RefCell<T>s are tracked 'at runtime', unlike Rust's native
// reference types which are entirely tracked statically, at compile time.

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
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

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        //New node needs +2 links, everything else should be +0
        let new_head = Node::new(elem);
        //Option patterns here
        match self.head.take() {
            Some(old_head) => {
                //Non-empty list, need to connect the old head
                // Seems we need to explicitly borrow a RefCell using borrow_mut
                old_head.borrow_mut().prev = Some(new_head.clone()); //+1 New head
                new_head.borrow_mut().next = Some(old_head); //+1 old head
                self.head = Some(new_head); //+1 new head -1 old head
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
            //Here we can do some risky business and try to conver a Result to an option using ok()
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    //Definition of 'borrow' is
    //borrow<'a>(&'a self) -> Ref<'a, T>
    //borrow_mut<'a>(&'a self) -> RefMut<'a, T>

    //RefCell will behave like Rc but for borrowing, This can be enforced
    //During Runtime

    //We *can* follow Option patterns here and map over a Ref as well
    //This follows Monad behavior as well.
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::List;

    //Basic stack tests for pop_front(), push_front()
    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        //Check exhaustation
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
    }
}
