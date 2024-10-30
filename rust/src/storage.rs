pub mod owned;
pub mod view;

use crate::memory::Memory;

pub trait Storage<T, M>
where
    T: Default + Copy + Clone,
    M: Memory<T>
{
    type Type;

    fn new( mem: Self::Type ) -> Self;
    fn from( src: &[T] ) -> Self;
    fn cap( &self ) -> usize;
    fn len( &self ) -> usize;
    fn push( &mut self, value: T );
    fn pop( &mut self ) -> Option<T>;
    fn resize( &mut self, new_cap: usize );
    //fn view( &mut self, start: usize, end: usize ) -> View<T, M>;
}