use my_iterator::MyIterator;

struct Rangei32{
    start: i32,
    end: i32,
}

impl MyIterator for Rangei32 {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.start == self.end {
            return None;
        }
        let retval = self.start;

        self.start += 1;
        Some(retval)
    }
}

fn main() {
    let mut itr = Rangei32 {start:0, end: 10}.map(|v| v + 1);
    while let Some(v) = itr.next() {
        println!("{}", v);
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn it_turn_vec_into_my_iter() {
        let v = vec![1, 2];
        let mut itr = v.my_iter();
        assert_eq!(&1, itr.next().unwrap());
        assert_eq!(&2, itr.next().unwrap());
        assert_eq!(None, itr.next());
    }

    #[test]
    fn map_should_add_1() {
        let mut itr = Rangei32 {start: 0, end: 5}
                                .map(|v| v + 1);

        assert_eq!(1, itr.next().unwrap());
        assert_eq!(2, itr.next().unwrap());
        assert_eq!(3, itr.next().unwrap());
        assert_eq!(4, itr.next().unwrap());
        assert_eq!(5, itr.next().unwrap());
        assert_eq!(None, itr.next());
    }

    #[test]
    fn filter_should_filter_odd_number() {
        let mut itr = Rangei32 {start: 0, end: 5}
                                .filter(|v| *v % 2 == 0);

        assert_eq!(0, itr.next().unwrap());
        assert_eq!(2, itr.next().unwrap());
        assert_eq!(4, itr.next().unwrap());
        assert_eq!(None, itr.next());
    }

}
