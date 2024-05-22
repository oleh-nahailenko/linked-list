
pub struct LinkedList<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>
}

impl<T> LinkedList<T> {

    pub fn new() -> Self {
        LinkedList { head: Link::None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value: value,
            next: self.head.take()
        });

        self.head = Link::Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.value
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.value
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();
        while let Link::Some(mut boxed_value) = current_link {
            current_link = boxed_value.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::second;

    use super::LinkedList;

    #[test]
    fn pop_empty_linked_list_returns_none() {
        let mut list: second::LinkedList<i32> = LinkedList::new();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn pop_returns_values_sequentially() {
        let mut list: second::LinkedList<i32> = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
    }

    #[test]
    fn peek_empty_linked_list_returns_none() {
        let list: second::LinkedList<i32> = LinkedList::new();

        assert_eq!(list.peek(), None);
    }

    #[test]
    fn peek_linked_list_has_head_returns_some() {
        let mut list: second::LinkedList<i32> = LinkedList::new();
        list.push(1);

        assert_eq!(list.peek(), Some(&1));
    }

    #[test]
    fn peek_mut_empty_linked_list_returns_none() {
        let mut list: second::LinkedList<i32> = LinkedList::new();

        assert_eq!(list.peek_mut(), None);
    }

    #[test]
    fn peek_mut_linked_list_has_head_returns_some() {
        let mut list: second::LinkedList<i32> = LinkedList::new();
        list.push(1);

        assert_eq!(list.peek_mut(), Some(&mut 1));
        list.peek_mut().map(|value| {
            *value = 11
        });

        assert_eq!(list.peek(), Some(&11));
        assert_eq!(list.pop(), Some(11));
    }
}