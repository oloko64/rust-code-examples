type Link<T> = Option<Box<Node<T>>>;

/// A implementation of a single linked list
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        Self { head: None }
    }

    pub fn push(&mut self, data: T) {
        // Getting the old head, and leaving None in its place
        // This is done to avoid leaving the list in a invalid state
        let old_head = self.head.take();
        let new_head = Node {
            data,
            next: old_head,
        };

        self.head = Some(Box::new(new_head));
    }

    pub fn pop(&mut self) -> Option<T> {
        // Getting the old head, and leaving None in its place
        // This is done to avoid leaving the list in a invalid state
        let old_head = self.head.take();
        old_head.map(|n| {
            self.head = n.next;
            n.data
        })

        // Same as above
        // match old_head {
        //     Some(n) => {
        //         self.head = n.next;
        //         Some(n.data)
        //     }
        //     None => None,
        // }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.data)

        // Same as above
        // match self.head {
        //     Some(ref n) => Some(&n.data),
        //     None => None,
        // }
    }
}
