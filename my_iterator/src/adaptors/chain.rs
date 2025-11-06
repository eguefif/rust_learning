use crate::MyIterator;

pub struct Chain<I> {
    iter1: I,
    iter2: I
}

impl<I> Chain<I> {
    pub fn new(iter1: I, iter2: I) -> Self {
        Chain {
            iter1, iter2
        }
    }
}

impl<I: MyIterator> MyIterator for Chain<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(value) = self.iter1.next() {
            return Some(value);
        }
        while let Some(value) = self.iter2.next() {
            return Some(value);
        }
        None
    }
}
