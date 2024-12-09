// Copyright 2024 Bewusstsein Labs

use std::{
    fmt::Debug,
    ops::{ Index, IndexMut, Deref, DerefMut }
};

use crate::memory::{ MemoryTraits, MemoryType, Memory };

#[derive( Clone, Copy, PartialEq )]
pub struct Array<T, const N: usize> ( [T; N] ) where T: 'static + Default + Copy;

impl<T, const N: usize> Array<T, N>
where
    T: 'static + Default + Copy
{
    pub fn new( array: [T; N] ) -> Self {
        Self( array )
    }
}

impl<T, const N: usize> Debug for Array<T, N>
where
    T: 'static + Default + Copy + Debug
{
    fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result {
        f.debug_list().entries( self.0.iter() ).finish()
    }
}

impl<T, const N: usize> Default for Array<T, N>
where
    T: 'static + Default + Copy
{
    fn default() -> Self {
        Self ( [ Default::default(); N ] )
    }
}

impl<T, const N: usize> Deref for Array<T, N>
where
    T: 'static + Default + Copy
{
    type Target = [T; N];

    fn deref( &self ) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for Array<T, N>
where
    T: 'static + Default + Copy
{
    fn deref_mut( &mut self ) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const N: usize> Index<usize> for Array<T, N>
where
    T: 'static + Default + Copy
{
    type Output = T;

    fn index( &self, index: usize ) -> &Self::Output {
        &self.0[ index ]
    }
}

impl<T, const N: usize> IndexMut<usize> for Array<T, N>
where
    T: 'static + Default + Copy
{
    fn index_mut( &mut self, index: usize ) -> &mut Self::Output {
        &mut self.0[ index ]
    }
}

impl<T, const N: usize> IntoIterator for Array<T, N>
where
    T: 'static + Default + Copy
{
    type Item = T;
    type IntoIter = std::array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive( Copy, Clone, Default, Debug )]
pub struct Stack<const CAP: usize>;
impl<const CAP: usize> MemoryType for Stack<CAP> {
    type Data<T> = Array<T, CAP> where T: 'static + Default + Copy + Debug;
}

impl<T, const CAP: usize> MemoryTraits for Memory<T, Stack<CAP>>
where
    T: 'static + Default + Copy + Clone + Debug
{
    type Type = T;
    type New = ();
    type Take = [T; CAP];
    type Data = Array<T, CAP>;

    fn new( _: () ) -> Self {
        Self ( Array::default() )
    }

    fn take( src: [T; CAP] ) -> Self {
        Self ( Array::new( src ) )
    }

    fn cap( &self ) -> usize {
        CAP
    }

    fn len( &self ) -> usize {
        self.0.len()
    }

    fn reserve( &mut self, _: usize ) {
        // Do nothing
    }

    fn resize( &mut self, _: usize, _: Self::Type ) {
        // Do nothing
    }

    fn push( &mut self, _: Self::Type ) {
        // Do nothing
    }

    fn pop( &mut self ) -> Option<Self::Type> {
        None
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
        let stack = Memory::<u32, Stack<10>>::new(());
        assert_eq!( stack.cap(), 10 );
    }

    #[test]
    fn default_test() {
        let stack = Memory::<u32, Stack<10>>::default();
        assert_eq!( stack.cap(), 10 );
    }

    #[test]
    fn default_test_2() {
        let stack = Memory::<f32, Stack<{10 * 2}>>::default();
        assert_eq!( stack.cap(), 20 );
    }

    /*
    #[test]
    fn from_test() {
        let src = [ 1, 2, 3, 4, 5 ];
        let heap = Memory::<u32, Stack<5>>::from( &src );
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
        let src = [ 1, 2, 3, 4, 5 ];
        let stack = Memory::<u32, Stack<5>>::take( src );
        assert_eq!( stack.cap(), 5 );
        assert_eq!( stack[ 0 ], 1 );
        assert_eq!( stack[ 1 ], 2 );
        assert_eq!( stack[ 2 ], 3 );
        assert_eq!( stack[ 3 ], 4 );
        assert_eq!( stack[ 4 ], 5 );
    }

    #[test]
    fn reserve_test() {
        let mut stack = Memory::<u32, Stack<5>>::new(());
        stack.reserve( 10 );
        assert_eq!( stack.cap(), 5 );
        assert_eq!( stack[ 0 ], 0 );
        assert_eq!( stack[ 1 ], 0 );
        assert_eq!( stack[ 2 ], 0 );
        assert_eq!( stack[ 3 ], 0 );
        assert_eq!( stack[ 4 ], 0 );
    }

    #[test]
    fn resize_test() {
        let mut stack = Memory::<u32, Stack<5>>::new(());
        stack.resize( 10, 1 );
        assert_eq!( stack.cap(), 5 );
        assert_eq!( stack[ 0 ], 0 );
        assert_eq!( stack[ 1 ], 0 );
        assert_eq!( stack[ 2 ], 0 );
        assert_eq!( stack[ 3 ], 0 );
        assert_eq!( stack[ 4 ], 0 );
    }

    #[test]
    fn as_ptr_test() {
        let stack = Memory::<u32, Stack<5>>::new(());
        let ptr = stack.as_ptr();
        assert_eq!( unsafe { *ptr }, 0 );
    }

    #[test]
    fn iter_test() {
        let src = [ 1, 2, 3, 4, 5 ];
        let stack = Memory::<u32, Stack<5>>::take( src );
        for ( i, value ) in stack.iter().enumerate() {
            assert_eq!( value, &src[ i ] );
        }
    }

    #[test]
    fn iter_mut_test() {
        let src = [ 1, 2, 3, 4, 5 ];
        let mut stack = Memory::<u32, Stack<5>>::take( src );
        for ( i, value ) in stack.iter_mut().enumerate() {
            *value = src[ i ] + 1;
        }
        for ( i, value ) in stack.iter().enumerate() {
            assert_eq!( value, &( src[ i ] + 1 ) );
        }
    }

    #[test]
    fn into_iter_test() {
        let src = [ 1, 2, 3, 4, 5 ];
        let stack = Memory::<u32, Stack<5>>::take( src );
        for ( i, value ) in stack.into_iter().enumerate() {
            assert_eq!( value, src[ i ] );
        }
    }
}
