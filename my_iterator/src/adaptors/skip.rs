use crate::MyIterator;

pub struct Skip<I> {
    iter: I,
}

impl<I: MyIterator> Skip<I> {
    pub fn new(mut iter: I, n: usize) -> Self {
        let mut acc = 0;
        while acc != n {
            if let None = iter.next() {
                break
            }
            acc += 1;
        }
        Skip {iter}
    }
}

impl<I: MyIterator> MyIterator for Skip<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option::<Self::Item> {
        self.iter.next()
    }
}
