// Copyright 2024 Bewusstsein Labs

pub mod heap;
pub mod stack;

use std::ops::{ Index, IndexMut };

pub trait MemoryType {
    type Data<T>: Index<usize, Output = T> + IndexMut<usize, Output = T> where T: 'static + Default + Copy;
}

pub trait MemoryTraits: Index<usize, Output = Self::Type> + IndexMut<usize, Output = Self::Type> {
    type Data: Index<usize, Output = Self::Type> + IndexMut<usize, Output = Self::Type>;
    type Type: Copy + Clone + Default;
    type New;
    type From;
    type Take;

    fn new( new_type: Self::New ) -> Self;
    fn from( from_type: Self::From ) -> Self;
    fn take( take_type: Self::Take ) -> Self;
    fn cap( &self ) -> usize;
    fn resize( &mut self, cap: usize );
    fn as_ptr( &self ) -> *const Self::Type;
}

pub struct Memory<T, U>
where
    T: 'static + Copy + Clone + Default,
    U: MemoryType,
{
    data: U::Data<T>
}

impl<T, U> Index<usize> for Memory<T, U>
where
    T: Default + Copy,
    U: MemoryType
{
    type Output = T;

    fn index( &self, index: usize ) -> &Self::Output {
        &self.data[ index ]
    }
}

impl<T, U> IndexMut<usize> for Memory<T, U>
where
    T: Default + Copy,
    U: MemoryType
{
    fn index_mut( &mut self, index: usize ) -> &mut Self::Output {
        &mut self.data[ index ]
    }
}
