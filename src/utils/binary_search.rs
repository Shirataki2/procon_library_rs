pub trait BinarySearch<T: Ord> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();
        while left != right {
            let mid = (left + right) / 2;
            match self[mid].cmp(&x) {
                std::cmp::Ordering::Less => {
                    left = mid + 1;
                },
                _ => {
                    right = mid;
                }
            }
        }
        left
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();
        while left != right {
            let mid = (left + right) / 2;
            match self[mid].cmp(&x) {
                std::cmp::Ordering::Greater => {
                    right = mid;
                },
                _ => {
                    left = mid + 1;
                }
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let vec = vec![1, 2, 4, 6, 7, 12, 54, 60];

        assert_eq!(vec.lower_bound(&4), 2);
        assert_eq!(vec.upper_bound(&4), 3);
    }
}