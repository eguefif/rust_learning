use crate::adaptors::map::Map;

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
}
