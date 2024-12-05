// Copyright 2024 Bewusstsein Labs

use crate::memory::{ MemoryTraits, MemoryType, Memory };

pub struct Stack<const CAP: usize>;
impl<const CAP: usize> MemoryType for Stack<CAP> {
    type Data<T> = [T; CAP] where T: 'static + Default + Copy;
}

impl<T, const CAP: usize> MemoryTraits for Memory<T, Stack<CAP>>
where
    T: 'static + Default + Copy + Clone,
    [(); CAP]:
{
    type Type = T;
    type Data = [ T; CAP ];
    type New = ();
    type From = &'static [T];
    type Take = [T; CAP];

    fn new( _: () ) -> Self {
        Self{ data: [ Default::default(); CAP ] }
    }

    fn from( src: &[T] ) -> Self {
        let mut this = Self { data: [ Default::default(); CAP ] };
        let len = src.len().min( CAP );
        this.data[ ..len ].copy_from_slice( &src[ ..len ] );
        this
    }

    fn take( src: [T; CAP] ) -> Self {
        Self { data: src }
    }

    fn cap( &self ) -> usize {
        CAP
    }

    fn resize( &mut self, _: usize ) {
        // Do nothing
    }

    fn as_ptr( &self ) -> *const T {
        self.data.as_ptr()
    }
}
