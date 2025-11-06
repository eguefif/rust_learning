use crate::MyIterator;

/// This type is used to impl MyIterator to vec. It keeps an
/// index named current to keep track of where we are and
/// check if we reached the end of the vec.
///
/// The lifetime 'a tie the instance of MyIter with the slice
/// from the collections. 
pub struct MyIter<'a, T> {
    size: usize,
    current: usize,
    data: &'a [T],
}

impl<'a, T> MyIter<'a, T> {
    pub(crate) fn new(size: usize, data: &'a [T]) -> Self {
        Self {
            size,
            data,
            current: 0,
        }
    }
}

impl<'a, T> MyIterator for MyIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.size {
            return None;
        }
        let value = Some(&self.data[self.current]);
        self.current += 1;
        value
    }
}

/// This trait is impl for Vec to return a type that impl
/// MyIterator. We cannot impl our trait directly to vec
/// because we need information to iterator through the vec
/// such as an index to keep track of where we are.
pub trait ToMyIterator {
    type Item;

    // The '_ lifetime will be expand by rust
    // fn my_iter<'a>(&'a self) -> MyIter<'a, Self::Item>;
    // This ensures that the instance of MyIter won't outlive
    // the vec. The return value MyIter<'a, Self::Item> defines
    // the lifetime that will be used in the type struct MyIter<'a, T>
    fn my_iter(&self) -> MyIter<'_, Self::Item>;
}

impl<T> ToMyIterator for Vec<T> {
    type Item = T;

    fn my_iter(&self) -> MyIter<'_, Self::Item> {
        MyIter::new(self.len(), self.as_slice())
    }
}
