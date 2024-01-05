use std::{
    cmp::{max, min},
    fmt::Display,
};

use self::incrementable::Incrementable;

pub struct Interval<T: Ord + Eq + Display + Copy + Incrementable> {
    start: T,
    end: T,
}
impl<T: Ord + Eq + Display + Copy + Incrementable> Interval<T> {
    fn new(start: T, end: T) -> Interval<T> {
        if start > end {
            let mut start = start;
            let mut end = end;
            Interval {
                start: end.post_inc(),
                end: start.post_inc(),
            }
        } else {
            Interval { start, end }
        }
    }

    fn collide(&self, other: &Interval<T>) -> bool {
        other.start <= self.end && self.start <= other.end
    }

    fn intesect(&self, other: &Interval<T>) -> Option<Interval<T>> {
        if self.collide(other) {
            let overlap_start = max(self.start, other.start);
            let overlap_end = min(self.end, other.end);
            Some(Interval {
                start: overlap_start,
                end: overlap_end,
            })
        } else {
            None
        }
    }

    fn get_overlap(&self, other: &Interval<T>) -> IntervalOverlap<T> {
        if self.collide(other) {
            let overlap_start = max(self.start, other.start);
            let overlap_end = min(self.end, other.end);
            let overlap = Some(Interval {
                start: overlap_start,
                end: overlap_end,
            });

            let left = if other.start >= overlap_start {
                Some(Interval::new(other.start, overlap_start))
            } else {
                None
            };
            let right = if overlap_end >= other.end {
                Some(Interval::new(overlap_end, other.end))
            } else {
                None
            };

            IntervalOverlap {
                left,
                overlap,
                right,
            }
        } else {
            IntervalOverlap {
                left: None,
                overlap: Some(Interval::new(self.start, self.end)),
                right: None,
            }
        }
    }

    fn disjunct(&self, other: &Interval<T>) {}
}

pub struct IntervalOverlap<T: Ord + Eq + Display + Copy + Incrementable> {
    left: Option<Interval<T>>,
    overlap: Option<Interval<T>>,
    right: Option<Interval<T>>,
}

mod incrementable {
    // Copied from aSpex in
    // https://stackoverflow.com/questions/41669634/implementing-a-generic-incrementable-trait-in-rust
    pub trait Incrementable: Copy + std::ops::AddAssign<Self> {
        fn one() -> Self;

        fn post_inc(&mut self) -> Self {
            self.post_inc_by(Self::one())
        }

        fn post_inc_by(&mut self, n: Self) -> Self {
            let tmp = *self;
            *self += n;
            tmp
        }
    }

    impl Incrementable for u8 {
        fn one() -> Self {
            1
        }
    }
    impl Incrementable for u16 {
        fn one() -> Self {
            1
        }
    }
    impl Incrementable for u32 {
        fn one() -> Self {
            1
        }
    }
    impl Incrementable for u64 {
        fn one() -> Self {
            1
        }
    }
    impl Incrementable for i8 {
        fn one() -> Self {
            1
        }
    }
    impl Incrementable for i16 {
        fn one() -> Self {
            1
        }
    }
    impl Incrementable for i32 {
        fn one() -> Self {
            1
        }
    }
    impl Incrementable for i64 {
        fn one() -> Self {
            1
        }
    }
    impl Incrementable for f32 {
        fn one() -> Self {
            1.0
        }
    }
    impl Incrementable for f64 {
        fn one() -> Self {
            1.0
        }
    }
}
