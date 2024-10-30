// Copyright 2024 Bewusstsein Labs

pub mod heap;
pub mod stack;

use std::ops::{ Index, IndexMut };

pub trait Memory<T>: Index<usize, Output = T> + IndexMut<usize, Output = T>
where
    T: Default + Copy
{
    type NewType;
    type TakeType;
    fn new( new_type: Self::NewType ) -> Self;
    fn from( from_type: &[T] ) -> Self;
    fn take( take_type: Self::TakeType ) -> Self;
    fn cap( &self ) -> usize;
    fn resize( &mut self, cap: usize );
    fn as_ptr( &self ) -> *const T;
}
