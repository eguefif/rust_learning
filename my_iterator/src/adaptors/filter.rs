use crate::MyIterator;

pub struct Filter<I, F> {
    iter: I,
    f: F
}

impl<I, F> Filter<I, F> {
    pub fn new(iter: I, f: F) -> Self {
        Self {iter, f}
    }
}

impl<I, F> MyIterator for Filter<I, F>
    where
        I: MyIterator,
        F: Fn(&I::Item) -> bool
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        loop {
            if let Some(value) = self.iter.next() {
                if (self.f)(&value) == true {
                    return Some(value)
                }
            } else {
                break;
            }
        }
        None
    }
}
