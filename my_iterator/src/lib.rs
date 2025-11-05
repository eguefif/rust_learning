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
