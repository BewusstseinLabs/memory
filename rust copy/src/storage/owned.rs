// Copyright 2024 Bewusstsein Labs

use std::ops::{ Index, IndexMut };

use crate::memory::{ MemoryTraits, MemoryType, Memory };
use crate::storage::{ StorageTraits, StorageType, Storage, StorageIterator };

#[derive(Debug, Clone)]
pub struct Owned();
impl<T> StorageType<T> for Owned {
    type Mem<M> = Memory<T, M> where T: 'static + Default + Copy, M: MemoryType;
}

impl<T, M> StorageTraits for Storage<T, M, Owned>
where
    T: Copy + Clone + Default,
    M: MemoryType,
    Memory<T, M>: MemoryTraits<Type = T>
{

    type Type = T;
    type Mem = Memory<T, M>;

    fn new( cap: <Self::Mem as MemoryTraits>::New ) -> Self {
        Self {
            mem: <Self::Mem as MemoryTraits>::new( cap ),
            len: 0,
        }
    }

    fn from( src: <Self::Mem as MemoryTraits>::From ) -> Self {
        let mem = <Self::Mem as MemoryTraits>::from( src );
        let len = mem.cap();
        Self {
            mem,
            len
        }
    }

    fn take ( src: <Self::Mem as MemoryTraits>::Take ) -> Self {
        let mem = <Self::Mem as MemoryTraits>::take( src );
        let len = mem.cap();
        Self {
            mem,
            len
        }
    }

    fn cap( &self ) -> usize {
        self.mem.cap()
    }

    fn len( &self ) -> usize {
        self.len
    }

    fn push( &mut self, value: <Self::Mem as MemoryTraits>::Type ) {
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

    fn pop( &mut self ) -> Option<<Self::Mem as MemoryTraits>::Type> {
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

impl<T, M> Storage<T, M, Owned>
where
    T: 'static + Default + Copy,
    M: MemoryType,
    Owned: StorageType<T>,
    Storage<T, M, Owned>: StorageTraits<Type = T>
{
    pub fn iter( &self ) -> StorageIterator<T, M, Owned> {
        self.into_iter()
    }
}

impl <'a, T, M, Owned> Iterator for StorageIterator<'a, T, M, Owned>
where
    T: 'static + Default + Copy,
    M: MemoryType,
    Owned: StorageType<T>,
    Storage<T, M, Owned>: StorageTraits<Type = T>
{
    type Item = T;

    fn next( &mut self ) -> Option<Self::Item> {
        if self.index < self.storage.len() {
            let value = self.storage[ self.index ];
            self.index += 1;
            Some( value )
        } else {
            None
        }
    }
}

impl<'a, T, M, Owned> IntoIterator for &'a Storage<T, M, Owned>
where
    T: 'static + Default + Copy,
    M: MemoryType,
    Owned: StorageType<T>,
    Storage<T, M, Owned>: StorageTraits<Type = T>
{
    type Item = T;
    type IntoIter = StorageIterator<'a, T, M, Owned>;

    fn into_iter(self) -> Self::IntoIter {
        StorageIterator {
            storage: self,
            index: 0
        }
    }
}
