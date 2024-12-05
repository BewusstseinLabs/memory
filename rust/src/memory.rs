// Copyright 2024 Bewusstsein Labs

pub mod heap;
pub mod stack;

use std::ops::{ Index, IndexMut };

pub trait MemoryType {
    type Data<T>: Default + Clone + Index<usize, Output = T> + IndexMut<usize, Output = T> + IntoIterator<Item = T> where T: 'static + Default + Copy;
}

pub trait MemoryTraits: Index<usize, Output = Self::Type> + IndexMut<usize, Output = Self::Type> + IntoIterator<Item = Self::Type> {
    type Type: Copy + Default;
    type New;
    type Take;
    type Data: Default + Index<usize, Output = Self::Type> + IndexMut<usize, Output = Self::Type>;

    fn new( new_type: Self::New ) -> Self;
    fn take( take_type: Self::Take ) -> Self;
    fn cap( &self ) -> usize;
    fn len( &self ) -> usize;
    fn is_empty( &self ) -> bool { self.len() == 0 }
    fn reserve( &mut self, cap: usize );
    fn resize( &mut self, cap: usize, value: Self::Type );
    fn push( &mut self, value: Self::Type );
    fn pop( &mut self ) -> Option<Self::Type>;
    fn as_ptr( &self ) -> *const Self::Type;
    fn iter( &self ) -> std::slice::Iter<Self::Type>;
    fn iter_mut( &mut self ) -> std::slice::IterMut<Self::Type>;
}

#[derive( Clone, Default )]
pub struct Memory<T, U>( U::Data<T> )
where
    T: 'static + Copy + Default,
    U: MemoryType;

impl<T, U> Index<usize> for Memory<T, U>
where
    T: Default + Copy,
    U: MemoryType
{
    type Output = T;

    fn index( &self, index: usize ) -> &Self::Output {
        &self.0[ index ]
    }
}

impl<T, U> IndexMut<usize> for Memory<T, U>
where
    T: Default + Copy,
    U: MemoryType
{
    fn index_mut( &mut self, index: usize ) -> &mut Self::Output {
        &mut self.0[ index ]
    }
}

impl<T, U> IntoIterator for Memory<T, U>
where
    T: Default + Copy,
    U: MemoryType
{
    type Item = T;
    type IntoIter = <U::Data<T> as IntoIterator>::IntoIter;

    fn into_iter( self ) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T, U> IntoIterator for &'a Memory<T, U>
where
    T: Default + Copy,
    U: MemoryType,
    Memory<T, U>: MemoryTraits<Type = T>
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter( self ) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, U> IntoIterator for &'a mut Memory<T, U>
where
    T: Default + Copy,
    U: MemoryType,
    Memory<T, U>: MemoryTraits<Type = T>
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter( self ) -> Self::IntoIter {
        self.iter_mut()
    }
}
