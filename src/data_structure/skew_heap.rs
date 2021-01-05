pub mod skew_heap {
    use std::ops::*;
    use std::mem::swap;

    #[derive(Debug)]
    pub struct SkewHeap<T: Ord>(Option<Box<SkewNode<T>>>);

    #[derive(Debug)]
    struct SkewNode<T: Ord> {
        value: T,
        left: SkewHeap<T>,
        right: SkewHeap<T>,
    }

    impl<T: Ord> SkewNode<T> {
        fn new(value: T) -> Self {
            Self { value, left: SkewHeap(None), right: SkewHeap(None) }
        }
    }

    impl<T: Ord> SkewHeap<T>
    {
        pub fn new() -> Self {
            Self(None)
        }

        pub fn pop(&mut self) -> Option<T> {
            self.0.take().map(|node| {
                let v = node.value;
                *self = node.left + node.right;
                v
            })
        }

        pub fn push(&mut self, value: T) {
            *self = Self(self.0.take()) + Self(Some(Box::new(SkewNode::new(value))))
        }

        pub fn peek(&self) -> Option<&T> {
            self.0.as_ref().map(|node| &node.value)
        }
    }

    impl<T: Ord> Default for SkewHeap<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T: Ord> Add for SkewHeap<T> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Self(match (self.0, rhs.0) {
                (None, r) => r,
                (l, None) => l,
                (Some(mut l), Some(mut r)) => {
                    if l.value > r.value {
                        swap(&mut l, &mut r);
                    }
                    l.right = l.right + Self(Some(r));
                    swap(&mut l.left, &mut l.right);
                    Some(l)
                }
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::skew_heap::*;

    #[test]
    fn test_skew_heap() {
        let mut heap = SkewHeap::<i32>::new();
        assert_eq!(heap.peek(), None);
        heap.push(10);
        heap.push(1);
        heap.push(100);
        assert_eq!(heap.peek(), Some(&1));
        heap.pop();
        assert_eq!(heap.peek(), Some(&10));
    }

    #[test]
    fn test_skew_heap_2() {
        let mut heap1 = SkewHeap::<i32>::new();
        assert_eq!(heap1.peek(), None);
        heap1.push(10);
        heap1.push(1);
        heap1.push(100);
        let mut heap2 = SkewHeap::<i32>::new();
        assert_eq!(heap2.peek(), None);
        heap2.push(1000);
        heap2.push(-10);
        heap2.push(5);
        let mut heap = heap1 + heap2;
        assert_eq!(heap.peek(), Some(&-10));
        heap.pop();
        assert_eq!(heap.peek(), Some(&1));
        heap.pop();
        assert_eq!(heap.peek(), Some(&5));
    }
}