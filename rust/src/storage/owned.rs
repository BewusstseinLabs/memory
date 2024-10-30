use std::ops::{ Index, IndexMut };

use crate::memory::Memory;
use crate::storage::Storage;

#[derive(Debug, Clone)]
pub struct Owned<T, M>
where
    T: Default + Copy + Clone,
    M: Memory<T> + Index<usize, Output = T> + IndexMut<usize, Output = T>
{
    mem: M,
    len: usize,
}

impl<T, M> Owned<T, M>
where
    T: Default + Copy + Clone,
    M: Memory<T> + Index<usize, Output = T> + IndexMut<usize, Output = T>
{
    pub fn new( cap: usize ) -> Self {
        Owned {
            mem: M::new( cap ),
            len: 0,
        }
    }

    pub fn from( src: &[T] ) -> Self {
        let len = src.len();
        Owned {
            mem: M::from( src ),
            len,
        }
    }

    pub fn cap( &self ) -> usize {
        self.mem.cap()
    }

    pub fn len( &self ) -> usize {
        self.len
    }

    pub fn push( &mut self, value: T ) {
        if self.len == self.cap() {
            self.resize( self.cap() + size_of::<T>() );
        }
        if self.len < self.cap() {
            self.mem[ self.len ] = value;
            self.len += 1;
        } else {
            panic!("Cannot push to a full stack");
        }
    }

    pub fn pop( &mut self ) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some( self.mem[ self.len ] )
        }
    }

    pub fn resize( &mut self, new_cap: usize ) {
        if new_cap > self.cap() {
            self.mem.mem_resize( new_cap );
        }
    }
}

impl<T, M> Index<usize> for Owned<T, M>
where
    T: Default + Copy + Clone,
    M: Memory<T> + Index<usize, Output = T> + IndexMut<usize, Output = T>
{
    type Output = T;
    fn index( &self, index: usize ) -> &Self::Output {
        assert!( index < self.len, "Index out of bounds" );
        &self.mem[ index ]
    }
}

impl<T, M> IndexMut<usize> for Owned<T, M>
where
    T: Default + Copy + Clone,
    M: Memory<T> + Index<usize, Output = T> + IndexMut<usize, Output = T>
{
    fn index_mut( &mut self, index: usize ) -> &mut Self::Output {
        assert!( index < self.len, "Index out of bounds" );
        &mut self.mem[ index ]
    }
}

impl<T, M> Storage<T, M> for Owned<T, M>
where
    T: Default + Copy + Clone,
    M: Memory<T> + Index<usize, Output = T> + IndexMut<usize, Output = T>
{
    type Type = usize;

    fn new( cap: usize ) -> Self {
        Self::new( cap )
    }

    fn from( src: &[T] ) -> Self {
        Self::from( src )
    }

    fn cap( &self ) -> usize {
        self.cap()
    }

    fn len( &self ) -> usize {
        self.len()
    }

    fn push( &mut self, value: T ) {
        self.push( value );
    }

    fn pop( &mut self ) -> Option<T> {
        self.pop()
    }

    fn resize( &mut self, new_cap: usize ) {
        self.resize( new_cap );
    }

    //fn view( &mut self, start: usize, end: usize ) -> View<T, M> {
    //    assert!( start <= end && end <= self.len(), "Invalid view range" );
    //    View::new( self, start, end )
    //}
}

