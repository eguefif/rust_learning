use my_iterator::MyIterator;
use my_iterator::my_iter::ToMyIterator;

fn main() {
    println!("\nNew example with my_iter");
    let v = vec![1, 2, 3, 4, 5];
    let mut itr = v.my_iter();
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
        let v = vec![0, 1, 2, 3, 4, 5];
        let mut itr = v.my_iter().map(|v| v + 1);

        assert_eq!(Some(1), itr.next());
        assert_eq!(Some(2), itr.next());
        assert_eq!(Some(3), itr.next());
        assert_eq!(Some(4), itr.next());
        assert_eq!(Some(5), itr.next());
        assert_eq!(Some(6), itr.next());
        assert_eq!(None, itr.next());
    }

    #[test]
    fn map_should_return_lengths() {
        let v = vec!["hello".to_string(), "world".to_string()];
        let mut itr = v.my_iter().map(|v| v.len());

        assert_eq!(Some(5), itr.next());
        assert_eq!(Some(5), itr.next());
        assert_eq!(None, itr.next());
    }

    #[test]
    fn filter_should_filter_odd_number() {
        let v= vec![0, 1, 2, 3, 4, 5];
        let mut itr = v.my_iter().filter(|x| *x % 2 == 0);

        assert_eq!(Some(&0), itr.next());
        assert_eq!(Some(&2), itr.next());
        assert_eq!(Some(&4), itr.next());
        assert_eq!(None, itr.next());
    }

    #[test]
    fn fold_should_accumulate() {
        let v = vec![0, 1, 2];
        let result = v.my_iter().fold(0, |acc, v| acc + v);

        assert_eq!(3, result);
    }

    #[test]
    fn take_should_stop_iterator() {
        let v = vec![0, 1, 2, 3, 4];
        let mut itr = v.my_iter().take(2);

        assert_eq!(Some(&0), itr.next());
        assert_eq!(Some(&1), itr.next());
        assert_eq!(None, itr.next());
    }

    #[test]
    fn skip_should_skip_n_values() {
        let v = vec![0, 1, 2, 3, 4];
        let mut itr = v.my_iter().skip(2);

        assert_eq!(Some(&2), itr.next());
        assert_eq!(Some(&3), itr.next());
        assert_eq!(Some(&4), itr.next());
        assert_eq!(None, itr.next());
    }

    #[test]
    fn chain_chains_two_iterators() {
        let v1 = vec![0, 1,];
        let v2 = vec![2, 3];
        let mut itr = v1.my_iter().chain(v2.my_iter());

        assert_eq!(Some(&0), itr.next());
        assert_eq!(Some(&1), itr.next());
        assert_eq!(Some(&2), itr.next());
        assert_eq!(Some(&3), itr.next());
        assert_eq!(None, itr.next());
    }

    #[test]
    fn zip_two_iterator() {
        let v1 = vec![0, 1,];
        let v2 = vec![2, 3];
        let mut itr = v1.my_iter().zip(v2.my_iter());

        assert_eq!(Some((&0, &2)), itr.next());
        assert_eq!(Some((&1, &3)), itr.next());
        assert_eq!(None, itr.next());
    }

    #[test]
    fn enumerate_iterator() {
        let v1 = vec![0, 1, 2];
        let mut itr = v1.my_iter().enumerate();

        assert_eq!(Some((0, &0)), itr.next());
        assert_eq!(Some((1, &1)), itr.next());
        assert_eq!(Some((2, &2)), itr.next());
        assert_eq!(None, itr.next());
    }

    #[test]
    fn any_find_one_and_return_true() {
        let v1 = vec![0, 1, 2];
        let result = v1.my_iter().any(|x| *x == 1);

        assert_eq!(result, true);
    }

    #[test]
    fn any_find_nothing_and_return_false() {
        let v1 = vec![0, 1, 2];
        let result = v1.my_iter().any(|x| *x == 4);

        assert_eq!(result, false);
    }

    #[test]
    fn all_find_all_and_return_true() {
        let v1 = vec![1, 1, 1];
        let result = v1.my_iter().all(|x| *x == 1);

        assert_eq!(result, true);
    }

    #[test]
    fn all_find_one_false_and_return_false() {
        let v1 = vec![0, 1, 2];
        let result = v1.my_iter().all(|x| *x == 2);

        assert_eq!(result, false);
    }
}
