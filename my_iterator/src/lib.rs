use std::slice::Iter;
use crate::adaptors::map::Map;
use crate::adaptors::filter::Filter;

pub mod adaptors;

pub trait MyIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn map<F, B>(self, f: F) -> Map::<Self, F> 
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B
    {
        Map::new(self, f)
    }

    fn filter<F>(self, f: F) -> Filter::<Self, F>
    where
        Self: Sized,
        F: Fn(&Self::Item) -> bool
    {
        Filter::new(self, f)
    }
}

pub struct MyVecIter<'a, T> {
    iter: Iter<'a, T>
}

impl<'a, T> MyIterator for MyVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub trait ToMyIterator {
    type Item;

    fn my_iter<'a>(&'a self) -> MyVecIter::<'a, Self::Item>;
}

impl<T> ToMyIterator for Vec<T> {
    type Item = T;

    fn my_iter<'a>(&'a self) -> MyVecIter::<'a, Self::Item> {
        MyVecIter::<Self::Item> {
            iter: self.iter()
        }
    }
}
