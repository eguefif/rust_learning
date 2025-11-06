use crate::MyIterator;

pub struct Zip<I> {
    iter1: I,
    iter2: I,
}

impl<I> Zip<I> {
    pub fn new(iter1: I, iter2: I) -> Self {
        Self {
            iter1, iter2
        }
    }
}

impl<I: MyIterator> MyIterator for Zip<I> {
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value1) = self.iter1.next() {
            if let Some(value2) = self.iter2.next() {
                return Some((value1, value2));
            }
        }
        None
    }
}
