// Copyright 2024 Bewusstsein Labs

mod test;

use std::ptr::NonNull;
use std::ops::{ Index, IndexMut };

use crate::memory::Memory;

#[derive( Debug, Clone )]
pub struct Heap<T>
where
    T: Default + Copy + Clone
{
    ptr: NonNull<T>,
    cap: usize,
}

impl<T> Heap<T>
where
    T: Default + Copy + Clone
{
    pub fn new( cap: usize ) -> Self {
        let mut heap = Heap {
            ptr: NonNull::dangling(),
            cap: 0,
        };
        heap.resize( cap );
        heap
    }

    pub fn from( src: &[T] ) -> Self {
        let mut heap = Heap {
            ptr: NonNull::dangling(),
            cap: 0,
        };
        heap.resize( src.len() );
        unsafe {
            std::ptr::copy_nonoverlapping(
                src.as_ptr(),
                heap.ptr.as_ptr(),
                src.len()
            );
        }
        heap
    }

    pub fn take( src: Vec<T> ) -> Self {
        let cap = src.len();
        let ptr = src.as_ptr() as *mut T;
        std::mem::forget( src ); // Prevent Vec from deallocating its memory

        Heap {
            ptr: NonNull::new( ptr ).unwrap(),
            cap,
        }
    }

    pub fn cap( &self ) -> usize {
        self.cap
    }

    pub fn resize( &mut self, cap: usize ) {
        if self.cap == 0 {
            self.ptr = NonNull::new( unsafe {
                std::alloc::alloc(
                    std::alloc::Layout::array::<T>( cap ).unwrap()
                ) as *mut T
            }).unwrap();
        } else {
            self.ptr = NonNull::new( unsafe {
                std::alloc::realloc(
                    self.ptr.as_ptr() as *mut u8,
                    std::alloc::Layout::array::<T>( self.cap ).unwrap(),
                    cap
                ) as *mut T
            }).unwrap();
        }
        self.cap = cap;
    }

    pub fn as_ptr( &self ) -> *const T {
        self.ptr.as_ptr()
    }
}

impl<T> Index<usize> for Heap<T>
where
    T: Default + Copy + Clone
{
    type Output = T;

    fn index( &self, index: usize ) -> &Self::Output {
        unsafe {
            &*self.ptr.as_ptr().add( index )
        }
    }
}

impl<T> IndexMut<usize> for Heap<T>
where
    T: Default + Copy + Clone
{
    fn index_mut( &mut self, index: usize ) -> &mut Self::Output {
        unsafe {
            &mut *self.ptr.as_ptr().add( index )
        }
    }
}

impl<T> Drop for Heap<T>
where
    T: Default + Copy + Clone
{
    fn drop( &mut self ) {
        unsafe {
            std::alloc::dealloc(
                self.ptr.as_ptr() as *mut u8,
                std::alloc::Layout::array::<T>( self.cap ).unwrap()
            );
        }
    }
}

impl<T> Memory<T> for Heap<T>
where
    T: Default + Copy + Clone
{
    type NewType = usize;
    type TakeType = Vec<T>;

    fn new( cap: Self::NewType ) -> Self {
        Self::new( cap )
    }

    fn from( src: &[T] ) -> Self {
        Self::from( src )
    }

    fn take( src: Self::TakeType ) -> Self {
        Self::take( src )
    }

    fn cap( &self ) -> usize {
        self.cap()
    }

    fn resize( &mut self, cap: usize ) {
        self.resize( cap );
    }

    fn as_ptr( &self ) -> *const T {
        self.as_ptr()
    }
}
