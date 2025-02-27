# Rust Data Structures

A high-performance data structure library implemented in Rust.

## Overview

This library provides efficient, well-tested, and easy-to-use implementations of common and specialized data structures in Rust. The focus is on performance, memory efficiency, and a clean API.

## Features

- **High Performance**: Optimized implementations with a focus on speed and memory efficiency
- **Memory Safety**: Takes advantage of Rust's ownership model while providing safe abstractions
- **Comprehensive Testing**: Thoroughly tested with unit tests and property-based tests
- **Familiar API**: Designs similar to Rust's standard library where appropriate
- **No Unsafe Code Leakage**: While some implementations use unsafe for performance, all external APIs are safe

## Data Structures

The library includes the following data structures:

- **Vector**: A dynamically-sized array implementation with optimized memory management
- **LinkedList**: A doubly linked list with O(1) operations at both ends
- **BinaryHeap**: A priority queue implemented as a binary heap
- **HashMap**: A high-performance hash table implementation
- **AVLTree**: A self-balancing binary search tree
- **BTree**: A B-tree implementation optimized for disk and memory
- **Trie**: An efficient prefix tree for string-related operations
- **BloomFilter**: A space-efficient probabilistic data structure
- **LRUCache**: A Least Recently Used (LRU) cache implementation

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust_data_structures = "0.1.0"
```

### Example: Vector

```rust
use rust_data_structures::vector::Vector;

fn main() {
    // Create a new vector
    let mut vec = Vector::new();
    
    // Add elements
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    // Access elements
    assert_eq!(vec[0], 1);
    assert_eq!(vec[1], 2);
    assert_eq!(vec[2], 3);
    
    // Remove elements
    assert_eq!(vec.pop(), Some(3));
    
    // Check length
    assert_eq!(vec.len(), 2);
}
```

### Example: LinkedList

```rust
use rust_data_structures::linked_list::LinkedList;

fn main() {
    // Create a new linked list
    let mut list = LinkedList::new();
    
    // Add elements at both ends
    list.push_front(1);
    list.push_back(3);
    list.push_front(0);
    
    // Iterate through the list
    for item in list.iter() {
        println!("{}", item);
    }
    
    // Remove elements from both ends
    assert_eq!(list.pop_front(), Some(0));
    assert_eq!(list.pop_back(), Some(3));
}
```

## Benchmarks

The library includes benchmarks to compare its performance with Rust's standard library and other popular data structure libraries.

To run the benchmarks:

```bash
cargo bench
```

