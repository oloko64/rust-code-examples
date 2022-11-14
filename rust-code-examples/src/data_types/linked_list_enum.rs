use std::mem;

#[derive(Debug)]
pub enum LinkedList<T> {
    Empty,
    Elem(T, Box<LinkedList<T>>),
}

// This implementation of LinkedList does not require T to derive Clone
// It use mem::replace to swap the current node with the next one
impl<T> LinkedList<T> {
    /// Create a new LinkedList with Empty as the first node
    pub fn new() -> Self {
        LinkedList::Empty
    }

    /// Add a new element to the back of the LinkedList
    pub fn push_back(&mut self, elem: T) {
        match self {
            LinkedList::Empty => {
                *self = LinkedList::Elem(elem, Box::new(LinkedList::Empty));
            }
            LinkedList::Elem(_, next) => {
                next.push_back(elem);
            }
        }
    }

    /// Add a new element to the front of the LinkedList
    pub fn push_front(&mut self, elem: T) {
        let new_list = LinkedList::Elem(elem, Box::new(mem::replace(self, LinkedList::Empty)));
        *self = new_list;
    }

    /// Remove the first element of the LinkedList and return it
    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(self, LinkedList::Empty) {
            LinkedList::Empty => None,
            LinkedList::Elem(elem, next) => {
                *self = *next;
                Some(elem)
            }
        }
    }
}
