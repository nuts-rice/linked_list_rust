//Singly linked queue 
//Follow basic patterns of stack except with LIFO: remember, queues are like lines
//Since this is singly-linked, we can actually move either operation to the end


//We could store a pointer to END of list and work from there


use std::mem;

pub struct List<'a, T>{
    head: Link<T>,
    tail: Option<&'a mut Node<T>>, //This part is important. We need to be explicit about lifetimes
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T>{
    elem: T,
    next: Link<T>,
}

impl <'a, T> List<'a, T> {

    pub fn new() -> Self {
        List {head: None, tail: None}
    }

    //We could explicitly state lifetime of self here
    pub fn push(&'a mut self, elem: T) {
        let new_tail = Box::new(Node {
            elem: elem,
            //When you push onto the tail, your next is always None here. Last in line
            next: None,
        });

        //Push the into the right place and then grab a ref to the node
        //Following borrowing patterns again here 
        let new_tail = match self.tail.take(){

        Some(old_tail) => {
                //If the old tail existied, update it to point to the new tail
                old_tail.next = Some(new_tail);
                old_tail.next.as_deref_mut()
            }
            None => {
                //Otherwise, update the head to point to it 
                self.head = Some(new_tail);
                self.head.as_deref_mut()
            }
        };
        //Implicit return here to extract the tail reference 
        self.tail = new_tail;
    }

    pub fn pop(&'a mut self) -> Option<T> {
        //Grab the lists current head
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            //If we are out of the head make sure to set tail to None
            if self.head.is_none(){
                self.tail = None;
            }
            head.elem
        })
    }
}

mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        //Check empty list behaves right
        assert_eq!(list.pop(), None);
        
        //Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        //FIFO
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        //Check exhaustation
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

    }
}