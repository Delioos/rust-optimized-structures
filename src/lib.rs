// rust_data_structures
// A high-performance data structure library written in Rust

//! # Rust Data Structures
//! 
//! `rust_data_structures` is a collection of high-performance data structures
//! implemented in Rust. This library aims to provide efficient, well-tested,
//! and easy-to-use implementations of common and specialized data structures.

// Module declarations
pub mod vector;
pub mod linked_list;

// TODO: Implement these modules
// pub mod binary_heap;
// pub mod hash_map;
// pub mod avl_tree;
// pub mod btree;
// pub mod trie;
// pub mod bloom_filter;
// pub mod lru_cache;

// Re-exports for convenient access
pub use vector::Vector;
pub use linked_list::LinkedList;

// TODO: Re-export these when implemented
// pub use binary_heap::BinaryHeap;
// pub use hash_map::HashMap;
// pub use avl_tree::AVLTree;
// pub use btree::BTree;
// pub use trie::Trie;
// pub use bloom_filter::BloomFilter;
// pub use lru_cache::LRUCache;

/// Library version information
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_exists() {
        assert!(!super::version().is_empty());
    }
}
