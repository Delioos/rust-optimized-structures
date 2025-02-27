//! A high-performance doubly linked list implementation
//!
//! This module provides an optimized doubly linked list implementation
//! with focus on performance and memory efficiency.

use std::ptr::NonNull;
use std::marker::PhantomData;
use std::fmt;
use std::iter::FromIterator;

struct Node<T> {
    element: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node {
            element,
            next: None,
            prev: None,
        }
    }
}

/// A doubly linked list implementation
///
/// This implementation provides a doubly linked list with O(1) operations
/// at both ends.
///
/// # Examples
///
/// ```
/// use rust_data_structures::linked_list::LinkedList;
///
/// let mut list = LinkedList::new();
/// list.push_front(1);
/// list.push_back(2);
/// list.push_front(0);
///
/// assert_eq!(list.len(), 3);
/// assert_eq!(list.pop_front(), Some(0));
/// assert_eq!(list.pop_back(), Some(2));
/// ```
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<Box<Node<T>>>,
}

// Safe to implement Send and Sync if T is Send and Sync
unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    /// Creates a new, empty linked list
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
            marker: PhantomData,
        }
    }

    /// Returns the length of the linked list
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the linked list is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Clears the linked list, removing all elements
    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    /// Adds an element to the front of the linked list
    pub fn push_front(&mut self, element: T) {
        // Create a new node
        let mut node = Box::new(Node::new(element));
        
        // Set the next pointer of the new node to the current head
        node.next = self.head;
        node.prev = None;
        
        let node_ptr = NonNull::new(Box::into_raw(node)).unwrap();
        
        // Update the previous pointer of the old head
        if let Some(head) = self.head {
            unsafe {
                (*head.as_ptr()).prev = Some(node_ptr);
            }
        } else {
            // If the list was empty, this is also the tail
            self.tail = Some(node_ptr);
        }
        
        // Update the head pointer
        self.head = Some(node_ptr);
        self.len += 1;
    }

    /// Adds an element to the back of the linked list
    pub fn push_back(&mut self, element: T) {
        // Create a new node
        let mut node = Box::new(Node::new(element));
        
        // Set the previous pointer of the new node to the current tail
        node.prev = self.tail;
        node.next = None;
        
        let node_ptr = NonNull::new(Box::into_raw(node)).unwrap();
        
        // Update the next pointer of the old tail
        if let Some(tail) = self.tail {
            unsafe {
                (*tail.as_ptr()).next = Some(node_ptr);
            }
        } else {
            // If the list was empty, this is also the head
            self.head = Some(node_ptr);
        }
        
        // Update the tail pointer
        self.tail = Some(node_ptr);
        self.len += 1;
    }

    /// Removes the front element from the linked list and returns it
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|node| {
            // Get the old head
            let node = unsafe { Box::from_raw(node.as_ptr()) };
            
            // Update the head to the next node
            self.head = node.next;
            
            // If there's a new head, update its prev pointer
            if let Some(new_head) = self.head {
                unsafe {
                    (*new_head.as_ptr()).prev = None;
                }
            } else {
                // If there's no new head, the list is empty
                self.tail = None;
            }
            
            self.len -= 1;
            node.element
        })
    }

    /// Removes the back element from the linked list and returns it
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|node| {
            // Get the old tail
            let node = unsafe { Box::from_raw(node.as_ptr()) };
            
            // Update the tail to the previous node
            self.tail = node.prev;
            
            // If there's a new tail, update its next pointer
            if let Some(new_tail) = self.tail {
                unsafe {
                    (*new_tail.as_ptr()).next = None;
                }
            } else {
                // If there's no new tail, the list is empty
                self.head = None;
            }
            
            self.len -= 1;
            node.element
        })
    }

    /// Returns a reference to the front element
    pub fn front(&self) -> Option<&T> {
        unsafe {
            self.head.map(|node| &(*node.as_ptr()).element)
        }
    }

    /// Returns a mutable reference to the front element
    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.head.map(|node| &mut (*node.as_ptr()).element)
        }
    }

    /// Returns a reference to the back element
    pub fn back(&self) -> Option<&T> {
        unsafe {
            self.tail.map(|node| &(*node.as_ptr()).element)
        }
    }

    /// Returns a mutable reference to the back element
    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.tail.map(|node| &mut (*node.as_ptr()).element)
        }
    }

    /// Returns an iterator over the linked list
    pub fn iter(&self) -> Iter<T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            marker: PhantomData,
        }
    }

    /// Returns a mutable iterator over the linked list
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            len: self.len,
            marker: PhantomData,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = LinkedList::new();
        for item in self.iter() {
            new_list.push_back(item.clone());
        }
        new_list
    }
}

impl<T: fmt::Debug> fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = LinkedList::new();
        for item in iter {
            list.push_back(item);
        }
        list
    }
}

impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}

/// An iterator over the linked list
pub struct Iter<'a, T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| {
                unsafe {
                    // Move the head pointer to the next node
                    let node_ref = &*node.as_ptr();
                    self.head = node_ref.next;
                    self.len -= 1;
                    
                    // Return a reference to the current node's element
                    &node_ref.element
                }
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|node| {
                unsafe {
                    // Move the tail pointer to the previous node
                    let node_ref = &*node.as_ptr();
                    self.tail = node_ref.prev;
                    self.len -= 1;
                    
                    // Return a reference to the current node's element
                    &node_ref.element
                }
            })
        }
    }
}

/// A mutable iterator over the linked list
pub struct IterMut<'a, T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| {
                unsafe {
                    // Move the head pointer to the next node
                    let node_ptr = node.as_ptr();
                    self.head = (*node_ptr).next;
                    self.len -= 1;
                    
                    // Return a mutable reference to the current node's element
                    &mut (*node_ptr).element
                }
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|node| {
                unsafe {
                    // Move the tail pointer to the previous node
                    let node_ptr = node.as_ptr();
                    self.tail = (*node_ptr).prev;
                    self.len -= 1;
                    
                    // Return a mutable reference to the current node's element
                    &mut (*node_ptr).element
                }
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop_front() {
        let mut list = LinkedList::new();
        
        // Push some elements onto the front
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);
        
        assert_eq!(list.len(), 3);
        
        // Pop them off in the expected order
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
        
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_push_pop_back() {
        let mut list = LinkedList::new();
        
        // Push some elements onto the back
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        
        assert_eq!(list.len(), 3);
        
        // Pop them off in the expected order
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
        
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_push_front_pop_back() {
        let mut list = LinkedList::new();
        
        // Push some elements onto the front
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);
        
        // Pop them off from the back
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        
        for item in list.iter_mut() {
            *item *= 2;
        }
        
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);
    }
} 