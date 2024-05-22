use std::mem;

pub struct LinkedList {
    head: Link
}

enum Link {
    Empty, 
    More(Box<Node>)
}

struct Node {
    value: i32,
    next: Link
}

impl LinkedList {

    pub fn new() -> Self {
        LinkedList { head: Link::Empty }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value: value,
            next: mem::replace(&mut self.head, Link::Empty)
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut current_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_value) = current_link {
            current_link = mem::replace(&mut boxed_value.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn pop_empty_linked_list_returns_none() {
        let mut list = LinkedList::new();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn pop_returns_values_sequentially() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
    }

}