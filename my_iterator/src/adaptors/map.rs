use crate::MyIterator;

pub struct Map<I, F> {
    iter: I,
    f: F
}

impl<I, F> Map<I, F> {
    pub fn new(iter: I, f: F) -> Self {
        Self {
            iter,
            f
        }
    }
}

impl<A, I: MyIterator, F> MyIterator for Map<I, F> 
    where F: FnMut(I::Item) -> A

{
    type Item = A;

    fn next(&mut self) -> Option<A> {
        let value = self.iter.next()?;

        Some((self.f)(value))
    }
}
