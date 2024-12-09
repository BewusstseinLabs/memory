// Copyright 2024 Bewusstsein Labs

use std::{
    fmt::Debug,
    ptr::NonNull,
    ops::{ Index, IndexMut, Deref, DerefMut }
};

use crate::memory::{ MemoryTraits, MemoryType, Memory };

#[derive( Copy, Clone, Default, Debug )]
pub struct Heap();
impl MemoryType for Heap {
    type Data<T> = Vec<T> where T: 'static + Default + Copy + Debug;
}

impl<T> MemoryTraits for Memory<T, Heap>
where
    T: 'static + Default + Copy + Clone + Debug
{
    type Type = T;
    type New = usize;
    type Take = Vec<T>;
    type Data = Vec<T>;

    fn new( cap: usize ) -> Self {
        Self ( Vec::with_capacity( cap ) )
    }

    fn take( src: Vec<T> ) -> Self {
        Self ( src )
    }

    fn cap( &self ) -> usize {
        self.0.capacity()
    }

    fn len( &self ) -> usize {
        self.0.len()
    }

    fn reserve( &mut self, cap: usize ) {
        self.0.reserve( cap );
    }

    fn resize( &mut self, cap: usize, value: Self::Type ) {
        self.0.resize( cap, value );
    }

    fn push( &mut self, value: Self::Type ) {
        self.0.push( value );
    }

    fn pop( &mut self ) -> Option<Self::Type> {
        self.0.pop()
    }

    fn as_ptr( &self ) -> *const T {
        self.0.as_ptr()
    }

    fn iter( &self ) -> std::slice::Iter<Self::Type> {
        self.0.iter()
    }

    fn iter_mut( &mut self ) -> std::slice::IterMut<Self::Type> {
        self.0.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let heap = Memory::<u32, Heap>::new( 10 );
        assert_eq!( heap.cap(), 10 );
    }

    #[test]
    fn default_test() {
        let heap = Memory::<u32, Heap>::default();
        assert_eq!( heap.cap(), 0 );
    }

    /*
    #[test]
    fn from_test() {
        let src = [ 1, 2, 3, 4, 5 ];
        let heap = Memory::<u32, Heap>::from( &src );
        assert_eq!( heap.cap(), 5 );
        assert_eq!( heap[ 0 ], 1 );
        assert_eq!( heap[ 1 ], 2 );
        assert_eq!( heap[ 2 ], 3 );
        assert_eq!( heap[ 3 ], 4 );
        assert_eq!( heap[ 4 ], 5 );
    }
    */

    #[test]
    fn take_test() {
        let src = vec![ 1, 2, 3, 4, 5 ];
        let heap = Memory::<u32, Heap>::take( src );
        assert_eq!( heap.cap(), 5 );
        assert_eq!( heap[ 0 ], 1 );
        assert_eq!( heap[ 1 ], 2 );
        assert_eq!( heap[ 2 ], 3 );
        assert_eq!( heap[ 3 ], 4 );
        assert_eq!( heap[ 4 ], 5 );
    }

    #[test]
    fn reserve_test() {
        let mut heap = Memory::<u32, Heap>::new( 5 );
        heap.reserve( 10 );
        assert_eq!( heap.cap(), 10 );
    }

    #[test]
    fn resize_test() {
        let mut heap = Memory::<u32, Heap>::new( 5 );
        heap.resize( 10, 1 );
        assert_eq!( heap.cap(), 10 );
        assert_eq!( heap[ 0 ], 1 );
        assert_eq!( heap[ 1 ], 1 );
        assert_eq!( heap[ 2 ], 1 );
        assert_eq!( heap[ 3 ], 1 );
        assert_eq!( heap[ 4 ], 1 );
        assert_eq!( heap[ 5 ], 1 );
        assert_eq!( heap[ 6 ], 1 );
        assert_eq!( heap[ 7 ], 1 );
        assert_eq!( heap[ 8 ], 1 );
        assert_eq!( heap[ 9 ], 1 );
    }

    #[test]
    fn as_ptr_test() {
        let heap = Memory::<u32, Heap>::new( 5 );
        let ptr = heap.as_ptr();
        assert_eq!( unsafe { *ptr }, 0 );
    }

    #[test]
    fn iter_test() {
        let src = vec![ 1, 2, 3, 4, 5 ];
        let stack = Memory::<u32, Heap>::take( src.clone() );
        for ( i, value ) in stack.iter().enumerate() {
            assert_eq!( value, &src[ i ] );
        }
    }

    #[test]
    fn iter_mut_test() {
        let src = vec![ 1, 2, 3, 4, 5 ];
        let mut stack = Memory::<u32, Heap>::take( src.clone() );
        for ( i, value ) in stack.iter_mut().enumerate() {
            *value = src[ i ] + 1;
        }
        for ( i, value ) in stack.iter().enumerate() {
            assert_eq!( value, &( src[ i ] + 1 ) );
        }
    }

    #[test]
    fn into_iter_test() {
        let src = vec![ 1, 2, 3, 4, 5 ];
        let stack = Memory::<u32, Heap>::take( src.clone() );
        for ( i, value ) in stack.into_iter().enumerate() {
            assert_eq!( value, src[ i ] );
        }
    }
}

