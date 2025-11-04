pub trait MyIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
