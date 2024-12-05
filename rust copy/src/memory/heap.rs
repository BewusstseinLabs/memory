// Copyright 2024 Bewusstsein Labs

use std::ptr::NonNull;
use std::ops::{ Index, IndexMut };

use crate::memory::{ MemoryTraits, MemoryType, Memory };

pub struct Heap();
impl MemoryType for Heap {
    type Data<T> = HeapData<T> where T: 'static + Default + Copy;
}

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
