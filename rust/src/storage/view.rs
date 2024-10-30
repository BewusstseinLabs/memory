use std::ops::{ Index, IndexMut };

use crate::memory::Memory;
use crate::storage::Storage;

#[derive(Debug)]
pub struct View<'a, T, M>
where
    T: Default + Copy + Clone,
    M: Memory<T> + Index<usize, Output = T> + IndexMut<usize, Output = T>
{
    mem: &'a mut M,
    start: usize,
    end: usize,
}

impl<'a, T, M> View<'a, T, M>
where
    T: Default + Copy,
    M: Memory<T> + Index<usize, Output = T> + IndexMut<usize, Output = T>
{
    fn new( mem: &'a mut M, start: usize, end: usize ) -> Self {
        assert!( start <= end && end <= mem.mem_cap(), "Invalid view range" );
        View { mem, start, end }
    }

    fn len( &self ) -> usize {
        self.end - self.start
    }
}

impl<'a, T, M> Index<usize> for View<'a, T, M>
where
    T: Default + Copy,
    M: Memory<T> + Index<usize, Output = T> + IndexMut<usize, Output = T>
{
    type Output = T;

    fn index( &self, index: usize ) -> &Self::Output {
        assert!( index < self.len(), "Index out of bounds" );
        &self.storage[ self.start + index ]
    }
}

impl<'a, T, M> IndexMut<usize> for View<'a, T, M>
where
    T: Default + Copy,
    M: Memory<T> + Index<usize, Output = T> + IndexMut<usize, Output = T>
{
    fn index_mut( &mut self, index: usize ) -> &mut Self::Output {
        assert!( index < self.len(), "Index out of bounds" );
        &mut self.storage[ self.start + index ]
    }
}

impl<'a, T, M> Storage<T, M> for View<'a, T, M>
where
    T: Default + Copy,
    M: Memory<T> + Index<usize, Output = T> + IndexMut<usize, Output = T>
{
    type Type = ( &'a mut M, usize, usize );

    fn new( mem: Self::Type ) -> Self {
        Self::new( mem.0, mem.1, mem.2 )
    }

    fn from( _: &[T] ) -> Self {
        Self::new( &mut M::new( 0 ), 0, 0 )
    }

    fn cap( &self ) -> usize {
        0
    }

    fn len( &self ) -> usize {
        0
    }

    fn push( &mut self, _: T ) {
        // Do nothing
    }

    fn pop( &mut self ) -> Option<T> {
        None
    }

    fn resize( &mut self, _: usize ) {
        // Do nothing
    }
}
