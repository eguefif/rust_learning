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
    let mut itr = Rangei32 {start:0, end: 10};
    while let Some(v) = itr.next() {
        println!("{}", v);
    }
}
