//! A high-performance vector implementation
//!
//! This module provides an optimized vector (dynamic array) implementation
//! with focus on performance and memory efficiency.

use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::ptr::{self, NonNull};

/// A high-performance vector implementation
///
/// This implementation uses raw pointers and manual memory management
/// to achieve better performance than the standard library's Vec while
/// maintaining a similar API.
///
/// # Examples
///
/// ```
/// use rust_data_structures::vector::Vector;
///
/// let mut vec = Vector::new();
/// vec.push(1);
/// vec.push(2);
/// vec.push(3);
///
/// assert_eq!(vec[0], 1);
/// assert_eq!(vec[1], 2);
/// assert_eq!(vec[2], 3);
/// assert_eq!(vec.len(), 3);
/// ```
pub struct Vector<T> {
    ptr: NonNull<T>,
    capacity: usize,
    len: usize,
    _marker: PhantomData<T>,
}

// Implement Default for Vector<T>
impl<T> Default for Vector<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Safe to implement Send and Sync if T is Send and Sync
unsafe impl<T: Send> Send for Vector<T> {}
unsafe impl<T: Sync> Sync for Vector<T> {}

impl<T> Vector<T> {
    /// Creates a new, empty vector
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            capacity: 0,
            len: 0,
            _marker: PhantomData,
        }
    }

    /// Creates a new vector with the specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            return Self::new();
        }

        // Allocate memory for the specified capacity
        let layout = Layout::array::<T>(capacity).unwrap();
        let ptr = unsafe { 
            NonNull::new(alloc::alloc(layout) as *mut T).unwrap() 
        };

        Self {
            ptr,
            capacity,
            len: 0,
            _marker: PhantomData,
        }
    }

    /// Returns the current length of the vector
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the vector is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the current capacity of the vector
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Adds an element to the end of the vector
    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.grow();
        }

        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), value);
        }
        
        self.len += 1;
    }

    /// Removes the last element from the vector and returns it
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(ptr::read(self.ptr.as_ptr().add(self.len)))
            }
        }
    }

    /// Gets a reference to an element at the specified index
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            unsafe {
                Some(&*self.ptr.as_ptr().add(index))
            }
        }
    }

    /// Gets a mutable reference to an element at the specified index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            None
        } else {
            unsafe {
                Some(&mut *self.ptr.as_ptr().add(index))
            }
        }
    }

    // Private method to grow the vector's capacity
    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 1 } else { self.capacity * 2 };
        
        let ptr = if self.capacity == 0 {
            let layout = Layout::array::<T>(new_capacity).unwrap();
            unsafe { 
                NonNull::new(alloc::alloc(layout) as *mut T).unwrap() 
            }
        } else {
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            let new_layout = Layout::array::<T>(new_capacity).unwrap();
            
            unsafe {
                let ptr = alloc::realloc(
                    self.ptr.as_ptr() as *mut u8,
                    old_layout,
                    new_layout.size(),
                );
                NonNull::new(ptr as *mut T).unwrap()
            }
        };

        self.ptr = ptr;
        self.capacity = new_capacity;
    }

    /// Converts the vector into a raw parts tuple
    pub fn into_raw_parts(self) -> (*mut T, usize, usize) {
        let result = (self.ptr.as_ptr(), self.len, self.capacity);
        
        // Avoid running the destructor
        mem::forget(self);
        
        result
    }

    /// Creates a vector from raw parts
    ///
    /// # Safety
    ///
    /// This function is unsafe because it assumes the raw parts are valid.
    pub unsafe fn from_raw_parts(ptr: *mut T, len: usize, capacity: usize) -> Self {
        Self {
            ptr: NonNull::new_unchecked(ptr),
            len,
            capacity,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        if self.capacity == 0 {
            return;
        }

        // Drop all elements
        for i in 0..self.len {
            unsafe {
                ptr::drop_in_place(self.ptr.as_ptr().add(i));
            }
        }

        // Deallocate the memory
        unsafe {
            let layout = Layout::array::<T>(self.capacity).unwrap();
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

impl<T> Deref for Vector<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(self.ptr.as_ptr(), self.len)
        }
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len)
        }
    }
}

impl<T, I: std::slice::SliceIndex<[T]>> Index<I> for Vector<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}

impl<T, I: std::slice::SliceIndex<[T]>> IndexMut<I> for Vector<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut **self, index)
    }
}

impl<T: Clone> Clone for Vector<T> {
    fn clone(&self) -> Self {
        let mut new_vec = Self::with_capacity(self.capacity);
        
        for i in 0..self.len {
            unsafe {
                let item = ptr::read(self.ptr.as_ptr().add(i));
                new_vec.push(item.clone());
            }
        }
        
        new_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vec: Vector<i32> = Vector::new();
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 0);
    }

    #[test]
    fn test_with_capacity() {
        let vec: Vector<i32> = Vector::with_capacity(10);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 10);
    }

    #[test]
    fn test_push_pop() {
        let mut vec = Vector::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.pop(), Some(1));
        assert_eq!(vec.pop(), None);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_get() {
        let mut vec = Vector::new();
        vec.push(1);
        vec.push(2);
        
        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.get(2), None);
    }

    #[test]
    fn test_indexing() {
        let mut vec = Vector::new();
        vec.push(1);
        vec.push(2);
        
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
    }
} 