#![allow(unused)]

use std::thread::current;

#[derive(Clone)]
pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}

#[derive(Clone)]
pub struct Node<T> {
    pub val: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T>{
    pub fn new() -> Self {
        LinkedList { head: Option::None }
    }

    pub fn push(&mut self, val: T) {
        let new_node = Box::new(
            Node {
                val,
                next: Option::None,
            }
        );

        if self.head.is_none() {
            self.head = Some(new_node);
            return;
        }

        let mut current_node = self.head.as_mut().unwrap();

        while !current_node.next.is_none() {
            current_node = current_node.next.as_mut().unwrap();
        }

        current_node.next = Some(new_node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_linked_list_is_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.head.is_none());
    }

    #[test]
    fn test_push_single_element() {
        let mut list = LinkedList::new();
        list.push(42);
        assert!(list.head.is_some());
        assert_eq!(list.clone().head.unwrap().val, 42);
        assert!(list.clone().head.unwrap().next.is_none()); // No next node
    }

    #[test]
    fn test_push_multiple_elements() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // Verify the entire structure of the list
        let mut current_node = list.head.as_ref();
        assert_eq!(current_node.unwrap().val, 1); // First node
        current_node = current_node.unwrap().next.as_ref();
        assert_eq!(current_node.unwrap().val, 2); // Second node
        current_node = current_node.unwrap().next.as_ref();
        assert_eq!(current_node.unwrap().val, 3); // Third node
        assert!(current_node.unwrap().next.is_none()); // No fourth node
    }
}
