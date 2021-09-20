use std::{marker::PhantomData, mem, ptr::NonNull};

pub struct MyLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<Box<Node<T>>>,
}

#[allow(dead_code)]
struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    element: T,
}

impl<T> Node<T> {
    #[allow(dead_code)]
    pub fn new(element: T) -> Self {
        Node {
            next: None,
            prev: None,
            element,
        }
    }
}

impl<T> Default for MyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for MyLinkedList<T> {
    fn drop(&mut self) {
        struct DropGuard<'a, T>(&'a mut MyLinkedList<T>);

        impl<'a, T> Drop for DropGuard<'a, T> {
            fn drop(&mut self) {
                while self.0.pop_front_node().is_some() {}
            }
        }

        while let Some(node) = self.pop_front_node() {
            let guard = DropGuard(self);
            drop(node);
            mem::forget(guard);
        }
    }
}

// public method
impl<T> MyLinkedList<T> {
    #[inline]
    pub fn new() -> Self {
        MyLinkedList {
            head: None,
            tail: None,
            len: 0,
            marker: PhantomData,
        }
    }
}

// private method
impl<T> MyLinkedList<T> {
    /// Adds the give node to the front of the list
    #[inline]
    #[allow(dead_code)]
    fn push_front_node(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.next = self.head;
            node.prev = None;
            let node = Some(Box::leak(node).into());

            match self.head {
                None => {
                    self.tail = node;
                }
                Some(head) => (*head.as_ptr()).prev = node,
            }

            self.head = node;
            self.len += 1;
        }
    }

    /// Removes and returns the node at the front of the list
    #[inline]
    #[allow(dead_code)]
    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;

            match self.head {
                None => self.tail = None,
                Some(head) => (*head.as_ptr()).prev = None,
            }

            self.len -= 1;
            node
        })
    }

    /// Adds the given node to the back of list.
    #[inline]
    #[allow(dead_code)]
    fn push_back_node(&mut self, mut node: Box<Node<T>>) {
        // This method takes care not create mutable references to the whole nodes,
        // to maintain validity of aliasing poiners into 'element'
        unsafe {
            node.next = None;
            node.prev = self.tail;
            let node = Some(Box::leak(node).into());

            match self.tail {
                None => self.head = node,
                // Not creating new mutable (unique!) references overlapping 'element'.
                Some(tail) => (*tail.as_ptr()).next = node,
            }

            self.tail = node;
            self.len += 1;
        }
    }

    /// Remove and return the node at the back of list.
    #[inline]
    #[allow(dead_code)]
    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        self.tail.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.tail = node.prev;

            match self.tail {
                None => self.head = None,
                Some(tail) => (*tail.as_ptr()).next = None,
            }

            self.len -= 1;
            node
        })
    }
}
#[cfg(test)]
mod tests {
    use crate::{MyLinkedList, Node};

    #[test]
    fn test_op_my_linked_list() {
        let mut list = MyLinkedList::new();
        list.push_front_node(Box::new(Node::new(3)));
        list.push_front_node(Box::new(Node::new(2)));
        list.push_front_node(Box::new(Node::new(1)));
        let l1 = list.pop_front_node().unwrap().element;
        assert!(l1 == 1);
        let l2 = list.pop_front_node().unwrap().element;
        assert!(l2 == 2);
        let l3 = list.pop_front_node().unwrap().element;
        assert!(l3 == 3);
        list.push_back_node(Box::new(Node::new(3)));
        list.push_back_node(Box::new(Node::new(2)));
        list.push_back_node(Box::new(Node::new(1)));
        let l1 = list.pop_back_node().unwrap().element;
        assert!(l1 == 1);
        let l2 = list.pop_back_node().unwrap().element;
        assert!(l2 == 2);
        let l3 = list.pop_back_node().unwrap().element;
        assert!(l3 == 3);
    }
}
