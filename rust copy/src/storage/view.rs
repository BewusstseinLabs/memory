// Copyright 2024 Bewusstsein Labs

use std::ops::{ Index, IndexMut };

use crate::memory::{ MemoryTraits, MemoryType, Memory };
use crate::storage::{ StorageTraits, StorageType, Storage };

#[derive(Debug, Clone)]
pub struct View();
impl StorageType for View {}

impl<T, M> StorageTraits for Storage<T, M, View>
where
    T: Copy + Clone + Default,
    M: MemoryType,
    Memory<T, M>: MemoryTraits<Type = T> + Index<usize, Output = T> + IndexMut<usize, Output = T>,
    Self: StorageTraits
{
    type Mem = &'a Memory<T, M>;
    type Type = <Self::Mem as MemoryTraits>::Type;
    type New = ();
    type From = <Self::Mem as MemoryTraits>::From;
    type Take = ();

    fn new( _: Self::New ) -> Self {
        panic!("Cannot create a new view");
    }

    fn from( src: Self::From ) -> Self {
        let mem = <Self::Mem as MemoryTraits>::from( src );
        let len = mem.cap();
        Self {
            mem,
            len
        }
    }

    fn take ( _: Self::Take ) -> Self {
        panic!("Cannot take a view");
    }

    fn cap( &self ) -> usize {
        self.mem.cap()
    }

    fn len( &self ) -> usize {
        self.len
    }

    fn push( &mut self, value: Self::Type ) {
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

    fn pop( &mut self ) -> Option<Self::Type> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some( self.mem[ self.len ] )
        }
    }

    fn resize( &mut self, new_cap: usize ) {
        if new_cap > self.cap() {
            self.mem.resize( new_cap );
        }
    }
}
