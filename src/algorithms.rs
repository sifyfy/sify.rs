//! sify::algorithms

use std::cmp;

pub fn binary_search<F, T>(p: F, xs: &mut [T]) -> Option<usize>
    where F: Fn(&T) -> cmp::Ordering,
          T: Ord
{
    xs.sort();
    binary_search_no_sort(p, xs)
}

pub fn binary_search_no_sort<F, T>(p: F, xs: &[T]) -> Option<usize>
    where F: Fn(&T) -> cmp::Ordering
{
    let mut left: usize = 0;
    let mut right: usize = xs.len() - 1;
    while left != right {
        let center = (right + left) / 2;
        match p(&xs[center]) {
            cmp::Ordering::Less => {
                right = center - 1;
            }
            cmp::Ordering::Equal => return Some(center),
            cmp::Ordering::Greater => {
                left = center + 1;
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn binary_search_found() {
        let target: u64 = 6;
        let mut xs: [u64; 10] = [3, 1, 9, 7, 5, 8, 4, target, 3, 7];
        let result = super::binary_search(|x| target.cmp(x), &mut xs).unwrap();
        let expected = xs.binary_search(&target).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn binary_search_notfound() {
        let target: u64 = 6;
        let mut xs: [u64; 10] = [3, 1, 9, 7, 5, 8, 4, 2, 3, 7];
        let result = super::binary_search(|x| target.cmp(x), &mut xs);
        let expected = None;
        assert_eq!(result, expected);
    }
}