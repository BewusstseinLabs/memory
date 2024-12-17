// Copyright 2024 Bewusstsein Labs

pub mod heap;
pub mod stack;

use std::{
    fmt::Debug,
    ops::{ Deref, Index, IndexMut }
};

pub trait MemoryType {
    type Data<T>: Default + Clone + Debug + Index<usize, Output = T> + IndexMut<usize, Output = T> + IntoIterator<Item = T> where T: 'static + Default + Copy + Debug;
}

pub trait MemoryTraits: Index<usize, Output = Self::Type> + IndexMut<usize, Output = Self::Type> + IntoIterator<Item = Self::Type> {
    type Type: Copy + Default;
    type New;
    type Data: Default + Index<usize, Output = Self::Type> + IndexMut<usize, Output = Self::Type>;

    fn new( new_type: Self::New ) -> Self;
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

impl<T, U> Copy for Memory<T, U>
where
    T: Default + Copy + Debug,
    U: MemoryType + Clone,
    U::Data<T>: Copy
{}

#[derive( Clone, Default, Debug )]
pub struct Memory<T, U>( U::Data<T> )
where
    T: 'static + Copy + Default + Debug,
    U: MemoryType;

impl<T, U> Deref for Memory<T, U>
where
    T: Default + Copy + Debug,
    U: MemoryType
{
    type Target = U::Data<T>;

    fn deref( &self ) -> &Self::Target {
        &self.0
    }
}

impl<T, U> PartialEq for Memory<T, U>
where
    T: Default + Copy + Debug + PartialEq,
    U: MemoryType,
    U::Data<T>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T, U> Index<usize> for Memory<T, U>
where
    T: Default + Copy + Debug,
    U: MemoryType
{
    type Output = T;

    fn index( &self, index: usize ) -> &Self::Output {
        &self.0[ index ]
    }
}

impl<T, U> IndexMut<usize> for Memory<T, U>
where
    T: Default + Copy + Debug,
    U: MemoryType
{
    fn index_mut( &mut self, index: usize ) -> &mut Self::Output {
        &mut self.0[ index ]
    }
}

impl<T, U> IntoIterator for Memory<T, U>
where
    T: Default + Copy + Debug,
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
    T: Default + Copy + Debug,
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
    T: Default + Copy + Debug,
    U: MemoryType,
    Memory<T, U>: MemoryTraits<Type = T>
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter( self ) -> Self::IntoIter {
        self.iter_mut()
    }
}
