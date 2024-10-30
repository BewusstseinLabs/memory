// Copyright 2024 Bewusstsein Labs

use crate::memory::{
    Memory,
    heap::Heap
};

#[test]
fn test_heap() {
    let mut heap = Heap::<u8>::new();

    assert_eq!( heap.len(), 0 );

    heap.resize( 10 );

    assert_eq!( heap.len(), 10 );

    for i in 0..10 {
        heap[ i ] = i as u8;
    }

    for i in 0..10 {
        assert_eq!( heap[ i ], i as u8 );
        println!("heap[{}] = {}", i, heap[ i ]);
    }
}