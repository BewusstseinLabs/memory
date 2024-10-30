// Copyright 2024 Bewusstsein Labs

use crate::memory::{
    Memory,
    stack::Stack
};

#[test]
fn test_stack() {
    let mut stack = Stack::<u8, 10>::new();

    assert_eq!( stack.len(), 10 );

    for i in 0..10 {
        stack[ i ] = i as u8;
    }

    for i in 0..10 {
        assert_eq!( stack[ i ], i as u8 );
        println!("stack[{}] = {}", i, stack[ i ]);
    }
}