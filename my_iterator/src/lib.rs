use crate::adaptors::map::Map;
use crate::adaptors::filter::Filter;
use crate::adaptors::take::Take;
use crate::adaptors::skip::Skip;
use crate::adaptors::chain::Chain;

pub mod adaptors;
pub mod my_iter;

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

    fn fold<F, A>(mut self, acc: A, f: F) -> A 
    where
        Self: Sized,
        F: Fn(A, Self::Item) -> A
    {
        let mut accum = acc;
        while let Some(value) = self.next() {
            accum = f(accum, value);
        }
        accum
    }

    fn take(self, n: usize) -> Take::<Self> 
    where
        Self: Sized,
    {
        Take::new(self, n)
    }

    fn skip(self, n: usize) -> Skip::<Self>
    where
        Self: Sized,
    {
        Skip::new(self, n)
    }

    fn chain(self, iter: Self) -> Chain::<Self> 
    where
        Self: Sized,
    {
        Chain::new(self, iter)
    }
}