/*
#[derive( Debug, Clone )]
pub struct HeapData<T> {
    pub ptr: NonNull<T>,
    pub cap: usize,
}

impl<T> Index<usize> for HeapData<T> {
    type Output = T;

    fn index( &self, index: usize ) -> &Self::Output {
        unsafe {
            &*self.ptr.as_ptr().add( index )
        }
    }
}

impl<T> IndexMut<usize> for HeapData<T>
where
    T: 'static + Default + Copy + Clone
{
    fn index_mut( &mut self, index: usize ) -> &mut Self::Output {
        unsafe {
            &mut *self.ptr.as_ptr().add( index )
        }
    }
}

impl<T> Drop for HeapData<T> {
    fn drop( &mut self ) {
        unsafe {
            std::alloc::dealloc(
                self.ptr.as_ptr() as *mut u8,
                std::alloc::Layout::array::<T>( self.cap ).unwrap()
            );
        }
    }
}

impl<T> MemoryTraits for Memory<T, Heap>
where
    T: 'static + Default + Copy + Clone
{
    type Type = T;
    type Data = HeapData<Self::Type>;
    type New = usize;
    type From = &'static [T];
    type Take = Vec<T>;

    fn new( cap: usize ) -> Self {
        let mut this = Self { data: HeapData {
            ptr: NonNull::dangling(),
            cap: 0,
        }};
        this.resize( cap );
        this
    }

    fn from( src: &[T] ) -> Self {
        let mut this = Self { data: HeapData {
            ptr: NonNull::dangling(),
            cap: 0,
        }};
        this.resize( src.len() );
        unsafe {
            std::ptr::copy_nonoverlapping(
                src.as_ptr(),
                this.data.ptr.as_ptr(),
                src.len()
            );
        }
        this
    }

    fn take( src: Vec<T> ) -> Self {
        let cap = src.len();
        let ptr = src.as_ptr() as *mut T;
        std::mem::forget( src ); // Prevent Vec from deallocating its memory

        Self { data: HeapData {
            ptr: NonNull::new( ptr ).unwrap(),
            cap,
        }}
    }

    fn cap( &self ) -> usize {
        self.data.cap
    }

    fn resize( &mut self, cap: usize ) {
        if self.data.cap == 0 {
            self.data.ptr = NonNull::new( unsafe {
                std::alloc::alloc(
                    std::alloc::Layout::array::<T>( cap ).unwrap()
                ) as *mut T
            }).unwrap();
        } else {
            self.data.ptr = NonNull::new( unsafe {
                std::alloc::realloc(
                    self.data.ptr.as_ptr() as *mut u8,
                    std::alloc::Layout::array::<T>( self.data.cap ).unwrap(),
                    cap
                ) as *mut T
            }).unwrap();
        }
        self.data.cap = cap;
    }

    fn as_ptr( &self ) -> *const T {
        self.data.ptr.as_ptr()
    }
}

*/
