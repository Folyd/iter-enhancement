#![feature(control_flow_enum)]

use std::ops::ControlFlow;

pub trait IteratorExtend: Iterator {
    ///
    /// ```
    /// use iter_enhancement::IteratorExtend;
    /// let a = [1, 2, 3];
    /// let mut iter = a.iter();
    /// assert!(iter.at_least(1, |&x| x > 0));
    /// assert_eq!(iter.next(), Some(&2));
    ///
    /// assert!(!a.iter().at_least(1, |&x| x > 5));
    /// ```
    ///
    #[inline]
    fn at_least<F>(&mut self, threshold: usize, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
    {
        #[inline]
        fn check<T>(
            threshold: usize,
            mut f: impl FnMut(T) -> bool,
        ) -> impl FnMut(usize, T) -> ControlFlow<usize, usize> {
            move |mut n, x| {
                n += f(x) as usize;

                if n < threshold {
                    ControlFlow::Continue(n)
                } else {
                    ControlFlow::Break(n)
                }
            }
        }

        matches!(self.try_fold(0, check(threshold, f)), ControlFlow::Break(_))
    }

    #[inline]
    fn at_least_simple<F>(&mut self, threshold: usize, mut f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
    {
        let mut n = 0;
        for item in self {
            n += f(item) as usize;

            if n >= threshold {
                return true;
            }
        }

        false
    }

     /// ```
     /// // we can still use `iter`, as there are more elements.
     /// use iter_enhancement::IteratorExtend;
     /// let a = [1, 1, 3];
     /// let mut iter = a.iter();
     /// assert!(!iter.at_most(1, |&x| x == 1));
     /// assert_eq!(iter.next(), Some(&3));
     /// ```
    #[inline]
    fn at_most<F>(&mut self, threshold: usize, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
    {
        #[inline]
        fn check<T>(
            threshold: usize,
            mut f: impl FnMut(T) -> bool,
        ) -> impl FnMut(usize, T) -> ControlFlow<usize, usize> {
            move |mut n, x| {
                n += f(x) as usize;

                if n <= threshold {
                    ControlFlow::Continue(n)
                } else {
                    ControlFlow::Break(n)
                }
            }
        }

        matches!(
            self.try_fold(0, check(threshold, f)),
            ControlFlow::Continue(_)
        )
    }
}

impl<T: ?Sized> IteratorExtend for T where T: Iterator {}

#[cfg(test)]
mod tests {
    use crate::IteratorExtend;

    #[test]
    fn test_at_least() {
        let vec = vec![1, 2, 3, 4, 5];
        assert!(vec.iter().at_least(0, |i| i % 2 == 0));
        assert!(vec.iter().at_least(1, |i| i % 2 == 0));
        assert!(vec.iter().at_least(2, |i| i % 2 == 0));
        assert!(!vec.iter().at_least(3, |i| i % 2 == 0));
        assert!(vec.iter().at_least(0, |&i| i > 100));
        assert!(vec.iter().at_least(5, |&i| i < 100));

        assert!(!&vec[..0].iter().at_least(0, |_| panic!()));
    }

    #[test]
    fn test_at_least_simple() {
        let vec = vec![1, 2, 3, 4, 5];
        assert!(vec.iter().at_least_simple(0, |i| i % 2 == 0));
        assert!(vec.iter().at_least_simple(1, |i| i % 2 == 0));
        assert!(vec.iter().at_least_simple(2, |i| i % 2 == 0));
        assert!(!vec.iter().at_least_simple(3, |i| i % 2 == 0));
        assert!(vec.iter().at_least_simple(0, |&i| i > 100));
        assert!(vec.iter().at_least_simple(5, |&i| i < 100));

        assert!(!&vec[..0].iter().at_least_simple(0, |_| panic!()));
    }

    #[test]
    fn test_at_most() {
        let vec = vec![1, 2, 3, 4, 5];
        assert!(vec.iter().at_most(0, |&i| i > 100));
        assert!(vec.iter().at_most(5, |&i| i < 100));
        assert!(vec.iter().at_most(3, |i| i % 2 == 0));
        assert!(vec.iter().at_most(2, |i| i % 2 == 0));
        assert!(!vec.iter().at_most(1, |i| i % 2 == 0));
        assert!(!vec.iter().at_most(0, |i| i % 2 == 0));

        assert!(&vec[..0].iter().at_most(0, |_| panic!()));
    }
}
