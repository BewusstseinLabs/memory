// Copyright 2024 Bewusstsein Labs

pub mod owned;
//pub mod view;

use std::ops::{ Index, IndexMut };

use crate::memory::{ MemoryTraits, MemoryType, Memory };

pub trait StorageType<T> {
    type Mem<M>: Index<usize, Output = T> + IndexMut<usize, Output = T> where T: 'static + Default + Copy, M: MemoryType;
}

pub trait StorageTraits: Index<usize, Output = <Self::Mem as MemoryTraits>::Type> + IndexMut<usize, Output = <Self::Mem as MemoryTraits>::Type> {
    type Type: Copy + Clone + Default;
    type Mem: MemoryTraits<Type = Self::Type> + Index<usize, Output = Self::Type> + IndexMut<usize, Output = Self::Type>;

    fn new( mem: <Self::Mem as MemoryTraits>::New ) -> Self;
    fn from( src: <Self::Mem as MemoryTraits>::From ) -> Self;
    fn take( src: <Self::Mem as MemoryTraits>::Take ) -> Self;
    fn cap( &self ) -> usize;
    fn len( &self ) -> usize;
    fn push( &mut self, value: <Self::Mem as MemoryTraits>::Type );
    fn pop( &mut self ) -> Option<<Self::Mem as MemoryTraits>::Type>;
    fn resize( &mut self, new_cap: usize );
    //fn view( &mut self, start: usize, end: usize ) -> View<T, M>;
}

pub struct Storage<T, M, S>
where
    T: 'static + Default + Copy,
    M: MemoryType,
    S: StorageType<T>
{
    mem: S::Mem<M>,
    len: usize
}

impl<T, M, S> Index<usize> for Storage<T, M, S>
where
    T: Default + Copy,
    M: MemoryType,
    S: StorageType<T>,
{
    type Output = T;

    fn index( &self, index: usize ) -> &Self::Output {
        &self.mem[ index ]
    }
}

impl<T, M, S> IndexMut<usize> for Storage<T, M, S>
where
    T: Default + Copy,
    M: MemoryType,
    S: StorageType<T>,
{
    fn index_mut( &mut self, index: usize ) -> &mut Self::Output {
        &mut self.mem[ index ]
    }
}

pub struct StorageIterator<'a, T, M, S>
where
    T: 'static + Default + Copy,
    M: MemoryType,
    S: StorageType<T>,
    Storage<T, M, S>: StorageTraits<Type = T>
{
    storage: &'a Storage<T, M, S>,
    index: usize
}
