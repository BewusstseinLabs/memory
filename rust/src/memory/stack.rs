// Copyright 2024 Bewusstsein Labs

mod test;

use std::ops::{ Index, IndexMut };

use crate::memory::Memory;

#[derive( Debug, Clone, Copy )]
pub struct Stack<T, const CAP: usize>
where
    T: Default + Copy + Clone
{
    mem: [ T; CAP ]
}

impl<T, const CAP: usize> Stack<T, CAP>
where
    T: Default + Copy + Clone
{
    pub fn new() -> Self {
        Stack {
            mem: [ Default::default(); CAP ]
        }
    }

    pub fn from( src: &[T] ) -> Self {
        let mut stack = Stack {
            mem: [ Default::default(); CAP ],
        };
        let len = src.len().min( CAP );
        stack.mem[ ..len ].copy_from_slice( &src[ ..len ] );
        stack
    }

    pub fn take( src: [T; CAP] ) -> Self {
        Stack { mem: src }
    }

    pub fn cap( &self ) -> usize {
        CAP
    }

    pub fn resize( &mut self, _: usize ) {
        // Do nothing
    }

    pub fn as_ptr( &self ) -> *const T {
        self.mem.as_ptr()
    }
}

impl<T, const CAP: usize> Index<usize> for Stack<T, CAP>
where
    T: Default + Copy + Clone
{
    type Output = T;

    fn index( &self, index: usize ) -> &Self::Output {
        &self.mem[ index ]
    }
}

impl<T, const CAP: usize> IndexMut<usize> for Stack<T, CAP>
where
    T: Default + Copy + Clone
{
    fn index_mut( &mut self, index: usize ) -> &mut Self::Output {
        &mut self.mem[ index ]
    }
}

impl<T, const CAP: usize> Memory<T> for Stack<T, CAP>
where
    T: Default + Copy + Clone
{
    type NewType = ();
    type TakeType = [T; CAP];

    fn new( _: Self::NewType ) -> Self {
        Self::new()
    }

    fn from( src: &[T] ) -> Self {
        Self::from( src )
    }

    fn take( src: Self::TakeType ) -> Self {
        Self::take( src )
    }

    fn cap( &self ) -> usize {
        self.cap()
    }

    fn resize( &mut self, _: usize ) {
        // Do nothing
    }

    fn as_ptr( &self ) -> *const T {
        self.as_ptr()
    }
}
